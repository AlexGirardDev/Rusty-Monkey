use std::ptr::null;
use lexer::token::Token;
use crate::object::Object;
use parser::ast::{BlockStatement, Expression, Program, Statement};
use crate::eval_error::EvalError;

pub fn eval(program: Program) -> Result<Object, EvalError> {
    for st in program.statements {
        match st {
            Statement::ExpressionStatement(exp) => return eval_expression(&exp),
            _ => {}
        }
    }
    return Err(EvalError::GenericError(String::from("rip")));
}

pub fn eval_expression(exp: &Expression) -> Result<Object, EvalError> {
    return match exp {
        Expression::IntLiteral(i) => Ok(Object::Int(*i)),
        Expression::Bool(b) => Ok(Object::Bool(*b)),
        Expression::PrefixExpression(t,right) => eval_prefix_expression(t,right),
        _ => Ok(Object::Null)
    };
}

fn eval_prefix_expression(token: &Token, exp: &Box<Expression>) -> Result<Object, EvalError> {
    return match token {
        Token::Bang => Ok(eval_bang_operator_expression(eval_expression(exp.as_ref())?)?),
        _ => Ok(Object::Null)
    };
}

fn eval_bang_operator_expression(right: Object) -> Result<Object, EvalError> {
    let result = match right {
        Object::Bool(b) => !b,
        Object::Null => true,
        _ => false
    };
    return Ok(Object::Bool(result));
}
