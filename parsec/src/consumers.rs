use super::{Context, Parser, Result};

pub fn any_char() -> AnyChar {
    AnyChar::new()
}

pub fn char(char: char) -> impl Parser<Output = char> {
    any_char()
        .filter(move |c| char.eq(c))
        .label(move || format!("expect \'{}\'", char))
}

pub fn whitespace() -> impl Parser<Output = char> {
    any_char()
        .filter(|c| c.is_whitespace())
        .label(|| "expect whitespace")
}

pub fn whitespaces() -> impl Parser<Output = String> {
    whitespace().many().map(|str| str.into_iter().collect())
}

pub fn alphabetic() -> impl Parser<Output = char> {
    any_char()
        .filter(|c| c.is_ascii_alphabetic())
        .label(|| "expect alphabetic")
}

pub fn number() -> impl Parser<Output = i64> {
    digit()
        .many()
        .map(|str| str.into_iter().collect::<String>())
        .and_then(|str| str.parse().map_err(|e| format!("{:?}", e)))
        .label(|| "expect number")
}

pub fn digit() -> impl Parser<Output = char> {
    any_char()
        .filter(|c| c.is_ascii_digit())
        .label(|| "expect digit")
}

pub fn identifier() -> impl Parser<Output = String> {
    alphabetic()
        .or(char('_'))
        .pair(
            alphabetic()
                .or(digit())
                .or(char('_'))
                .many()
                .map(|str| str.into_iter().collect::<String>()),
        )
        .map(|(first, mut str)| {
            str.insert(0, first);
            str
        })
        .label(|| "expect identifier")
}

pub fn string(str: &str) -> Str {
    Str::new(str)
}

pub fn bool() -> impl Parser<Output = bool> {
    string("true")
        .map(|_| true)
        .or(string("false").map(|_| false))
        .label(|| "expect bool")
}

pub fn eos() -> EOS {
    EOS::new()
}

// ============

/// AnyChar
#[derive(Debug, Clone)]
pub struct AnyChar;

impl AnyChar {
    fn new() -> Self {
        Self
    }
}

impl Default for AnyChar {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for AnyChar {
    type Output = char;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        context.next()
    }
}

/// Str
#[derive(Debug, Clone)]
pub struct Str<'a> {
    pub str: &'a str,
}

impl<'a> Str<'a> {
    fn new(str: &'a str) -> Self {
        Self { str }
    }
}

impl<'a> Parser for Str<'a> {
    type Output = String;

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        context
            .begin_tran(|ctx| {
                for ref char in self.str.chars() {
                    if !ctx.next()?.eq(char) {
                        return Err(ctx.throw_parser_err(""));
                    }
                }
                return Ok(self.str.to_owned());
            })
            .map_err(|_| context.throw_parser_err(format!("expect \"{}\"", self.str)))
    }
}

/// EOS
#[derive(Debug, Clone)]
pub struct EOS;

impl EOS {
    fn new() -> Self {
        Self
    }
}

impl Default for EOS {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for EOS {
    type Output = ();

    fn parse_raw(&self, context: &mut Context) -> Result<Self::Output> {
        if context.next().is_err() {
            Ok(())
        } else {
            Err(context.throw_parser_err("expect end of stream"))
        }
    }
}
