use std::collections::HashMap;

pub(super) type Env = HashMap<String, Box<Expression>>;

#[derive(Debug, Clone)]
pub(super) enum Expression {
    Atom(Atom),
    Oper {
        oper: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    If {
        pred: Box<Expression>,
        then: Box<Expression>,
        or_else: Box<Expression>,
    },
    Let {
        env: Env,
        exp: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub(super) enum Atom {
    Int(i64),
    Bool(bool),
    Var(String),
}

impl From<i64> for Atom {
    fn from(int: i64) -> Self {
        Self::Int(int)
    }
}

impl From<bool> for Atom {
    fn from(bool: bool) -> Self {
        Self::Bool(bool)
    }
}

impl From<String> for Atom {
    fn from(string: String) -> Self {
        Self::Var(string)
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Operator {
    // Arithmetic Operators
    Add,
    Subtract,
    Multiply,
    Divide,

    // Relational Operators
    Eq,
    Lt,
    Gt,
}
