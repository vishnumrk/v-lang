use crate::frontend::ast::Ast;
use crate::runtime::environment::Environment;
use crate::runtime::interpreter;
use crate::runtime::types::RuntimeVal;

pub fn evaluate_program(env: &mut Environment, statements: Vec<Ast>) -> RuntimeVal {
    let mut res = RuntimeVal::Null;
    for statement in statements {
        res = interpreter::evaluate(env, statement);
    }
    res
}

pub fn evaluate_variable_declaration(
    env: &mut Environment,
    constant: bool,
    identifier: String,
    value: Ast,
) -> RuntimeVal {
    let val = interpreter::evaluate(env, value);
    env.declare_variable(identifier.as_str(), val, constant)
}
