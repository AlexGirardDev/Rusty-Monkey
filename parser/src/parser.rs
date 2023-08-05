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
        let ident = if let Token::Ident(i) = &self.peek_token {
            i.clone()
        } else {
            return Err(self.peek_error(TokenType::Identifier));
        };

        self.next_token();

        if let Token::Assign = &self.peek_token {
            self.next_token();
        } else {
            return Err(self.peek_error(TokenType::Assign));
        }
        return Ok(Statement::Let(ident, self.parse_temp_expression()?));
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.cur_token {
            Token::Return => self.next_token(),
            _ => return Err(self.peek_error(TokenType::Return))
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
        if Parser::is_prefix_token(&self.cur_token) {
            return self.parse_prefix_expression();
        }

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
            Token::Dash | Token::Bang => true,
            _ => false
        };
    }

    fn is_infix_token(token: &Token) -> bool {
        return match token {
            Token::Dash => true,
            _ => false
        };
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        self.next_token();
        return Ok(Expression::PrefixExpression(token, Box::new(self.parse_expression(Iota::Lowest.precedence())?)));
    }

    fn parse_infix_expression(&mut self, left_side: &Expression) -> Expression {
        return Expression::Constant;
    }

    fn peek_error(&self, expected_token: TokenType) -> ParserError
    {
        return ParserError::WrongPeekToken { expected_token, actual_token: TokenType::from(&self.peek_token) };
    }
    fn cur_error(&self, expected_token: TokenType) -> ParserError
    {
        return ParserError::WrongCurrentToken { expected_token, actual_token: TokenType::from(&self.cur_token) };
    }
}
