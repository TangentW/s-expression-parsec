use parsec::{consumers::*, fail, Any, Context, Parser};

use super::expression::{Atom, Env, Expression, Operator};

pub(super) fn parse(input: &str) -> parsec::Result<Expression> {
    Context::from_str(input).parse_by(exp())
}

fn exp() -> impl Parser<Output = Expression> {
    let_exp()
        .or(if_exp())
        .or(oper_exp())
        .or(atom_exp())
        .or(fail(|| "syntax error"))
        .trim_whitespaces()
}

// Atom

fn atom_exp() -> impl Parser<Output = Expression> {
    number()
        .map(Atom::Int)
        .or(bool().map(Atom::Bool))
        .or(identifier().map(Atom::Var))
        .map(Expression::Atom)
        .trim_whitespaces()
        .or(fail(|| "atom expression syntax error"))
}

// Operator

fn oper_exp() -> impl Parser<Output = Expression> {
    oper()
        .flat_map(move |oper| {
            exp()
                .map_box()
                .pair(exp().map_box())
                .map(move |(lhs, rhs)| Expression::Oper { oper, lhs, rhs })
        })
        .in_parentheses()
        .or(fail(|| "operator expression syntax error"))
}

fn oper() -> impl Parser<Output = Operator> {
    any_char().and_then(|c| match c {
        '+' => Ok(Operator::Add),
        '-' => Ok(Operator::Subtract),
        '*' => Ok(Operator::Multiply),
        '/' => Ok(Operator::Divide),
        '=' => Ok(Operator::Eq),
        '<' => Ok(Operator::Lt),
        '>' => Ok(Operator::Gt),
        _ => Err("expect operator"),
    })
}

// If

fn if_exp() -> impl Parser<Output = Expression> {
    string("if")
        .flat_map(|_| exp().map_box().pair(exp().map_box()).pair(exp().map_box()))
        .map(|((pred, then), or_else)| Expression::If {
            pred,
            then,
            or_else,
        })
        .in_parentheses()
        .or(fail(|| "if expression syntax error"))
}

// Let

fn let_exp() -> impl Parser<Output = Expression> {
    string("let")
        .flat_map(|_| {
            binding()
                .many()
                .map(|bindings| bindings.into_iter().collect::<Env>())
                .pair(exp().map_box())
        })
        .map(|(env, exp)| Expression::Let { env, exp })
        .in_parentheses()
        .or(fail(|| "let expression syntax error"))
}

fn binding() -> impl Parser<Output = (String, Box<Expression>)> {
    identifier()
        .pair(exp().map_box())
        .in_parentheses()
        .or(fail(|| "expect binding"))
}

// Ext

trait ParserExt: Parser {
    fn map_box(self) -> Any<Box<Self::Output>>;
    fn trim_whitespaces(self) -> Any<Self::Output>;
    fn in_parentheses(self) -> Any<Self::Output>;
}

impl<T> ParserExt for T
where
    T: 'static + Parser,
{
    fn map_box(self) -> Any<Box<Self::Output>> {
        self.map(Box::new).to_any()
    }

    fn trim_whitespaces(self) -> Any<Self::Output> {
        self.between(whitespaces(), whitespaces()).to_any()
    }

    fn in_parentheses(self) -> Any<Self::Output> {
        self.trim_whitespaces()
            .between(char('('), char(')'))
            .trim_whitespaces()
            .to_any()
    }
}
