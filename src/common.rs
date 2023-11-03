use std::net::SocketAddr;

use crate::message;

pub const MESSAGE_SIZE: usize = 400;
pub const SLICE_SIZE: usize = MESSAGE_SIZE - message::HEADER_LEN;

pub fn parse_socket(s: &str) -> SocketAddr {
  if let Some(portspec) = s.strip_prefix("localhost") {
    return SocketAddr::new(
      "127.0.0.1".parse().expect("Hardcoded"),
      match portspec.is_empty() {
        true => 6677,
        false => portspec
          .strip_prefix(':')
          .expect("Expected : after localhsot")
          .parse()
          .expect("Expected port number"),
      },
    );
  }
  match s.parse() {
    Ok(ip) => SocketAddr::new(ip, 6677),
    Err(_) => s.parse().expect("Invalid address"),
  }
}
