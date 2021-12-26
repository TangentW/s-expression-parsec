use std::{borrow::Cow, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub pos: usize,
    pub msg: Cow<'static, str>,
}

impl Error {
    pub fn new(pos: usize, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            pos,
            msg: msg.into(),
        }
    }

    pub fn eos(pos: usize) -> Error {
        Error::new(pos, "end of stream")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.pos, self.msg)
    }
}
