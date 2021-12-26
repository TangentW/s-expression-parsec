use std::{borrow::Cow, marker::PhantomData};

use super::{Parser, Result};

pub fn fail<F, Output>(msg: F) -> Fail<F, Output> {
    Fail::new(msg)
}

#[derive(Debug)]
pub struct Fail<F, Output> {
    f: F,
    marker: PhantomData<Output>,
}

impl<F, Output> Fail<F, Output> {
    pub fn new(msg: F) -> Self {
        Self {
            f: msg,
            marker: PhantomData,
        }
    }
}

impl<F, Output> Clone for Fail<F, Output>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<F, Output, Msg> Parser for Fail<F, Output>
where
    Msg: Into<Cow<'static, str>>,
    F: Clone + Fn() -> Msg,
{
    type Output = Output;

    fn parse_raw(&self, context: &mut crate::Context) -> Result<Self::Output> {
        Err(context.throw_parser_err((self.f)()))
    }
}
