use crate::ast::Ast;
use crate::program::Program;
use lexer::lexer::Lexer;
use lexer::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token:  Token,
    peek_token:  Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lex: Lexer<'a>) -> Self {
        let mut parse = Parser {
            lexer: lex,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
        };

        parse.next_token();
        parse.next_token();
        return parse;
    }
    pub fn next_token(&mut self  ) {
        
        self.peek_token = self.lexer.next_token();
    }
    pub fn ParseProgram(&self) {}
}
