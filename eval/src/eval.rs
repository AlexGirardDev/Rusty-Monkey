use std::fmt::Display;

use crate::eval_error::EvalError;
use crate::object::Object;
use lexer::token::{Token, self};
use parser::ast::{Expression, Program, Statement};

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
        Expression::IntLiteral(i) => Ok((*i).into()),
        Expression::Bool(b) => Ok((*b).into()),
        Expression::PrefixExpression(t, right) => eval_prefix_expression(t, right),
        Expression::InfixExpression(t, left, right) => eval_infix_expression(t, left, right),
        _ => Ok(Object::Null),
    };
}

fn eval_infix_expression(
    token: &Token,
    left_exp: &Box<Expression>,
    right_exp: &Box<Expression>,
) -> Result<Object, EvalError> {
    let left = eval_expression(left_exp)?;
    let right = eval_expression(right_exp)?;
    eval_infix_objects(token, left, right)
}

fn eval_infix_objects(token: &Token, left: Object, right: Object) -> Result<Object, EvalError> {
    return match token {
        Token::Dash => left - right,
        Token::Plus => left + right,
        Token::ForwardSlash => left / right,
        Token::Asterisk => left * right,
        Token::NotEqual => eval_obj_comparison(left, right,ObjectComparison::NotEqual ),
        Token::Equal => eval_obj_comparison(left, right,ObjectComparison::Equal ),
        Token::LessThan => eval_obj_comparison(left, right,ObjectComparison::LessThan ),
        Token::LessThanEqual => eval_obj_comparison(left, right,ObjectComparison::LessThanEqual ),
        Token::GreaterThan => eval_obj_comparison(left, right,ObjectComparison::GreaterThan ),
        Token::GreaterThanEqual => eval_obj_comparison(left, right,ObjectComparison::GreaterThanEqual ),
        t => Err(EvalError::IncompatibleTypes(left,right,t.to_string())), 
    };
}

fn eval_obj_comparison(
    left: Object,
    right: Object,
    comp: ObjectComparison,
) -> Result<Object, EvalError> {
    let r = match comp {
        ObjectComparison::GreaterThan
        | ObjectComparison::GreaterThanEqual
        | ObjectComparison::LessThan
        | ObjectComparison::LessThanEqual => {
            let (Object::Int(l), Object::Int(r)) = (&left, &right) else { 
                    return Err(EvalError::IncompatibleTypes(left, right,comp.to_string() )) };

            let result = match comp {
                ObjectComparison::GreaterThan => l > r,
                ObjectComparison::GreaterThanEqual => l >= r,
                ObjectComparison::LessThan => l < r,
                ObjectComparison::LessThanEqual => l <= r,
                _ => false
            };
            result.into()
        },
        ObjectComparison::Equal | ObjectComparison::NotEqual => {
            let result = match (left, right) {
                (Object::Bool(l), Object::Bool(r)) => l == r,
                (Object::String(l), Object::String(r)) => l == r,
                (Object::Int(l), Object::Int(r)) => l == r,
                (Object::Null, Object::Null) => true,
                (l, r) => return Err(EvalError::IncompatibleTypes(l, r,comp.to_string()) ),
            };
              match  comp {
                ObjectComparison::NotEqual => !result,
                _ => result
            }
        }

    };
    Ok(r.into())
}

// fn eval_object_gt(left: Object, right: Object, inclusive: bool) -> Result<Object, EvalError> {
//     if let (Object::Int(l), Object::Int(r)) = (left, right) {
//         let result = (inclusive && l == r) || l > r;
//         Ok(result.into())
//     } else {
//         Err(EvalError::IncompatibleTypes(left, right, ">".to_string()))
//     }
// }
// fn eval_object_lt(left: Object, right: Object, inclusive: bool) -> Result<Object, EvalError> {
//     if let (Object::Int(l), Object::Int(r)) = (left, right) {
//         let result = (inclusive && l == r) || l < r;
//         Ok(result.into())
//     } else {
//         Err(EvalError::IncompatibleTypes(left, right, "<".to_string()))
//     }
// }
// // fn eval_object_lt(left: Object, right: Object, inclusive: bool)-> Result<Object, EvalError> {
//     let result = match (left, right) {
//         (Object::Bool(l), Object::Bool(r)) => l == r,
//         (Object::String(l), Object::String(r)) => l == r,
//         (Object::Null, Object::Null) => true,
//         (l, r) => return Err(EvalError::IncompatibleTypes(l, r, "<",to_string())),
//
// }
//
fn eval_object_equality(left: Object, right: Object, flip: bool) -> Result<Object, EvalError> {
    let result = match (left, right) {
        (Object::Bool(l), Object::Bool(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        (Object::Null, Object::Null) => true,
        (l, r) => return Err(EvalError::IncompatibleTypes(l, r, "equality".to_string())),
    };
    return Ok(if flip { !result } else { result }.into());
}

fn eval_prefix_expression(token: &Token, exp: &Box<Expression>) -> Result<Object, EvalError> {
    return match token {
        Token::Bang => Ok(eval_bang_operator_expression(eval_expression(
            exp.as_ref(),
        )?)?),
        Token::Dash => Ok(eval_minus_operator_expression(eval_expression(
            exp.as_ref(),
        )?)?),
        _ => Ok(Object::Null),
    };
}

fn eval_bang_operator_expression(right: Object) -> Result<Object, EvalError> {
    let result = match right {
        Object::Bool(b) => !b,
        Object::Null => true,
        _ => false,
    };
    return Ok(result.into());
}

fn eval_minus_operator_expression(right: Object) -> Result<Object, EvalError> {
    return match right {
        Object::Int(i) => Ok(Object::Int(-i)),
        _ => Ok(Object::Null),
    };
}

enum ObjectComparison {
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,
}

impl Display for ObjectComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ObjectComparison::GreaterThan => write!(f,">"),
            ObjectComparison::GreaterThanEqual => write!(f,">="),
            ObjectComparison::LessThan => write!(f,"<"),
            ObjectComparison::LessThanEqual => write!(f,"<="),
            ObjectComparison::Equal => write!(f,"=="),
            ObjectComparison::NotEqual => write!(f,"!=")
        }
    }
}
