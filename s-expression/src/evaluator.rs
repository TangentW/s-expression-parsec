use super::{
    expression::{Atom, Env, Expression, Operator},
    Error, Result, Val,
};

pub(super) fn eval(exp: Expression) -> Result<Val> {
    exp.eval(&Env::new())
}

impl Expression {
    fn eval(&self, env: &Env) -> Result<Val> {
        match self {
            Self::Atom(atom) => eval_atom(atom, env),
            Self::Oper { oper, lhs, rhs } => eval_oper(oper, env, lhs, rhs),
            Self::If {
                pred,
                then,
                or_else,
            } => eval_if(env, pred, then, or_else),
            Self::Let { env: self_env, exp } => eval_let(env, self_env, exp),
        }
    }
}

// Atom

fn eval_atom(atom: &Atom, env: &Env) -> Result<Val> {
    match atom {
        Atom::Int(val) => Ok(Val::Int(*val)),
        Atom::Bool(val) => Ok(Val::Bool(*val)),
        Atom::Var(id) => {
            let Some(exp) = env.get(id) else {
                return Err(Error::UnboundIdentifier);
            };
            return exp.eval(env);
        }
    }
}

// Operator

fn eval_oper(
    operator: &Operator,
    env: &Env,
    lhs: &Box<Expression>,
    rhs: &Box<Expression>,
) -> Result<Val> {
    let (Val::Int(lhs), Val::Int(rhs)) = (lhs.eval(env)?, rhs.eval(env)?) else {
        return Err(Error::TypeMismatch)
    };
    let val = match operator {
        Operator::Add => (lhs + rhs).into(),
        Operator::Subtract => (lhs - rhs).into(),
        Operator::Multiply => (lhs * rhs).into(),
        Operator::Divide => {
            if rhs == 0 {
                return Err(Error::DivisionByZero);
            }
            (lhs / rhs).into()
        }
        Operator::Eq => (lhs == rhs).into(),
        Operator::Lt => (lhs < rhs).into(),
        Operator::Gt => (lhs > rhs).into(),
    };
    Ok(val)
}

// If

fn eval_if(
    env: &Env,
    pred: &Box<Expression>,
    then: &Box<Expression>,
    or_else: &Box<Expression>,
) -> Result<Val> {
    let Val::Bool(bool) = pred.eval(env)? else {
            return Err(Error::TypeMismatch)
        };
    if bool {
        then.eval(env)
    } else {
        or_else.eval(env)
    }
}

// Let

fn eval_let(parent_env: &Env, self_env: &Env, exp: &Box<Expression>) -> Result<Val> {
    let mut env = parent_env.clone();
    env.extend(self_env.clone());
    exp.eval(&env)
}
