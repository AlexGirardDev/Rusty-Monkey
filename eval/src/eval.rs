use crate::object::Object;
use parser::ast::{Expression, Program, Statement};

pub fn eval(program: Program) -> Option<Object> {
    for st in program.statements {
        match st {
            Statement::ExpressionStatement(exp) => match exp {
                Expression::IntLiteral(i) => return Some(Object::Int(i)),
                Expression::Bool(b) => return Some(Object::Bool(b)),
                _ => {}
            },
            _ => {}
        }
    }
    return None;
}
