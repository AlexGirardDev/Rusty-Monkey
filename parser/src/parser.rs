use crate::ast::{Identifier, Program, Statement, Expression};
use lexer::lexer::Lexer;
use lexer::token::Token;
use crate::parse_error::{ParserError, TokenType};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    parse_errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: Lexer<'a>) -> Self {
        let mut parse = Parser {
            lexer: lex,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
            parse_errors: Vec::new(),
        };

        parse.next_token();
        parse.next_token();
        return parse;
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.cur_token != Token::Eof {
            if let Some(p) = self.parse_statement() {
                statements.push(p);
            }
            self.next_token();
        }

        return if statements.is_empty() { None } else { Some(Program { statements }) };
    }

    pub fn check_errors(&mut self) -> bool {
        return if self.parse_errors.is_empty() {
            false
        } else {
            self.parse_errors
                .iter()
                .for_each(|x| println!("Parse error: {}", x));
            true
        };
    }


    fn parse_statement(&mut self) -> Option<Statement> {
        return match &self.cur_token {
            Token::Let => match self.parse_let_statement() {
                Ok(s) => Some(s),
                Err(e) => {
                    &self.parse_errors.push(e);
                    None
                }
            },
            _ => None
        };
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let ident = match &self.peek_token {
            Token::Ident(i) => Identifier { value: i.clone() },
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Identifier, actual_token: TokenType::from(t) })
        };
        self.next_token();

        match &self.peek_token {
            Token::Assign => self.next_token(),
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Assign, actual_token: TokenType::from(t) })
        }

        while !matches!(&self.cur_token , Token::Semicolon) {//todo actualy parse the expression
            self.next_token();
        }

        return Ok(Statement::Let(ident, Expression::Constant));
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
        if p.check_errors(){
            panic!("ruh roh, program had errors")
        }
        let expected_statements: Vec<String> = vec![
            String::from("x"),
            String::from("y"),
            String::from("foobar"),
        ];
        assert_eq!(program.statements.len(), expected_statements.len());

        for (i, value_name) in expected_statements.iter().enumerate() {
            println!("Let statement {}", i);

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
