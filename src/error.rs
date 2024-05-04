use std::fmt::write;

#[derive(Debug)]
pub enum Gdl90Error {
    UnknownMessageType,
    ReservedContent,
    InvalidMessage,
    LogicError,
}

impl std::error::Error for Gdl90Error {

}

impl std::fmt::Display for Gdl90Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Gdl90Error::UnknownMessageType => write!(f, "Encountered unknown GDL90 message type"),
            Gdl90Error::ReservedContent => write!(f, "Encountered reserved GDL90 message content"),
            Gdl90Error::InvalidMessage => write!(f, "Encountered invalid GDL90 message content"),
            Gdl90Error::LogicError => write!(f, "Logic error encountered, please file a bug report"),
        }
    }
}

impl From<std::array::TryFromSliceError> for Gdl90Error {
    fn from(error: std::array::TryFromSliceError) -> Self {
        Gdl90Error::LogicError
    }
}