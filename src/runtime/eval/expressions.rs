 use std::collections::HashMap;

use RuntimeVal::{Null, Number};

use crate::frontend::ast::Ast;
use crate::runtime::environment::Environment;
use crate::runtime::interpreter::evaluate;
use crate::runtime::types::RuntimeVal;

pub fn evaluate_object_literal(
    env: &mut Environment,
    properties: Vec<(String, Option<Box<Ast>>)>,
) -> RuntimeVal {
    let mut objects: HashMap<String, RuntimeVal> = HashMap::new();
    for (key, value) in properties {
        let variable_name = key.clone();
        let val = evaluate(env, Ast::PropertyLiteral { key, value });
        objects.insert(variable_name, val);
    }

    RuntimeVal::Object(objects)
}

pub fn evaluate_property_literal(
    env: &mut Environment,
    key: String,
    value: Option<Box<Ast>>,
) -> RuntimeVal {
    match value {
        None => evaluate_identifier(env, key),
        Some(exp) => evaluate(env, *exp),
    }
}

pub fn evaluate_binary_expression(
    env: &mut Environment,
    left: Ast,
    right: Ast,
    operator: char,
) -> RuntimeVal {
    let left_val = evaluate(env, left);
    let right_val = evaluate(env, right);

    match (left_val, right_val, operator) {
        (Number(num1), Number(num2), '+') => Number(num1 + num2),
        (Number(num1), Number(num2), '-') => Number(num1 - num2),
        (Number(num1), Number(num2), '*') => Number(num1 * num2),
        (Number(num1), Number(num2), '/') => Number(num1 / num2),
        (Number(num1), Number(num2), '%') => Number(num1 % num2),
        _ => Null,
    }
}

pub fn evaluate_identifier(env: &mut Environment, var: String) -> RuntimeVal {
    env.lookup_variable(var.as_str()).unwrap()
}

pub fn evaluate_assignment_expression(
    env: &mut Environment,
    assignee: Ast,
    value: Ast,
) -> RuntimeVal {
    let variable: String = match assignee {
        Ast::Identifier(v) => v,
        _ => {
            panic!(
                "invalid left hand side in assignment operation found {:?}",
                assignee
            )
        }
    };
    let runtime_val = evaluate(env, value);
    env.assign_variable(variable.as_str(), runtime_val)
}

pub(crate) fn evaluate_member_expression(
    env: &mut Environment,
    object: Ast,
    property: Ast,
    computed: bool,
) -> RuntimeVal {
    if computed | matches!(object, Ast::MemberExpr { .. }) {
        let obj = evaluate(env, object);
        if let (RuntimeVal::Object(map), Ast::Identifier(var)) = (obj, property) {
            if !map.contains_key(var.as_str()) {
                panic!("{} field not found in object", var)
            }
            return map.get(var.as_str()).unwrap().clone();
        }
    } else if let (Ast::Identifier(obj), Ast::Identifier(member)) = (object, property) {
        if let Some(RuntimeVal::Object(map)) = env.lookup_variable(obj.as_str()) {
            if !map.contains_key(member.as_str()) {
                panic!("{} field not found in object {}", member, obj)
            }
            return map.get(member.as_str()).unwrap().clone();
        } else {
            panic!("{} no definition found", obj);
        }
    }
    Null
}

pub(crate) fn evaluate_call_expression(
    env: &mut Environment,
    caller: Ast,
    args: Vec<Ast>,
) -> RuntimeVal {
    Null
}
