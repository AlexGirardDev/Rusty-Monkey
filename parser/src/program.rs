
use lexer::lexer::Lexer;

use crate::{ast::BlockStatement, parse_error::ParserError, parser::Parser};

pub type Program = BlockStatement;
impl Program {
    pub fn parse(input: String) -> Self {
        let mut p = Parser::new(Lexer::new(&input));
        p.parse_program()
    }

    pub fn try_parse(input: &str) -> Result<Self, Vec<ParserError>> {
        let mut p = Parser::new(Lexer::new(input));
        let program = p.parse_program();
        let errors = p.parse_errors;
        if errors.is_empty() {
            return Ok(program);
        }
        Err(errors)
    }
}
