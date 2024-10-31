use crate::{error::Gdl90Error, message::Gdl90Message};

use std::fmt::Debug;

pub struct Gdl90Decoder {
    input_buffer: [u8; 2048],
    buffer_index: usize,
    incomplete: bool,
    control_escape: bool,
}

#[derive(Debug)]
pub enum DecodeResult {
    Incomplete,
    Complete(Vec<Result<Gdl90Message, Gdl90Error>>),
}

impl Gdl90Decoder {
    pub fn new() -> Gdl90Decoder {
        Gdl90Decoder {
            input_buffer: [0; 2048],
            buffer_index: 0,
            incomplete: false,
            control_escape: false,
        }
    }

    pub fn decode_bytes(&mut self, bytes: &[u8]) -> DecodeResult {
        let mut messages: Vec<Result<Gdl90Message, Gdl90Error>> = vec![];
        for byte in bytes {
            if self.control_escape {
                self.input_buffer[self.buffer_index] = byte ^ 0x20;
                self.buffer_index += 1;
                self.control_escape = false;
                continue;
            }
            match byte {
                0x7E => {
                    // Flag byte
                    if self.incomplete {
                        // message end received
                        messages.push(self.decode_message());
                        self.buffer_index = 0;
                        self.incomplete = false;
                    }
                    else {
                        // message start received <-- TODO: first flag byte ever received could also be end flag, need better way to synchronize
                        self.incomplete = true;
                    }
                },
                0x7D => {
                    // Control Escape
                    self.control_escape = true;
                },
                _ => {
                    self.input_buffer[self.buffer_index] = *byte;
                    self.buffer_index += 1;
                }
            }
        }
        if messages.is_empty() {
            DecodeResult::Incomplete
        } else {
            DecodeResult::Complete(messages)
        }
    }

    fn decode_message(&mut self) -> Result<Gdl90Message, Gdl90Error> {
        // TODO: check CRC
        Gdl90Message::from_bytes(&self.input_buffer[..self.buffer_index - 2])
    }
}