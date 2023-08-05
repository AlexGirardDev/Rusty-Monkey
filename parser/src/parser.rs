use crate::ast::{Identifier, Program, Statement, Expression, Iota};
use lexer::lexer::Lexer;
use lexer::token::Token;
use crate::parse_error::{ParserError, TokenType};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    pub parse_errors: Vec<ParserError>,
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

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
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
            Token::Ident(i) => i.clone(),
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Identifier, actual_token: TokenType::from(t) })
        };
        self.next_token();

        match &self.peek_token {
            Token::Assign => self.next_token(),
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Assign, actual_token: TokenType::from(t) })
        }

        return Ok(Statement::Let(ident, self.parse_temp_expression()?));
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token {
            Token::Return => self.next_token(),
            t => return Err(ParserError::WrongPeekToken { expected_token: TokenType::Return, actual_token: TokenType::from(t) })
        }

        return Ok(Statement::Return(self.parse_temp_expression()?));
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Iota::Lowest.precedence())?;

        if let Token::Semicolon = &self.peek_token {
            self.next_token();
        }
        return Ok(Statement::ExpressionStatement(expression));
    }

    fn parse_expression(&mut self, precedence: i8) -> Result<Expression, ParserError> {
        return match &self.cur_token {
            Token::Ident(i) => self.parse_identifier(i),
            Token::Int(i) => self.parse_int_literal(*i),
            _ => Err(ParserError::System)
        };
    }


    fn parse_temp_expression(&mut self) -> Result<Expression, ParserError> {
        while !matches!(&self.cur_token , Token::Semicolon) {
            self.next_token();
        }
        return Ok(Expression::Constant);
    }


    fn parse_identifier(&self, ident: &String) -> Result<Expression, ParserError> {
        return Ok(Expression::Identifier(ident.clone()));
    }

    fn parse_int_literal(&self, int: i32) -> Result<Expression, ParserError> {
        return Ok(Expression::IntLiteral(int));
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
