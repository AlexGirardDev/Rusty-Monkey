use lexer::lexer::Lexer;

use crate::{ast::BlockStatement, parser::Parser};

pub type Program = BlockStatement;
impl Program {
    pub fn new(input: String) -> Self {
        let mut p = Parser::new(Lexer::new(&input));
        p.parse_program()
    }
}
