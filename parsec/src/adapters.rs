use std::borrow::Cow;

use super::{Context, Parser, Result};

/// Map
#[derive(Debug, Clone)]
pub struct Map<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F, Output> Parser for Map<Parent, F>
where
    Parent: Parser,
    F: Clone + Fn(Parent::Output) -> Output,
{
    type Output = Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.parent.parse(context).map(&self.f)
    }
}

/// AndThen
#[derive(Debug, Clone)]
pub struct AndThen<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F, Output, Error> Parser for AndThen<Parent, F>
where
    Parent: Parser,
    Error: Into<Cow<'static, str>>,
    F: Clone + Fn(Parent::Output) -> std::result::Result<Output, Error>,
{
    type Output = Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.parent
            .parse(context)
            .and_then(|val| (self.f)(val).map_err(|e| context.throw_parser_err(e)))
    }
}

/// FlatMap
#[derive(Debug, Clone)]
pub struct FlatMap<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F, Next> Parser for FlatMap<Parent, F>
where
    Parent: Parser,
    F: Clone + Fn(Parent::Output) -> Next,
    Next: Parser,
{
    type Output = Next::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.parent
            .parse(context)
            .and_then(|x| (self.f)(x).parse(context))
    }
}

/// Pair
#[derive(Debug, Clone)]
pub struct Pair<Left, Right> {
    pub(super) left: Left,
    pub(super) right: Right,
}

impl<Left, Right> Parser for Pair<Left, Right>
where
    Left: Parser,
    Right: Parser,
{
    type Output = (Left::Output, Right::Output);

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        let left = self.left.parse(context)?;
        let right = self.right.parse(context)?;
        Ok((left, right))
    }
}

/// UseLeft
#[derive(Debug, Clone)]
pub struct UseLeft<Left, Right> {
    pub(super) left: Left,
    pub(super) right: Right,
}

impl<Left, Right> Parser for UseLeft<Left, Right>
where
    Left: Parser,
    Right: Parser,
{
    type Output = Left::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.left
            .clone()
            .pair(self.right.clone())
            .map(|(val, _)| val)
            .parse(context)
    }
}

/// UseRight
#[derive(Debug, Clone)]
pub struct UseRight<Left, Right> {
    pub(super) left: Left,
    pub(super) right: Right,
}

impl<Left, Right> Parser for UseRight<Left, Right>
where
    Left: Parser,
    Right: Parser,
{
    type Output = Right::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.left
            .clone()
            .pair(self.right.clone())
            .map(|(_, val)| val)
            .parse(context)
    }
}

/// UseRight
#[derive(Debug, Clone)]
pub struct Between<Parent, Left, Right> {
    pub(super) parent: Parent,
    pub(super) left: Left,
    pub(super) right: Right,
}

impl<Parent, Left, Right> Parser for Between<Parent, Left, Right>
where
    Parent: Parser,
    Left: Parser,
    Right: Parser,
{
    type Output = Parent::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.left
            .clone()
            .use_right(self.parent.clone())
            .use_left(self.right.clone())
            .parse(context)
    }
}

/// Or
#[derive(Debug, Clone)]
pub struct Or<Left, Right> {
    pub(super) left: Left,
    pub(super) right: Right,
}

impl<Left, Right> Parser for Or<Left, Right>
where
    Left: Parser,
    Right: Parser<Output = Left::Output>,
{
    type Output = Right::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        match self.left.parse(context) {
            Ok(val) => Ok(val),
            Err(left_err) => self.right.parse(context).map_err(|right_err| {
                if left_err.pos > right_err.pos {
                    left_err
                } else {
                    right_err
                }
            }),
        }
    }
}

/// Some
#[derive(Debug, Clone)]
pub struct Some<Parent> {
    pub(super) parent: Parent,
}

impl<Parent> Parser for Some<Parent>
where
    Parent: Parser,
{
    type Output = Vec<Parent::Output>;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        let first = self.parent.parse(context)?;
        let mut output = vec![first];
        while let Ok(val) = self.parent.parse(context) {
            output.push(val)
        }
        Ok(output)
    }
}

/// Many
#[derive(Debug, Clone)]
pub struct Many<Parent> {
    pub(super) parent: Parent,
}

impl<Parent> Parser for Many<Parent>
where
    Parent: Parser,
{
    type Output = Vec<Parent::Output>;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        if let Ok(some) = self.parent.clone().some().parse(context) {
            Ok(some)
        } else {
            Ok(Vec::new())
        }
    }
}

/// ManyTill
#[derive(Debug, Clone)]
pub struct ManyTill<Parent, End> {
    pub(super) parent: Parent,
    pub(super) end: End,
}

impl<Parent, End> Parser for ManyTill<Parent, End>
where
    Parent: Parser,
    End: Parser,
{
    type Output = Vec<Parent::Output>;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        let mut res = Vec::new();
        while self.end.parse(context).is_err() {
            let val = self.parent.parse(context)?;
            res.push(val);
        }
        Ok(res)
    }
}

/// Filter
#[derive(Debug, Clone)]
pub struct Filter<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F> Parser for Filter<Parent, F>
where
    Parent: Parser,
    F: Clone + Fn(&Parent::Output) -> bool,
{
    type Output = Parent::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.parent.parse(context).and_then(|val| {
            if (self.f)(&val) {
                Ok(val)
            } else {
                Err(context.throw_parser_err("unsatisfied"))
            }
        })
    }
}

/// Split
#[derive(Debug, Clone)]
pub struct Split<Parent, Sep> {
    pub(super) parent: Parent,
    pub(super) sep: Sep,
}

impl<Parent, Sep> Parser for Split<Parent, Sep>
where
    Parent: Parser,
    Sep: Parser,
{
    type Output = Vec<Parent::Output>;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        let first = self.parent.parse(context)?;
        let mut res = self
            .sep
            .clone()
            .use_right(self.parent.clone())
            .many()
            .parse(context)?;
        res.insert(0, first);
        Ok(res)
    }
}

/// Label
#[derive(Debug, Clone)]
pub struct Label<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F, Msg> Parser for Label<Parent, F>
where
    Parent: Parser,
    Msg: Into<Cow<'static, str>>,
    F: Clone + Fn() -> Msg,
{
    type Output = Parent::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        self.parent.parse(context).map_err(|mut e| {
            e.msg = (self.f)().into();
            e
        })
    }
}

/// Debug
#[derive(Debug, Clone)]
pub struct Debug<Parent, F> {
    pub(super) parent: Parent,
    pub(super) f: F,
}

impl<Parent, F> Parser for Debug<Parent, F>
where
    Parent: Parser<Output: std::fmt::Debug>,
    F: Clone + Fn(&Result<Parent::Output>),
{
    type Output = Parent::Output;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        let res = self.parent.parse(context);
        (self.f)(&res);
        res
    }
}
