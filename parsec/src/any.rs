use std::rc::Rc;

use super::{Context, Parser, Result};

pub struct Any<Output> {
    parse: Rc<dyn Fn(&mut Context) -> Result<Output>>,
}

impl<Output> Any<Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: 'static + Parser<Output = Output>,
    {
        Self {
            parse: Rc::new(move |ctx| parser.parse(ctx)),
        }
    }
}

impl<Output> Clone for Any<Output> {
    fn clone(&self) -> Self {
        Self {
            parse: self.parse.clone(),
        }
    }
}

impl<Output> Parser for Any<Output> {
    type Output = Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        (self.parse)(context)
    }
}
