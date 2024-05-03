use std::fmt::Debug;

use crate::error::Gdl90Error;

pub trait Message: Debug {

}

#[derive(Debug)]
pub enum Gdl90Message {
    Heartbeat(Heartbeat),
}

impl Gdl90Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Gdl90Message, Gdl90Error> {
        // TODO: error handling, return Result
        match bytes[0] {
            0x00 => {
                Ok(Gdl90Message::Heartbeat( Heartbeat::from_bytes(&bytes[1..])))
            },

            _ => {
                Err(Gdl90Error::UnknownMessageType)
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Heartbeat {
    status1: u8,
    status2: u8,
    timestamp: u16,
    uat_message_count: u16,
}

impl Heartbeat {
    pub fn new(status1: u8, status2: u8, timestamp: u16, uat_message_count: u16) -> Heartbeat {
        Heartbeat {
            status1,
            status2,
            timestamp,
            uat_message_count,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Heartbeat {
        // TODO: error handling
        if bytes.len() != 6 { panic!("Invalid size for Heartbeat message!"); }
        Heartbeat {
            status1: bytes[0],
            status2: bytes[1],
            timestamp: (bytes[3] as u16) << 8 | bytes[2] as u16,
            uat_message_count: (bytes[5] as u16) << 8 | bytes[4] as u16,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct OwnShip {
    status1: u8,
    status2: u8,
    timestamp: u16,
    uat_message_count: u16,
}