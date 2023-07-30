use crate::ast::Statement;

pub struct Program<T: Statement>{
    statements: [T]
}

