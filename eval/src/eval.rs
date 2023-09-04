use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::environment::{Env, Environment};
use crate::eval_error::EvalError;
use crate::{node::Node, object::Object};
use lexer::token::Token;
use parser::ast::{BlockStatement, Expression, Program, Statement};

pub type EvalResponse = Result<Rc<Object>, EvalError>;

pub fn eval(node: impl Into<Node>, env: &Env) -> EvalResponse {
    Ok(match &node.into() {
        Node::BlockStatement(s) => eval_block(s, env)?,
        Node::Program(p) => eval_program(p, env)?,
        Node::Statement(s) => eval_statement(s, env)?,
        Node::Expression(e) => eval_expression(e, env)?,
        Node::Object(o) => o.clone(),
    })
}

fn eval_block(block: &BlockStatement, env: &Env) -> EvalResponse {
    let mut result: Rc<Object> = Object::Null.into();
    for st in &block.statements {
        result = eval_statement(st, env)?;
        if let Object::Return(_) = result.as_ref() {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_program(block: &Program, env: &Env) -> EvalResponse {
    let mut result: Rc<Object> = Object::Null.into();
    for st in &block.statements {
        result = eval_statement(st, env)?;
        if let Object::Return(r) = result.as_ref() {
            return Ok(r.clone());
        }
    }
    Ok(result)
}

fn eval_statement(statement: &Statement, env: &Env) -> EvalResponse {
    match statement {
        Statement::ExpressionStatement(exp) => eval_expression(exp, env),
        Statement::Return(exp) => {
            let ex = eval_expression(exp, env)?;
            Ok(Object::Return(ex).into())
        }
        Statement::Let(ident, exp) => {
            let val = eval_expression(exp, env)?;
            env.clone().borrow_mut().set(ident, val);
            Ok(Object::Null.into())
        }
    }
}

pub fn eval_expression(exp: &Expression, env: &Env) -> EvalResponse {
    return match exp {
        Expression::IntLiteral(i) => Ok(Object::Int(*i).into()),
        Expression::StringLiteral(s) => Ok(Object::String(s.to_owned()).into()),
        Expression::Bool(b) => Ok(Object::Bool(*b).into()),
        Expression::PrefixExpression(t, right) => eval_prefix_expression(t, right, env),
        Expression::InfixExpression(t, left, right) => {
            eval_infix_objects(t, eval_expression(left, env)?, eval_expression(right, env)?)
        }
        Expression::IfExpression(con, if_exp, else_exp) => {
            eval_if_else_expression(con, if_exp, else_exp, env)
        }
        Expression::Identifier(ident) => match env.borrow().get(ident) {
            Some(v) => Ok(v),
            None => Err(EvalError::IdentifierNotFount(ident.to_string())),
        },
        Expression::FnExpression(idents, blk) => {
            Ok(Object::Function(idents.clone(), blk.clone(), Rc::clone(env)).into())
        }
        Expression::CallExpression(fun, values) => eval_call_expression(fun, values, env),
        Expression::Arrary(a) => eval_array_expression(a, env),
        Expression::IndexExpression(left, index_exp) => eval_index_expression(left, index_exp, env),
    };
}

fn eval_index_expression(left: &Expression, index_exp: &Expression, env: &Env) -> EvalResponse {
    let left = eval_expression(&left, env)?;
    let Object::Array(array) = left.as_ref() else {
        return Err(EvalError::IndexOperatorNotSupported(left.to_string()));
    };

    let index = eval_expression(&index_exp, env)?;
    let Object::Int(i) = *index else {
        return Err(EvalError::InvalidObjectType("Int".into(),index.to_string()));
    };
    if i >= array.len() as i64 || i < 0 {
        return Err(EvalError::IndexOutOfBounds(i, array.len() as i64));
    }
    Ok(array[usize::try_from(i).unwrap()].clone())
}

fn eval_array_expression(values: &[Expression], env: &Env) -> EvalResponse {
    Ok(Object::Array(expressions_to_objects(values, env)?).into())
}

fn eval_call_expression(fun: &Expression, values: &[Expression], env: &Env) -> EvalResponse {
    let res = eval_expression(fun, env)?;
    let (idents, blk, new_env) = match res.as_ref() {
        Object::Function(idents, blk, new_env) => (idents, blk, new_env),
        Object::Builtin(builtin) => {
            return builtin(&expressions_to_objects(values, env)?);
        }
        _ => unreachable!(),
    };
    let args: Vec<Rc<Object>> = values
        .iter()
        .map(|v| eval_expression(v, env))
        .collect::<Result<Vec<Rc<Object>>, EvalError>>()?;
    let scoped: Env = Rc::new(RefCell::new(Environment::new_closed(new_env.clone())));
    for (i, key) in idents.iter().enumerate() {
        scoped.borrow_mut().set(key, args[i].clone());
    }

    let result = eval_block(blk, &scoped)?;

    if let Object::Return(val) = result.as_ref() {
        return Ok(val.clone());
    }
    Ok(result)
}

fn eval_if_else_expression(
    cond: &Expression,
    if_exp: &BlockStatement,
    else_exp: &Option<BlockStatement>,
    env: &Env,
) -> EvalResponse {
    if is_truthy(eval_expression(cond, env)?) {
        eval_block(if_exp, env)
    } else {
        match else_exp {
            Some(exp) => eval_block(exp, env),
            None => Ok(Object::Null.into()),
        }
    }
}

fn is_truthy(obj: impl Into<Rc<Object>>) -> bool {
    match *obj.into() {
        Object::Bool(b) => b,
        Object::Null => false,
        _ => true,
    }
}

fn eval_infix_objects(token: &Token, left: Rc<Object>, right: Rc<Object>) -> EvalResponse {
    return match token {
        Token::Dash => left.as_ref() - right.as_ref(),
        Token::Plus => left.as_ref() + right.as_ref(),
        Token::ForwardSlash => left.as_ref() / right.as_ref(),
        Token::Asterisk => left.as_ref() * right.as_ref(),
        Token::NotEqual => eval_obj_comparison(left, right, ObjectComparison::NotEqual),
        Token::Equal => eval_obj_comparison(left, right, ObjectComparison::Equal),
        Token::LessThan => eval_obj_comparison(left, right, ObjectComparison::LessThan),
        Token::LessThanEqual => eval_obj_comparison(left, right, ObjectComparison::LessThanEqual),
        Token::GreaterThan => eval_obj_comparison(left, right, ObjectComparison::GreaterThan),
        Token::GreaterThanEqual => {
            eval_obj_comparison(left, right, ObjectComparison::GreaterThanEqual)
        }
        t => Err(EvalError::InvalidOperator(
            left.to_string(),
            t.to_string(),
            right.to_string(),
        )),
    };
}

fn eval_obj_comparison(
    left: Rc<Object>,
    right: Rc<Object>,
    comp: ObjectComparison,
) -> EvalResponse {
    let r = match comp {
        ObjectComparison::GreaterThan
        | ObjectComparison::GreaterThanEqual
        | ObjectComparison::LessThan
        | ObjectComparison::LessThanEqual => {
            let (Object::Int(l), Object::Int(r)) = (left.as_ref(), right.as_ref()) else {
                return Err(EvalError::InvalidOperator(left.to_string(), comp.to_string(), right.to_string()));
            };

            match comp {
                ObjectComparison::GreaterThan => l > r,
                ObjectComparison::GreaterThanEqual => l >= r,
                ObjectComparison::LessThan => l < r,
                ObjectComparison::LessThanEqual => l <= r,
                _ => false,
            }
        }
        ObjectComparison::Equal | ObjectComparison::NotEqual => {
            let result = match (left.as_ref(), right.as_ref()) {
                (Object::Bool(l), Object::Bool(r)) => l == r,
                (Object::String(l), Object::String(r)) => l == r,
                (Object::Int(l), Object::Int(r)) => l == r,
                (Object::Null, Object::Null) => true,
                (l, r) => {
                    return Err(EvalError::InvalidOperator(
                        l.to_string(),
                        comp.to_string(),
                        r.to_string(),
                    ));
                }
            };
            match comp {
                ObjectComparison::NotEqual => !result,
                _ => result,
            }
        }
    };
    Ok(Rc::new(r.into()))
}

fn eval_object_equality(left: Rc<Object>, right: Rc<Object>, flip: bool) -> EvalResponse {
    let result = match (left.as_ref(), right.as_ref()) {
        (Object::Bool(l), Object::Bool(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        (Object::Null, Object::Null) => true,
        (l, r) => {
            return Err(EvalError::InvalidOperator(
                l.to_string(),
                if flip { "!=" } else { "==" }.to_string(),
                r.to_string(),
            ));
        }
    };
    Ok(Object::Bool(if flip { !result } else { result }).into())
}

fn eval_prefix_expression(token: &Token, exp: &Expression, env: &Env) -> EvalResponse {
    let exp = eval_expression(exp, env)?;
    match token {
        Token::Bang => Ok(eval_bang_operator_expression(exp)?.into()),
        Token::Dash => Ok(eval_minus_operator_expression(exp)?.into()),
        _ => Err(EvalError::InvalidPrefix(token.to_owned())),
    }
}

fn eval_bang_operator_expression(right: Rc<Object>) -> Result<Object, EvalError> {
    let result = match right.as_ref() {
        Object::Bool(b) => !*b,
        Object::Null => true,
        _ => false,
    };
    Ok(Object::Bool(result))
}

fn eval_minus_operator_expression(right: Rc<Object>) -> Result<Object, EvalError> {
    match right.as_ref() {
        Object::Int(i) => Ok(Object::Int(-*i)),
        _ => Ok(Object::Null),
    }
}
fn expressions_to_objects(values: &[Expression], env: &Env) -> Result<Vec<Rc<Object>>, EvalError> {
    values
        .iter()
        .map(|v| eval_expression(v, env))
        .collect::<Result<Vec<Rc<Object>>, EvalError>>()
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
        match self {
            ObjectComparison::GreaterThan => write!(f, ">"),
            ObjectComparison::GreaterThanEqual => write!(f, ">="),
            ObjectComparison::LessThan => write!(f, "<"),
            ObjectComparison::LessThanEqual => write!(f, "<="),
            ObjectComparison::Equal => write!(f, "=="),
            ObjectComparison::NotEqual => write!(f, "!="),
        }
    }
}
