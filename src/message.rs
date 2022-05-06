pub struct Message {
    pub mtype: u8,
    pub slice: u32,
    pub last_slice: u32,
    pub file: u32,
    pub body: Vec<u8>
}

pub const HEADER_LEN: usize = 19;

pub fn parse_message(data: &[u8]) -> Option<Message> {
    if data.len() < 19 {
        eprintln!("Message shorter than header length {} < 19", data.len());
        return None
    }
    let checksum = data[0];
    let body_len: usize = u16::from_le_bytes(data[17..19].try_into().unwrap()).try_into().ok()?;
    let msg_len = 19 + body_len;
    if data.len() < msg_len {
        eprintln!("Message shorter than reported body length + header {}", msg_len);
        return None
    }
    if lrc(&data[4..msg_len]) != checksum { return None }
    Some(Message {
        mtype: data[4],
        slice: u32::from_le_bytes(data[5..9].try_into().unwrap()),
        last_slice: u32::from_le_bytes(data[9..13].try_into().unwrap()),
        file: u32::from_le_bytes(data[13..17].try_into().unwrap()),
        body: data[19..msg_len].to_vec()
    })
}

pub fn encode_message(data: &Message) -> Vec<u8> {
    let total_len = data.body.len() + HEADER_LEN;
    let mut buf = vec![0u8; total_len];
    let len_field:u16 = data.body.len().try_into().unwrap();
    buf[4] = data.mtype;
    buf[5..9].clone_from_slice(&data.slice.to_le_bytes());
    buf[9..13].clone_from_slice(&data.last_slice.to_le_bytes());
    buf[13..17].clone_from_slice(&data.file.to_le_bytes());
    buf[17..19].clone_from_slice(&len_field.to_le_bytes());
    buf[19..total_len].clone_from_slice(&data.body);
    let checksum = lrc(&buf[4..total_len]);
    buf[0] = checksum;
    buf
}

fn lrc(data: &[u8]) -> u8 {
    let mut result: u8 = 0;
    for b in data {
        result = result.wrapping_add(*b);
    }
    result = (result ^ 0xFF).wrapping_add(1);
    result
}

pub fn request(file: u32, slice: u32) -> Message {
    Message {
        mtype: 1,
        slice,
        last_slice: 0,
        file,
        body: vec![]
    }
}

pub fn response(req: Message, body: Vec<u8>, last_slice: u32) -> Message {
    Message {
        mtype: 2,
        last_slice,
        body,
        ..req
    }
}

pub fn matches(req: &Message, res: &Message) -> bool {
    req.mtype == 1 && res.mtype == 2
    && req.file == res.file
    && req.slice == res.slice
}