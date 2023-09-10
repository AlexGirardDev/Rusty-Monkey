use crate::ast::{BlockStatement, Expression, Identifier, Precedence, Program, Statement};
use crate::parse_error::{ParserError, TokenType};
use colored::Colorize;
use lexer::lexer::Lexer;
use lexer::token::Token;

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
        parse
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();
        while self.cur_token != Token::Eof {
            if let Some(p) = self.parse_statement() {
                statements.push(p);
            }
            self.next_token();
        }

        Program { statements }
    }
    pub fn get_program_input(&self) -> String {
        self.lexer.get_input()
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let statement = match &self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };

        match statement {
            Ok(statement) => Some(statement),
            Err(e) => match e {
                ParserError::UnexpectedStatementStart(_) => None,
                _ => {
                    self.parse_errors.push(e);
                    None
                }
            },
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let ident: String = self.expect_peek(TokenType::Identifier)?;
        self.expect_peek(TokenType::Assign)?;
        self.next_token();

        let exp = self.parse_expression(Precedence::LOWEST)?;
        if let Token::Semicolon = &self.peek_token {
            self.next_token();
        }
        Ok(Statement::Let(ident, exp))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST)?;
        if let Token::Semicolon = &self.peek_token {
            self.next_token();
        }
        Ok(Statement::Return(exp))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        if let Token::Semicolon = &self.peek_token {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(expression))
    }

    fn parse_hash(&mut self) -> Result<Expression, ParserError> {
        let mut map = Vec::<(Expression, Expression)>::new();
        if matches!(&self.peek_token ,Token::RBrace) {
            return Ok(map.into());
        }

        while !matches!(&self.cur_token, Token::RBrace) {
            let key = self.parse_expression(Precedence::LOWEST)?;
            self.expect_peek(TokenType::Colon)?;
            self.next_token();
            map.push((key, self.parse_expression(Precedence::LOWEST)?));
            self.next_token();
        }
        Ok(map.into())
    }
    fn parse_array(&mut self) -> Result<Expression, ParserError> {
        let mut values = Vec::<Expression>::new();
        if matches!(&self.peek_token, Token::RBracket) {
            self.next_token();
            return Ok(values.into());
        }

        self.next_token();

        values.push(self.parse_expression(Precedence::LOWEST)?);

        while matches!(&self.peek_token, Token::Comma) {
            self.next_token();
            self.next_token();
            values.push(self.parse_expression(Precedence::LOWEST)?);
        }
        self.expect_peek(TokenType::RBracket)?;
        Ok(values.into())
    }

    fn parse_fn_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_peek(TokenType::Lparen)?;
        let params = self.parse_fn_params()?;
        self.expect_peek(TokenType::LSquirly)?;

        Ok(Expression::FnExpression(
            params,
            self.parse_block_statement()?,
        ))
    }

    fn parse_fn_params(&mut self) -> Result<Vec<Identifier>, ParserError> {
        let mut idents = Vec::<Identifier>::new();
        if matches!(&self.peek_token, Token::RParent) {
            self.next_token();
            return Ok(idents);
        }

        self.next_token();

        if let Token::Ident(ident) = &self.cur_token {
            idents.push(ident.clone());
        } else {
            return Err(self.cur_error(TokenType::Identifier));
        }

        while matches!(&self.peek_token, Token::Comma) {
            self.next_token();
            self.next_token();

            if let Token::Ident(ident) = &self.cur_token {
                idents.push(ident.clone());
            } else {
                return Err(self.peek_error(TokenType::Identifier));
            }
        }
        self.expect_peek(TokenType::Rparen)?;
        Ok(idents)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_peek(TokenType::Lparen)?;
        self.next_token();
        let cond = self.parse_expression(Precedence::LOWEST)?;

        self.expect_peek(TokenType::Rparen)?;
        self.expect_peek(TokenType::LSquirly)?;
        let if_block = self.parse_block_statement()?;
        let else_block = if let Token::Else = &self.peek_token {
            self.next_token();
            self.expect_peek(TokenType::LSquirly)?;
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(Expression::IfExpression(
            Box::new(cond),
            if_block,
            else_block,
        ))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        self.next_token();
        let mut statements = Vec::<Statement>::new();
        while !matches!(&self.cur_token, Token::RBrace) {
            if let Some(s) = self.parse_statement() {
                statements.push(s);
            }
            self.next_token();
        }
        // self.next_token();
        Ok(BlockStatement { statements })
    }

    fn parse_expression(&mut self, precedence: i8) -> Result<Expression, ParserError> {
        if !Parser::is_prefix_token(&self.cur_token) {
            return Err(ParserError::NoValidPrefix(TokenType::from(&self.cur_token)));
        }
        let mut left_exp = self.parse_prefix_expression()?;
        while !matches!(self.peek_token, Token::Semicolon) && precedence < self.peek_precedence() {
            if !Parser::is_infix_token(&self.peek_token) {
                return Ok(left_exp);
            }
            self.next_token();

            left_exp = match &self.cur_token {
                Token::LParen => self.parse_call_expression(left_exp)?,
                Token::LBracket => self.parse_array_index_expression(left_exp)?,
                _ => self.parse_infix_expression(left_exp)?,
            };
        }

        Ok(left_exp)
    }

    fn parse_array_index_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, ParserError> {
        self.next_token();
        let val = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek(TokenType::RBracket)?;
        Ok(Expression::IndexExpression(Box::new(left), val.into()))
    }

    fn parse_call_expression(&mut self, left_side: Expression) -> Result<Expression, ParserError> {
        let params = self.parse_call_arguments()?;
        Ok(Expression::CallExpression(Box::new(left_side), params))
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut params = Vec::<Expression>::new();
        if matches!(&self.peek_token, Token::RParent) {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        params.push(self.parse_expression(Precedence::LOWEST)?);

        while matches!(&self.peek_token, Token::Comma) {
            self.next_token();
            self.next_token();
            params.push(self.parse_expression(Precedence::LOWEST)?);
        }
        self.expect_peek(TokenType::Rparen)?;
        Ok(params)
    }

    fn parse_identifier(&self, ident: &str) -> Result<Expression, ParserError> {
        Ok(Expression::Identifier(ident.to_owned()))
    }

    fn parse_int_literal(&self, int: i64) -> Result<Expression, ParserError> {
        Ok(Expression::IntLiteral(int))
    }

    fn parse_string_literal(&self, value: &str) -> Result<Expression, ParserError> {
        Ok(Expression::StringLiteral(value.to_owned()))
    }
    fn parse_bool(&self, b: bool) -> Result<Expression, ParserError> {
        Ok(Expression::Bool(b))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.cur_token {
            Token::Ident(i) => return self.parse_identifier(i),
            Token::Int(i) => return self.parse_int_literal(*i),
            Token::String(s) => return self.parse_string_literal(s),
            Token::Bool(b) => return self.parse_bool(*b),
            Token::LParen => return self.parse_grouped_expression(),
            Token::If => return self.parse_if_expression(),
            Token::Function => return self.parse_fn_expression(),
            Token::LBracket => return self.parse_array(),
            Token::LBrace => return self.parse_hash(),
            _ => {}
        };

        let token = self.cur_token.clone();
        self.next_token();
        Ok(Expression::PrefixExpression(
            token,
            Box::new(self.parse_expression(Precedence::PREFIX)?),
        ))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();
        let exp = self.parse_expression(Precedence::LOWEST)?;
        self.expect_peek(TokenType::Rparen)?;
        Ok(exp)
    }

    fn parse_infix_expression(&mut self, left_side: Expression) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        let prec = self.cur_precedence();
        self.next_token();

        let right_exp = self.parse_expression(prec)?;
        Ok(Expression::InfixExpression(
            token,
            Box::new(left_side),
            Box::new(right_exp),
        ))
    }

    fn peek_precedence(&self) -> i8 {
        Precedence::from(&self.peek_token)
    }

    fn cur_precedence(&self) -> i8 {
        Precedence::from(&self.cur_token)
    }

    fn is_prefix_token(token: &Token) -> bool {
        matches!(
            token,
            Token::Dash
                | Token::Bang
                | Token::Int(_)
                | Token::Ident(_)
                | Token::Bool(_)
                | Token::LParen
                | Token::LBrace
                | Token::LBracket
                | Token::If
                | Token::Function
                | Token::String(_)
        )
    }

    fn is_infix_token(token: &Token) -> bool {
        matches!(
            token,
            Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThanEqual
                | Token::GreaterThan
                | Token::Plus
                | Token::Dash
                | Token::ForwardSlash
                | Token::LParen
                | Token::LBracket
                | Token::Asterisk
        )
    }

    fn expect_peek<T: ExtractValue>(
        &mut self,
        expected_token: TokenType,
    ) -> Result<T, ParserError> {
        if TokenType::from(&self.peek_token) == expected_token {
            self.next_token();
            T::extract(self.cur_token.clone())
        } else {
            Err(self.peek_error(expected_token))
        }
    }

    fn peek_error(&self, expected_token: TokenType) -> ParserError {
        ParserError::WrongPeekToken {
            expected_token,
            actual_token: TokenType::from(&self.peek_token),
        }
    }

    fn cur_error(&self, expected_token: TokenType) -> ParserError {
        ParserError::WrongPeekToken {
            expected_token,
            actual_token: TokenType::from(&self.peek_token),
        }
    }

    pub fn check_and_print_errors(&self, program: &Program) {
        if !self.parse_errors.is_empty() {
            println!("{}", "Input:".yellow().underline());
            println!("{}", self.get_program_input());
            println!();
            println!("{}", "Errors:".yellow().underline());
            for parse_error in &self.parse_errors {
                println!(" - {}", parse_error.to_string().red());
            }

            println!("{}", "Statements:".yellow().underline());
            println!(" - {}", program);
            panic!("ruh roh, program had errors")
        }
    }
}

pub trait ExtractValue {
    fn extract(token: Token) -> Result<Self, ParserError>
    where
        Self: Sized;
}

impl ExtractValue for String {
    fn extract(token: Token) -> Result<Self, ParserError> {
        if let Token::Ident(value) = token {
            Ok(value)
        } else {
            Err(ParserError::WrongToken {
                expected_token: TokenType::Identifier,
                actual_token: TokenType::from(&token),
            }) // or whatever error type you want
        }
    }
}

impl ExtractValue for () {
    fn extract(_: Token) -> Result<Self, ParserError> {
        Ok(())
    }
}

impl ExtractValue for i64 {
    fn extract(token: Token) -> Result<Self, ParserError> {
        if let Token::Int(value) = token {
            Ok(value)
        } else {
            Err(ParserError::WrongToken {
                expected_token: TokenType::Int,
                actual_token: TokenType::from(&token),
            }) // or whatever error type you want
        }
    }
}
