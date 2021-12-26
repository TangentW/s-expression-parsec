use std::{borrow::Cow, str::Chars};

use super::{Error, Parser, Result};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    chars: Chars<'a>,
    pos: usize,
}

impl<'a> Context<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self { chars, pos: 0 }
    }

    pub fn from_str(str: &'a str) -> Self {
        Self::new(str.chars())
    }

    pub fn parse_by<T>(&mut self, parser: impl Parser<Output = T>) -> Result<T> {
        parser.parse(self)
    }

    pub fn next(&mut self) -> Result<char> {
        if let Some(char) = self.chars.next() {
            self.pos += 1;
            Ok(char)
        } else {
            Err(Error::eos(self.pos))
        }
    }

    pub fn throw_parser_err(&self, msg: impl Into<Cow<'static, str>>) -> Error {
        Error::new(self.pos, msg)
    }

    pub(super) fn begin_tran<T, O>(&mut self, tran: T) -> Result<O>
    where
        T: Fn(&mut Self) -> Result<O>,
    {
        let rollback_ctx = self.clone();
        let res = tran(self);
        if res.is_err() {
            *self = rollback_ctx;
        }
        res
    }
}
