use parser::ast::{Program, Statement, BlockStatement, Expression};

use crate::object::Object;


pub enum Node{
    BlockStatement(BlockStatement),
    Program(Program),
    Object(Object),
    Statement(Statement),
    Expression(Expression)

}
impl<'a> From<Object> for Node<'a> {
    fn from(v: Object) -> Self {
        Node::Object(v)
    }
}

impl<'a> From<&'a BlockStatement> for Node<'a>{
    fn from(value:&'a BlockStatement) -> Self {
        Node::BlockStatement(value)
    }
}
// impl<'a> From<BlockStatement> for Node<'a> {
//     fn from(v: &BlockStatement) -> Self {
//         Node::BlockStatement(v)
//     }
// }
//

impl<'a> From<Statement> for Node<'a> {
    fn from(v: Statement) -> Self {
        Node::Statement(v)
    }
}

impl<'a> From<Expression> for Node<'a> {
    fn from(v: Expression) -> Self {
        Node::Expression(v)
    }
}

