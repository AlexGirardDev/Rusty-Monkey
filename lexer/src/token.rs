use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Int(i64),

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
    GreaterThan,
    GreaterThanEqual,

    Plus,
    Comma,
    Semicolon,
    LParen,
    RParent,
    LBracket,
    RBracket,

    Function,
    Let,

    If,
    Else,
    Return,
    Bool(bool),
}

impl From<i64> for Token {
    fn from(value: i64) -> Self {
        Token::Int(value)
    }
}

impl From<bool> for Token {
    fn from(value: bool) -> Self {
        Token::Bool(value)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Ident(value)
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token::Ident(value.to_string())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Ident(x) => write!(f, "{}", x),
            Token::Int(x) => write!(f, "{}", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Assign => write!(f, "="),
            Token::Bang => write!(f, "!"),
            Token::Dash => write!(f, "-"),
            Token::ForwardSlash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParent => write!(f, ")"),
            Token::LBracket => write!(f, "{{"),
            Token::RBracket => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Bool(true) => write!(f, "true"),
            Token::Bool(false) => write!(f, "false"),
        };
    }
}
