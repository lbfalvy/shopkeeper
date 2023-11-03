use std::collections::HashMap;
use std::net::UdpSocket;
use std::os::unix::prelude::FileExt;
use std::path::PathBuf;
use std::{cmp, fs};

use crate::common::{parse_socket, MESSAGE_SIZE, SLICE_SIZE};
use crate::{message, util};

pub fn serve(path: PathBuf, interface: &str, log: usize) {
  let (paths, indices) = tree(&path);
  let socket = UdpSocket::bind(parse_socket(interface)).unwrap();
  let mut buf = [0; MESSAGE_SIZE];
  eprintln!("Serving {}", path.display());
  loop {
    let (_amt, src) = socket.recv_from(&mut buf).unwrap();
    if log > 0 {
      eprintln!("Received request:");
      util::print_hex(&buf, 16);
    }
    let result = message::parse_message(&buf);
    if result.is_none() {
      eprintln!("Message is malformed");
      continue;
    }
    let message = result.unwrap();
    if message.mtype != 1 {
      eprintln!("Message type not message (1)");
      continue;
    }
    let file_idx = message.file.try_into().unwrap();
    if paths.len() < file_idx {
      eprintln!("File out of range");
      continue;
    }
    let slice_index: usize = message.slice.try_into().unwrap();
    let req_path = &paths[file_idx];
    let (body, total): (Vec<u8>, usize) = if req_path.is_dir() {
      let dir_contents = ls(&req_path, &indices);
      get_str_slice(dir_contents.as_str(), slice_index)
    } else if req_path.is_file() {
      let file = fs::File::open(req_path).unwrap();
      get_file_slice(&file, slice_index)
    } else {
      eprintln!("Not a file or directory: {}", req_path.display());
      continue;
    };
    let res_pkg = message::encode_message(&message::response(
      message,
      body,
      total.try_into().unwrap(),
    ));
    if log > 0 {
      eprintln!("Responding with:");
      util::print_hex(&res_pkg, 16)
    };
    let _res = socket.send_to(&res_pkg[..], src);
  }
}

pub fn get_str_slice(data: &str, slice: usize) -> (Vec<u8>, usize) {
  let bytes = data.as_bytes();
  let start = (slice - 1) * SLICE_SIZE;
  if bytes.len() < start {
    return (vec![], last_slice(bytes.len()));
  }
  let end = cmp::min(start + SLICE_SIZE, bytes.len());
  return (bytes[start..end].to_vec(), last_slice(bytes.len()));
}

pub fn get_file_slice(file: &fs::File, slice: usize) -> (Vec<u8>, usize) {
  let len: usize = file.metadata().unwrap().len().try_into().unwrap();
  let leftover = len % SLICE_SIZE;
  let last = last_slice(len);
  if last < slice {
    return (vec![], last);
  }
  let readable = if last == slice { leftover } else { SLICE_SIZE };
  let mut result = vec![0u8; readable];
  let location: u64 = ((slice - 1) * SLICE_SIZE).try_into().unwrap();
  if 0 < result.len() {
    file.read_exact_at(&mut result[..], location).unwrap();
  }
  return (result, last);
}

fn last_slice(len: usize) -> usize {
  let leftover = len % SLICE_SIZE;
  return len / SLICE_SIZE + (if leftover == 0 { 0 } else { 1 });
}

pub fn ls(path: &PathBuf, indices: &HashMap<PathBuf, usize>) -> String {
  path
    .read_dir()
    .unwrap()
    .map(|opt_ent| match opt_ent {
      Err(_) => String::default(),
      Ok(ent) => match (
        indices.get(&ent.path()),
        ent.metadata(),
        ent.file_name().to_str(),
      ) {
        (Some(idx), Ok(meta), Some(name)) => match meta.is_dir() {
          true => format!("{}:0:{}/\n", idx, name),
          false => format!("{}:{}:{}\n", idx, meta.len(), name),
        },
        (opt_idx, res_meta, opt_name) => {
          eprintln!("Promblems on path {}", ent.path().display());
          if opt_idx.is_none() {
            eprintln!("No reverse index found")
          }
          if res_meta.is_err() {
            eprintln!("Can't retrieve metadata")
          }
          if opt_name.is_none() {
            eprintln!("Name not unicode")
          }
          String::default()
        },
      },
    })
    .collect::<String>()
}

pub fn tree(path: &PathBuf) -> (Vec<PathBuf>, HashMap<PathBuf, usize>) {
  let mut paths: Vec<PathBuf> = Vec::new();
  let mut indices: HashMap<PathBuf, usize> = HashMap::new();
  collect_subtree(&path.canonicalize().unwrap(), &mut paths, &mut indices);
  (paths, indices)
}

fn collect_subtree(
  path: &PathBuf,
  paths: &mut Vec<PathBuf>,
  indices: &mut HashMap<PathBuf, usize>,
) {
  let pos = paths.len();
  paths.push(path.clone());
  indices.insert(path.clone(), pos);
  if path.is_file() {
    return;
  }
  if path.is_dir() {
    match path.read_dir() {
      Err(_) => (),
      Ok(dir) =>
        for ent_opt in dir {
          match ent_opt {
            Err(_) => (),
            Ok(ent) => collect_subtree(&ent.path(), paths, indices),
          }
        },
    }
  } else {
    eprintln!("Unrecognized file type at {}", path.display())
  }
}
