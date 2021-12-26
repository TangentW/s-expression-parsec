#![feature(let_else)]

use s_expression;
use std::io::{self, Write};

fn main() {
    loop {
        let Some(exp) = read_exp() else { continue };
        match s_expression::run(exp) {
            Ok(val) => println!("{}", val),
            Err(err) => eprintln!("{}", err),
        }
        println!();
    }
}

fn read_exp() -> Option<String> {
    print!("> ");
    let _ = io::stdout().flush();
    let mut exp = String::new();
    match io::stdin().read_line(&mut exp) {
        Ok(_) => Some(exp),
        Err(err) => {
            eprintln!("input error: {}", err);
            return None;
        }
    }
}
