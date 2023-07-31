use std::string::ParseError;

use crate::ast::{Program, Statement};
use lexer::lexer::Lexer;
use lexer::token::Token;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lex: Lexer<'a>) -> Self {
        let mut parse = Parser {
            lexer: lex,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
        };

        parse.next_token();
        parse.next_token();
        return parse;
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&self) -> Result<Program, String> {
        return Err(String::from("ruh roh"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statments() {
        let input = "let x = 5;\
            let y = 10;\
            let foobar = 838383;
            ";

        let mut p = Parser::new(Lexer::new(&input));

        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);

        // let expected_statements: Vec<Statement> = vec![
        //     Statement::Let(Identifier("wow"), Expression::Constant),
        // ];
        let expected_statements: Vec<String> = vec![
            String::from("wow"),
        ];

        for (i, x) in expected_statements.iter().enumerate() {
            let statement = &program.statements[i];
            let test_result = test_let_statement(&statement, x);
            if test_result.is_err(){
                panic!("{}", test_result.unwrap_err());
            }
        }
    }

    fn test_let_statement(statement: &Statement, name: &str) -> Result<(), String> {
        if !matches!(statement, Statement::Let(_,_)) {
            return Err(format!("Expected let statement, got {:?}", statement));
        }
        return Ok(());
    }
}
// #[test]
// fn lexer_test() {
//     let input = "=+(){},;";
//     let mut lex = Lexer::new(&input);
//     let expected_stuff: Vec<Token> = vec![
//         Token::Assign,
//         Token::Plus,
//         Token::Lparen,
//         Token::Rparen,
//         Token::LSquirly,
//         Token::RSquirly,
//         Token::Comma,
//         Token::Semicolon,
//     ];
//     for stuff in expected_stuff {
//         let token = lex.next_token();
//         assert_eq!(token, stuff);
//     }
// }
