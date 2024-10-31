mod error;
mod message;
pub mod decoder;

#[cfg(test)]
mod tests {
    use crate::{decoder::*, message::{Gdl90Message, Heartbeat}};

    #[test]
    fn incomplete() {
        let mut decoder = Gdl90Decoder::new();
        let result = decoder.decode_bytes(&[0]);
        match result {
            DecodeResult::Incomplete => { assert!(true); }
            DecodeResult::Complete(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn heartbeat() {
        let mut decoder = Gdl90Decoder::new();
        let result = decoder.decode_bytes(&[0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E]);
        match result {
            DecodeResult::Incomplete => { assert!(false); }
            DecodeResult::Complete(messages) => {
                assert_eq!(messages.len(), 1);
                match &messages[0] {
                    Ok(message) => {
                        match message {
                            Gdl90Message::Heartbeat(heartbeat) => {
                                let expected = Heartbeat::new(0x81, 0x41, 53467, 520);
                                assert_eq!(*heartbeat, expected);
                            },
                            _ => { assert!(false); }
                        }
                    },
                    Err(_) => assert!(false)
                }
            }
        }
    }
}
