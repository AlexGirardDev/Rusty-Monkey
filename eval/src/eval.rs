use std::fmt::Display;
use std::rc::Rc;

use crate::environment::Environment;
use crate::eval_error::EvalError;
use crate::{node::Node, object::Object};
use lexer::token::Token;
use parser::ast::{BlockStatement, Expression, Program, Statement};

pub fn eval(node: Node, env: &Environment) -> Result<Rc<Object>, EvalError> {
    Ok(match node {
        Node::BlockStatement(s) => eval_block(s, env)?,
        Node::Program(p) => eval_program(p, env)?,
        Node::Statement(s) => eval_statement(s, env)?,
        Node::Expression(e) => eval_expression(e, env)?,
        Node::Object(o) => o.into(),
    })
}

pub fn eval_block(block: BlockStatement, env: &Environment) -> Result<Rc<Object>, EvalError> {
    let mut result: Rc<Object> = Object::Null.into();
    for st in block.statements {
        result = eval_statement(st, env)?;
        if let Object::Return(_) = *result {
            return Ok(result);
        }
    }
    Ok(result)
}

pub fn eval_program(block: Program, env: &Environment) -> Result<Rc<Object>, EvalError> {
    let mut result: Rc<Object> = Object::Null.into();
    for st in block.statements {
        result = eval_statement(st, env)?;
        if let Object::Return(r) = result.as_ref() {
            return Ok(r.clone());
        }
    }
    Ok(result)
}

pub fn eval_statement(statement: Statement, env: &Environment) -> Result<Rc<Object>, EvalError> {
    match statement {
        Statement::ExpressionStatement(exp) => eval_expression(exp, env),
        Statement::Return(exp) => {
            let ex = eval_expression(exp, env)?;
            Ok(Object::Return(ex).into())
        }
        Statement::Let(ident, exp) => {
            let val = eval_expression(exp, env)?;
            env.set(ident, val);
            Ok(Object::Null.into())
        }
    }
}

pub fn eval_expression(exp: Expression, env: &Environment) -> Result<Rc<Object>, EvalError> {
    return match exp {
        Expression::IntLiteral(i) => Ok(Object::Int(i).into()),
        Expression::Bool(b) => Ok(Object::Bool(b).into()),
        Expression::PrefixExpression(t, right) => eval_prefix_expression(t, *right, env),
        Expression::InfixExpression(t, left, right) => eval_infix_objects(
            t,
            eval_expression(*left, env)?,
            eval_expression(*right, env)?,
        ),
        Expression::IfExpression(con, if_exp, else_exp) => {
            eval_if_else_expression(*con, if_exp, else_exp, env)
        }
        Expression::Identifier(ident) => match env.get(&ident) {
            Some(v) => Ok(v),
            None => Err(EvalError::IdentifierNotFount(ident)),
        },
        Expression::FnExpression(idents, blk) => Ok(Object::Function(idents, blk).into()),
        Expression::CallExpression(fun, values) => {
            let mut values: Vec<Rc<Object>> = values
                .into_iter()
                .map(|v| eval_expression(v, env))
                .collect::<Result<Vec<Rc<Object>>, EvalError>>()?;
            let inner_env = Environment::new_closed(env);

            let block_statement = match *fun {
                Expression::Identifier(ident) => {
                    let fn_obj = env
                        .get(&ident)
                        .ok_or(EvalError::IdentifierNotFount(ident))?;
                    let Object::Function(idents, block_statement) = fn_obj.as_ref()
                        else {
                            return Err(EvalError::ImpossibleState(format!("expected function from env lookup but got {fn_obj}")));
                        };

                    for key in idents.iter() {
                        inner_env.set(key, values.pop().unwrap())
                    }
                    block_statement.to_owned()
                }
                Expression::FnExpression(i, block) => {
                    for key in i.iter() {
                        inner_env.set(key, values.pop().unwrap())
                    }
                    block
                }
                er => {
                    return Err(EvalError::ImpossibleState(format!(
                        "CallExpression exp property must be Ident or FnExpression but got {er}"
                    )));
                }
            };

            return eval_block(block_statement, &inner_env);
        }
        _ => Ok(Object::Null.into()),
    };
}

fn eval_if_else_expression(
    cond: Expression,
    if_exp: BlockStatement,
    else_exp: Option<BlockStatement>,
    env: &Environment,
) -> Result<Rc<Object>, EvalError> {
    if is_truthy(eval_expression(cond, env)?) {
        eval(if_exp.into(), env)
    } else {
        match else_exp {
            Some(exp) => eval(exp.into(), env),
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

fn eval_infix_objects(
    token: Token,
    left: Rc<Object>,
    right: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
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
// fn

fn eval_obj_comparison(
    left: Rc<Object>,
    right: Rc<Object>,
    comp: ObjectComparison,
) -> Result<Rc<Object>, EvalError> {
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
                // (Object::String(l), Object::String(r)) => l == r,
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

fn eval_object_equality(
    left: Rc<Object>,
    right: Rc<Object>,
    flip: bool,
) -> Result<Rc<Object>, EvalError> {
    let result = match (left.as_ref(), right.as_ref()) {
        (Object::Bool(l), Object::Bool(r)) => l == r,
        // (Object::String(l), Object::String(r)) => l == r,
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

fn eval_prefix_expression(
    token: Token,
    exp: Expression,
    env: &Environment,
) -> Result<Rc<Object>, EvalError> {
    let exp = eval_expression(exp, env)?;
    match token {
        Token::Bang => Ok(eval_bang_operator_expression(exp)?.into()),
        Token::Dash => Ok(eval_minus_operator_expression(exp)?.into()),
        _ => Err(EvalError::InvalidPrefix(token)),
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
