use crate::frontend::ast::Ast;
use crate::runtime::environment::Environment;
use crate::runtime::eval::{expressions, statements};
use crate::runtime::types::RuntimeVal;

pub fn evaluate(env: &mut Environment, ast: Ast) -> RuntimeVal {
    match ast {
        Ast::Program { statements } => statements::evaluate_program(env, statements),

        Ast::VariableDeclaration {
            constant,
            identifier,
            value,
        } => statements::evaluate_variable_declaration(env, constant, identifier, *value),

        Ast::AssignmentExpr { assignee, value } => {
            expressions::evaluate_assignment_expression(env, *assignee, *value)
        }

        Ast::BinaryExpr {
            left,
            right,
            operator,
        } => expressions::evaluate_binary_expression(env, *left, *right, operator),

        Ast::Identifier(var) => expressions::evaluate_identifier(env, var),
        Ast::NumericLiteral(num) => RuntimeVal::Number(num),
        Ast::PropertyLiteral { key, value } => {
            expressions::evaluate_property_literal(env, key, value)
        }
        Ast::ObjectLiteral { properties } => expressions::evaluate_object_literal(env, properties),
        Ast::MemberExpr {
            object,
            property,
            computed,
        } => expressions::evaluate_member_expression(env, *object, *property, computed),
        Ast::CallExpr { caller, args } => expressions::evaluate_call_expression(env, *caller, args),
    }
}
