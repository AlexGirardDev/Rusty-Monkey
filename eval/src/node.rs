use parser::ast::{BlockStatement, Expression, Program, Statement};

use crate::object::Object;

pub enum Node {
    BlockStatement(BlockStatement),
    Program(Program),
    Object(Object),
    Statement(Statement),
    Expression(Expression),
}
impl From<Object> for Node {
    fn from(v: Object) -> Self {
        Node::Object(v)
    }
}

impl From<BlockStatement> for Node {
    fn from(value: BlockStatement) -> Self {
        Node::BlockStatement(value)
    }
}

impl From<Statement> for Node {
    fn from(v: Statement) -> Self {
        Node::Statement(v)
    }
}

impl From<Expression> for Node {
    fn from(v: Expression) -> Self {
        Node::Expression(v)
    }
}
