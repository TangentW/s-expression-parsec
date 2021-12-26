use super::{Context, Parser, Result};

pub fn just<T, F>(output: F) -> Just<F>
where
    F: Fn() -> T,
{
    Just::new(output)
}

#[derive(Debug, Clone)]
pub struct Just<F> {
    f: F,
}

impl<F> Just<F> {
    pub fn new(output: F) -> Self {
        Self { f: output }
    }
}

impl<T, F> Parser for Just<F>
where
    F: Clone + Fn() -> T,
{
    type Output = T;

    fn parse_raw(&self, _context: &mut Context) -> Result<Self::Output> {
        Ok((self.f)())
    }
}
