use std::io;
use std::io::{stdout, Write};

use crate::frontend::{lexer, parser};
use crate::runtime::environment::Environment;
use crate::runtime::types::RuntimeVal;
use crate::runtime::{environment, interpreter};

fn flush_to_std_out() {
    stdout().flush().unwrap();
}

fn read_str() -> String {
    print!("> ");
    flush_to_std_out();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Didn't receive input");
    input
}

pub fn run() {
    print!("\nRepl 1.0.0\n");
    let mut environment = environment::global_env();
    loop {
        let val = execute(&mut environment, read_str());
        println!("{}", val);
    }
}

pub fn execute(environment: &mut Environment, source: String) -> RuntimeVal {
    let tokens = lexer::tokenize(source);
    // println!("{:#?}", tokens);
    let node = parser::parse(tokens);
    println!("{:#?}", node);
    interpreter::evaluate(environment, node)
}
