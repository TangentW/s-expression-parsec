use super::{adapters::*, Any, Context, Result};

pub trait Parser: Clone {
    type Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output>;

    fn parse(&self, context: &mut Context) -> Result<Self::Output> {
        context.begin_tran(|ctx| self.parse_raw(ctx))
    }

    fn to_any(self) -> Any<Self::Output>
    where
        Self: 'static,
    {
        Any::new(self)
    }

    fn map<F, O>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        Map { parent: self, f }
    }

    fn and_then<F, O>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        AndThen { parent: self, f }
    }

    fn flat_map<F, O>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O,
    {
        FlatMap { parent: self, f }
    }

    fn pair<P>(self, right: P) -> Pair<Self, P>
    where
        Self: Sized,
    {
        Pair { left: self, right }
    }

    fn use_left<P>(self, right: P) -> UseLeft<Self, P>
    where
        Self: Sized,
    {
        UseLeft { left: self, right }
    }

    fn use_right<P>(self, right: P) -> UseRight<Self, P>
    where
        Self: Sized,
    {
        UseRight { left: self, right }
    }

    fn between<L, R>(self, left: L, right: R) -> Between<Self, L, R>
    where
        Self: Sized,
    {
        Between {
            parent: self,
            left,
            right,
        }
    }

    fn or<P>(self, right: P) -> Or<Self, P>
    where
        Self: Sized,
    {
        Or { left: self, right }
    }

    fn some(self) -> Some<Self>
    where
        Self: Sized,
    {
        Some { parent: self }
    }

    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many { parent: self }
    }

    fn many_till<E>(self, end: E) -> ManyTill<Self, E> {
        ManyTill { parent: self, end }
    }

    fn filter<F>(self, predicate: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Output) -> bool,
    {
        Filter {
            parent: self,
            f: predicate,
        }
    }

    fn split<Sep>(self, sep: Sep) -> Split<Self, Sep> {
        Split { parent: self, sep }
    }

    fn label<F>(self, f: F) -> Label<Self, F> {
        Label { parent: self, f }
    }

    fn debug<F>(self, parse: F) -> Debug<Self, F>
    where
        F: Fn(&Result<Self::Output>),
    {
        Debug {
            parent: self,
            f: parse,
        }
    }
}
