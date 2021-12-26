// S-expression is a prefix notation invented for and popularized by the programming language Lisp. Your task is to evaluate some simple S-expressions.
//
// In this problem, S-expression is defined as:
// 1. An atom.
// a. A nonnegative integer. Its value is the value of the integer.
//
// b. A boolean value, true or false. Its value is the boolean value of itself.
//
// c. A variable, consisting of no more than 10 lower case letters, excluding reserved words "if", "let", "true" and "false". Its value is the value bound to the variable. If the varible is not bound yet, produce an error "Unbound Identifier". (See below for details)
//
// 2. ( f x ... )
// a. one of the following 4 forms: ( + x y ) ( - x y ) ( * x y ) ( / x y ) in which x and y are both S-expressions.
// To evaluate the S-expression, you need to first evaluate x then evaluate y.
// If the value of x or y is not an integer, produce an error "Type Mismatch".
// If both x and y are integers, its value is the their sum(x+y)/difference(x-y)/product(x*y)/quotient(x/y). The division is the same as the integer division ("/" operator) in C/C++/Java/C#, truncated division. If the value of y is zero, produce an error: "Division By Zero".
//
// b. ( if a b c ) in which a, b and c are S-expressions.
// To evaluate the S-expression, you need to evaluate a at first.
// If the value of a is not a boolean value, produce an error: "Type Mismatch".
// If the value of a is true, evaluate b and the S-expression's value is the value of b.
// If the value of a is false, evaluate c and the S-expression's value is the value of c.
// Note that b or c may not be evaluated during the calculation.
//
// c. ( let ( x a ) b ) in which x is a variable consisting of no more than 10 lower case letters, excluding reserved words "if", "let", "true" and "false", a and b are S-expressions.
// To evaluate the S-expression, you need to first evaluate a.
// Then bind the value of a to the variable x and evaluate b. Binding means if variable x appears in b, its value is the value of a. The S-expression's value is the value of b.
// Note that if x is bound to another value in b, the outer binding is ineffective. For example the value of ( let ( x 1 ) ( let ( x 2 ) x ) ) is 2.
//
// d. one of the following 3 forms: ( < x y ) ( > x y ) ( = x y ) in which x and y are S-expressions.
// To evaluate the S-expression, you need to first evaluate x then evaluate y.
// If the value of x or y is not an integer, produce an error "Type Mismatch".
// If both x and y are integers, its value is a boolean value indicating whether x < y, x > y or x = y is true.
// Given an S-expression, output its value. If an error occurs stop the evaluation and output the error.

#![feature(let_else)]

mod evaluator;
mod expression;
mod parser;
mod result;

pub use result::{Error, Result, Val};

pub fn run(input: impl AsRef<str>) -> Result<Val> {
    parser::parse(input.as_ref())
        .map_err(Error::Parser)
        .and_then(evaluator::eval)
}
