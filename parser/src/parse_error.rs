use std::fmt;

use lexer::token::Token;

#[derive(Debug)]
pub enum ParserError {
    WrongCurrentToken {
        expected_token: TokenType,
        actual_token: TokenType,
    },
    WrongPeekToken {
        expected_token: TokenType,
        actual_token: TokenType,
    },
    WrongToken {
        expected_token: TokenType,
        actual_token: TokenType,
    },
    InvalidTokenToExpression(Token),
    UnexpectedStatementStart(Token),
    NoValidPrefix(TokenType),
    ParserError(String),
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::WrongCurrentToken {
                actual_token,
                expected_token,
            } => write!(
                f,
                "Expected {} token but found an {}",
                expected_token, actual_token
            ),
            ParserError::WrongPeekToken {
                actual_token,
                expected_token,
            } => write!(
                f,
                "Peeked ahead and expected {} token but found an {}",
                expected_token, actual_token
            ),
            ParserError::WrongToken {
                actual_token,
                expected_token,
            } => write!(
                f,
                "Expected {} token but found an {}",
                expected_token, actual_token
            ),
            ParserError::UnexpectedStatementStart(token) => {
                write!(f, "{} is not a valid starting to a statement", token)
            }
            ParserError::NoValidPrefix(token) => write!(f, "{} is not a valid prefix token", token),
            ParserError::ParserError(str) => write!(f, "{}", str),
            ParserError::InvalidTokenToExpression(t) => {
                write!(f, "{} cannot be converted to an expression", t)
            }
        }
    }
}

impl From<&Token> for TokenType {
    fn from(token: &Token) -> Self {
        match token {
            Token::Ident(_) => TokenType::Identifier,
            Token::Int(_) => TokenType::Int,
            Token::Illegal => TokenType::Illegal,
            Token::Eof => TokenType::Eof,
            Token::Assign => TokenType::Assign,
            Token::Bang => TokenType::Bang,
            Token::Dash => TokenType::Dash,
            Token::ForwardSlash => TokenType::ForwardSlash,
            Token::Asterisk => TokenType::Asterisk,
            Token::Equal => TokenType::Equal,
            Token::NotEqual => TokenType::NotEqual,
            Token::LessThan => TokenType::LessThan,
            Token::LessThanEqual => TokenType::LessThanEqual,
            Token::GreaterThan => TokenType::GreaterThan,
            Token::GreaterThanEqual => TokenType::GreaterThanEqual,
            Token::Plus => TokenType::Plus,
            Token::Comma => TokenType::Comma,
            Token::Semicolon => TokenType::Semicolon,
            Token::LParen => TokenType::Lparen,
            Token::RParent => TokenType::Rparen,
            Token::LBrace => TokenType::LSquirly,
            Token::RBrace => TokenType::RSquirly,
            Token::Function => TokenType::Function,
            Token::Let => TokenType::Let,
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::Return => TokenType::Return,
            Token::Bool(_) => TokenType::Bool,
            Token::String(_) => TokenType::String,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Lparen => write!(f, "("),
            TokenType::Rparen => write!(f, ")"),
            TokenType::LSquirly => write!(f, "{{"),
            TokenType::RSquirly => write!(f, "}}"),
            TokenType::Assign => write!(f, "="),
            TokenType::Bang => write!(f, "!"),
            TokenType::Dash => write!(f, "-"),
            TokenType::ForwardSlash => write!(f, "/"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Equal => write!(f, "="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::LessThan => write!(f, "<"),
            TokenType::LessThanEqual => write!(f, "<="),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::GreaterThanEqual => write!(f, ">="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Identifier => write!(f, "ident"),
            e => write!(f, "{:?}", e),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    Int,
    Illegal,
    Eof,
    Assign,
    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThanEqual,
    GreaterThan,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
    If,
    Else,
    Return,
    Bool,
    String,
}
