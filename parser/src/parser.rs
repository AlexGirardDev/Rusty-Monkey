use crate::ast::{Identifier, Program, Statement, Expression, Precedence};
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
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if let Token::Semicolon = &self.peek_token {
            self.next_token();
        }
        return Ok(Statement::ExpressionStatement(expression));
    }

    fn parse_expression(&mut self, precedence: i8) -> Result<Expression, ParserError> {
        if !Parser::is_prefix_token(&self.cur_token) {
            return Err(ParserError::NoValidPrefix(TokenType::from(&self.cur_token)));
        }

        let mut left_exp = self.parse_prefix_expression()?;

        while !matches!(self.peek_token, Token::Semicolon)
            && precedence < self.peek_precedence()
        {
            if !Parser::is_infix_token(&self.peek_token) {
                return Ok(left_exp);
            }
            self.next_token();
            left_exp = self.parse_infix_expression(left_exp)?;
            println!("{}", left_exp);
        }

        return Ok(left_exp);
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


    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.cur_token {
            Token::Ident(i) => return self.parse_identifier(i),
            Token::Int(i) => return self.parse_int_literal(*i),
            _ => {}
        };

        let token = self.cur_token.clone();
        self.next_token();
        return Ok(Expression::PrefixExpression(token, Box::new(self.parse_expression(Precedence::PREFIX)?)));
    }

    fn parse_infix_expression(&mut self, left_side: Expression) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        let prec = self.cur_precedence();
        self.next_token();

        let right_exp = self.parse_expression(prec)?;
        return Ok(Expression::InfixExpression(token, Box::new(left_side), Box::new(right_exp)));
    }

    fn peek_precedence(&self) -> i8 {
        return Precedence::from(&self.peek_token);
    }

    fn cur_precedence(&self) -> i8 {
        return Precedence::from(&self.cur_token);
    }

    fn is_prefix_token(token: &Token) -> bool {
        return match token {
            Token::Dash |
            Token::Bang |
            Token::Int(_) |
            Token::Ident(_)
            => true,
            _ => false
        };
    }

    fn is_infix_token(token: &Token) -> bool {
        return match token {
            Token::Equal |
            Token::NotEqual |
            Token::LessThan |
            Token::GreaterThan |
            Token::Plus |
            Token::Dash |
            Token::ForwardSlash |
            Token::Asterisk => true,
            _ => false
        };
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
