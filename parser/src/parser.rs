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

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();
        while self.cur_token != Token::Eof {
            if let Some(p) = self.parse_statement() {
                statements.push(p);
            }
            self.next_token();
        }

        return Program { statements };
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
        let statement = match &self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        };
        return match statement {
            Ok(statement) => Some(statement),
            Err(e) => match e {
                ParserError::UnexpectedStatementStart(_) => None,
                _ => {
                    self.parse_errors.push(e);
                    None
                }
            }
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

        return Ok(Statement::Let(ident, self.parse_expression()?));
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token {
            Token::Return => self.next_token(),
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Return, actual_token: TokenType::from(t) })
        }

        return Ok(Statement::Return(self.parse_expression()?));
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token {
            Token::Ident(i) => {
                match &self.peek_token {
                    Token::Semicolon => {
                        let statement = Statement::ExpressionStatement(Expression::Identifier(Identifier { value: i.clone() }));
                        self.next_token();
                        self.next_token();
                        return Ok(statement);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Err(ParserError::WrongPeekToken {
            actual_token: TokenType::from(&self.peek_token),
            expected_token: TokenType::Eof,
        });
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        while !matches!(&self.cur_token , Token::Semicolon) {//todo actualy parse the expression
            self.next_token();
        }

        return Ok(Expression::Constant);
    }

    fn is_prefix_token(token: &Token) -> bool {
        return match token {
            Token::Dash => true,
            _ => false
        };
    }

    fn is_infix_token(token: &Token) -> bool {
        return match token {
            Token::Dash => true,
            _ => false
        };
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        return Expression::Constant;
    }

    fn parse_infix_expression(&mut self, left_side: &Expression) -> Expression {
        return Expression::Constant;
    }
}

#[cfg(test)]
mod tests {
    use std::os::linux::raw::stat;
    use super::*;

    #[test]
    fn test_identifier() {
        let input = "foobar;
        ";

        let mut p = Parser::new(Lexer::new(&input));

        let program = p.parse_program();
        if p.check_errors() {
            panic!("ruh roh, program had errors")
        } else {
            for s in &program.statements {
                println!("{}", s);
            }
        }

        assert_eq!(program.statements.len(), 1);

        for statement in &program.statements {
            println!("{}", statement);
        }

        let statement = &program.statements[0];
        match statement {
            Statement::ExpressionStatement(i) => {
                match i {
                    Expression::Identifier(i) => {
                        assert_eq!(i.value, "foobar");
                    }
                    s => { panic!("Expected identifier statement, got {:?}", s) }
                }
            }
            _ => { panic!("Expected expression statement, got {:?}", statement); }
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foobar = 838383;
        ";

        let mut p = Parser::new(Lexer::new(&input));

        let program = p.parse_program();
        if p.check_errors() {
            panic!("ruh roh, program had errors")
        }
        let expected_statements: Vec<String> = vec![
            String::from("x"),
            String::from("y"),
            String::from("foobar"),
        ];
        assert_eq!(program.statements.len(), expected_statements.len());
        for statement in &program.statements {
            println!("{}", statement);
        }
        for (i, value_name) in expected_statements.iter().enumerate() {
            let statement = &program.statements[i];
            let test_result = test_let_statement(&statement, value_name);
            if test_result.is_err() { panic!("{}", test_result.unwrap_err()); }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;\
        return 10;\
        return 838383;
        ";
        let expected_count = 3;

        let mut p = Parser::new(Lexer::new(&input));

        let program = p.parse_program();
        if p.check_errors() {
            panic!("ruh roh, program had errors")
        }
        assert_eq!(program.statements.len(), expected_count);
        for i in 0..expected_count {
            let statement = &program.statements[i];
            let test_result = test_return_statement(&statement);
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

    fn test_return_statement(statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Return(..) => {}
            _ => { panic!("Expected return statement, got {:?}", statement); }
        };

        return Ok(());
    }
}
