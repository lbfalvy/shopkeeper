use std::net::{UdpSocket, Ipv4Addr};
use std::time::Duration;
use std::thread;
use crate::common::{SLICE_SIZE, MESSAGE_SIZE};
use crate::message;

pub fn cat(path: &str, server: &str) -> Option<Vec<u8>> {
    let segments: Vec<&str> = path.split_inclusive("/").collect();
    let mut data = fetch(0, server)?;
    for &seg in segments.iter() {
        let (id, _, _) = String::from_utf8_lossy(&data).lines()
            .filter_map(|row| match row.split(":").collect::<Vec<_>>()[..] {
                [i, l, n] => Some((
                    i.parse::<u32>().ok()?, 
                    l.parse::<u32>().ok()?,
                    n
                )),
                _ => None
            })
            .find(|&(_, _, n)| n == seg)?;
        data = fetch(id, server)?;
    }
    Some(data)
}

pub fn fetch(id: u32, server: &str) -> Option<Vec<u8>> {
    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    sock.connect(server).ok()?;
    sock.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    let init = fetch_slice(&sock, message::request(id, 1))?;
    if init.last_slice == 0 { return Some(vec![]) }
    if init.last_slice == 1 { return Some(init.body) }
    let slice_cnt: usize = init.last_slice.try_into().ok()?;
    let mut buf = vec![0u8; slice_cnt * SLICE_SIZE];
    buf[..SLICE_SIZE].clone_from_slice(&init.body);
    for i in 2..(slice_cnt - 1) {
        let response = fetch_slice(&sock, message::request(id, i.try_into().ok()?))?;
        buf[((i-1) * SLICE_SIZE)..(i * SLICE_SIZE)].clone_from_slice(&response.body)
    }
    let last = fetch_slice(&sock, message::request(id, init.last_slice))?;
    let final_len = last.body.len();
    let final_offset = (slice_cnt - 1)*SLICE_SIZE;
    buf[final_offset..(final_offset + final_len)].clone_from_slice(&last.body);
    Some(buf[..(final_offset + final_len)].to_vec())
}

fn fetch_slice(sock: &UdpSocket, msg: message::Message) -> Option<message::Message> {
    let msg_buf = message::encode_message(&msg);
    let attempts = 3;
    let mut ans_buf = vec![0u8;MESSAGE_SIZE];
    loop {
        if sock.send(&msg_buf).is_err() { 
            thread::sleep(Duration::from_secs(3))
        } else {
            match sock.recv(&mut ans_buf) {
                Err(_) => if (--attempts) == 0 { break None },
                Ok(_) => { 
                    let answer = message::parse_message(&ans_buf)?;
                    if message::matches(&msg, &answer) {
                        break Some(answer);
                    }
                }
            }
        }
    }
}