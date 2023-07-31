use std::os::linux::raw::stat;
use std::string::ParseError;

use crate::ast::{Identifier, Program, Statement, Expression, Node};
use lexer::lexer::Lexer;
use lexer::token::Token;
use lexer::token::Token::Semicolon;

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

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.cur_token != Token::Eof {
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        return Ok(Program{statements});
    }
    fn parse_statement(&mut self) -> Result<Statement, String> {
        return match &self.cur_token {
            Token::Let => {
                self.parse_let_statement()
            }
            token => {
                Err(format!("expecting let but got {:?}", token))
            }
        };
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return Err(String::from("next token was not an identifier"))
        };
        let ident = Identifier {
            value: match &self.cur_token {
                Token::Ident(x) => x.clone(),
                _ => { return Err(String::from("next token was not an identifier")); }
            }
        };

        match self.peek_token {
            Token::Assign => self.next_token(),
            _ => return Err(String::from("Next token was not an assign"))
        }

        while !matches!(self.cur_token , Semicolon) {//todo actualy parse the expression
            self.next_token();
        }


        return Ok( Statement::Let(ident, Expression::Constant));
    }
    // fn cur_token_is(&self, token: Token) -> bool {
    //     return matches!(&self.cur_token, token);
    // }
    // fn peek_token_is(&self, token: Token) -> bool {
    //     return matches!(&self.cur_token, token);
    // }
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
        let expected_statements: Vec<String> = vec![
            String::from("x"),
            String::from("y"),
            String::from("foobar"),
        ];
        assert_eq!(program.statements.len(),expected_statements.len() );

        for (i, value_name) in expected_statements.iter().enumerate() {
            let statement = &program.statements[i];
            let test_result = test_let_statement(&statement, value_name);
            if test_result.is_err() { panic!("{}", test_result.unwrap_err()); }
        }
    }

    fn test_let_statement(statement: &Statement, name: &str) -> Result<(), String> {
        match statement {
            Statement::Let(x, ..) => {// test to make sure its a let type
                assert_eq!(x.value, name);// test to make sure name is correct
            }
            _ => { panic!("Expected let statement, got {:?}", statement); }
        };

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
