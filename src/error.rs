#[derive(Debug)]
pub enum Gdl90Error {
    UnknownMessageType
}

impl std::error::Error for Gdl90Error {

}

impl std::fmt::Display for Gdl90Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Gdl90Error::UnknownMessageType => write!(f, "Encountered unknown GDL90 message type")
        }
    }
}