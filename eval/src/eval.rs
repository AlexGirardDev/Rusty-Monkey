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
        Expression::PrefixExpression(t, right) => eval_prefix_expression(t, right),
        Expression::InfixExpression(t, left, right) => eval_infix_expression(t, left, right),
        _ => Ok(Object::Null)
    };
}

fn eval_infix_expression(token: &Token, left_exp: &Box<Expression>, right_exp: &Box<Expression>) -> Result<Object, EvalError> {
    let left = eval_expression(left_exp)?;
    let right = eval_expression(right_exp)?;
    return match left {
        Object::Int(_) => eval_int_infix_expression(token, left, right),
        _ => Ok(Object::Null)
        // Object::Bool(_) => {}
        // Object::Null => {}
    };
}

fn eval_int_infix_expression(token: &Token, left: Object, right: Object) -> Result<Object, EvalError> {
    // let left_val = if let Object::Int(i) = left { i };
    return match token {
        Token::Dash => left - right,
        Token::Plus => left + right,
        Token::ForwardSlash => left / right,
        Token::Asterisk => left * right,
        // Token::ForwardSlash => {}
        // // Token::NotEqual => {}
        // // Token::LessThan => {}
        // // Token::GreaterThan => {}
        // Token::Plus => {}
        _ => Err(EvalError::GenericError("not a valid int infix opperator".to_string()))
        // Token::Comma => {}
        // Token::Semicolon => {}
        // Token::LParen => {}
        // Token::RParent => {}
        // Token::LBracket => {}
        // Token::RBracket => {}
    };
}

fn eval_prefix_expression(token: &Token, exp: &Box<Expression>) -> Result<Object, EvalError> {
    return match token {
        Token::Bang => Ok(eval_bang_operator_expression(eval_expression(exp.as_ref())?)?),
        Token::Dash => Ok(eval_minus_operator_expression(eval_expression(exp.as_ref())?)?),
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

fn eval_minus_operator_expression(right: Object) -> Result<Object, EvalError> {
    return match right {
        Object::Int(i) => Ok(Object::Int(-i)),
        _ => Ok(Object::Null)
    };
}
