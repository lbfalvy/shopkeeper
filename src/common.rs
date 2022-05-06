use crate::message;

pub const MESSAGE_SIZE: usize = 400;
pub const SLICE_SIZE: usize = MESSAGE_SIZE - message::HEADER_LEN;