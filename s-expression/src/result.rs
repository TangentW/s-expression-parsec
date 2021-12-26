use std::{borrow::Cow, fmt::Display};

use parsec;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Val {
    Int(i64),
    Bool(bool),
}

impl From<i64> for Val {
    fn from(int: i64) -> Self {
        Self::Int(int)
    }
}

impl From<bool> for Val {
    fn from(bool: bool) -> Self {
        Self::Bool(bool)
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => val.fmt(f),
            Self::Bool(val) => val.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Parser(parsec::Error),
    TypeMismatch,
    UnboundIdentifier,
    DivisionByZero,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc: Cow<'static, str> = match self {
            Self::Parser(err) => format!("{}", err).into(),
            Self::TypeMismatch => "Type Mismatch".into(),
            Self::UnboundIdentifier => "Unbound Identifier".into(),
            Self::DivisionByZero => "Division By Zero".into(),
        };
        f.write_str(&desc)
    }
}
