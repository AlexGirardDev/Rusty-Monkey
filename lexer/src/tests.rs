#![allow(dead_code)]

use crate::{lexer::Lexer, token::Token};

#[test]
fn lexer_test() {
    let input = "=+(){},;";
    let mut lex = Lexer::new(input);
    let expected_stuff: Vec<Token> = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParent,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
    ];
    for stuff in expected_stuff {
        let token = lex.next_token();
        assert_eq!(token, stuff);
    }
}

#[test]
fn test_next_token() {
    let input = r#"
    let five = 5;
            let ten = 10;
            let add = fn (x, y)
            {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 >= 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
"foobar";
"";
[1,2];
{"foo":"bar"};
"#;
    let mut lex = Lexer::new(input);

    let expected_stuff: Vec<Token> = vec![
        Token::Let,
        Token::new("five"),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::new("ten"),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::new("add"),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::new("x"),
        Token::Comma,
        Token::new("y"),
        Token::RParent,
        Token::LBrace,
        Token::new("x"),
        Token::Plus,
        Token::new("y"),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::new("result"),
        Token::Assign,
        Token::new("add"),
        Token::LParen,
        Token::new("five"),
        Token::Comma,
        Token::new("ten"),
        Token::RParent,
        Token::Semicolon,
        Token::Bang,
        Token::Dash,
        Token::ForwardSlash,
        Token::Asterisk,
        Token::Int(5),
        Token::Semicolon,
        Token::Int(5),
        Token::LessThan,
        Token::Int(10),
        Token::GreaterThanEqual,
        Token::Int(5),
        Token::Semicolon,
        Token::If,
        Token::LParen,
        Token::Int(5),
        Token::LessThan,
        Token::Int(10),
        Token::RParent,
        Token::LBrace,
        Token::Return,
        Token::Bool(true),
        Token::Semicolon,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::Return,
        Token::Bool(false),
        Token::Semicolon,
        Token::RBrace,
        Token::Int(10),
        Token::Equal,
        Token::Int(10),
        Token::Semicolon,
        Token::Int(10),
        Token::NotEqual,
        Token::Int(9),
        Token::Semicolon,
        Token::String("foobar".to_owned()),
        Token::Semicolon,
        Token::String("".to_owned()),
        Token::Semicolon,
        Token::LBracket,
        Token::new(1),
        Token::Comma,
        Token::new(2),
        Token::RBracket,
        Token::Semicolon,
        Token::LBrace,
        Token::String("foo".to_owned()),
        Token::Colon,
        Token::String("bar".to_owned()),
        Token::RBrace,
        Token::Semicolon,

    ];
    for expected_token in expected_stuff {
        let actual_token = lex.next_token();
        assert_eq!(expected_token, actual_token);
    }
}
//     {token.ASSIGN, "="},
// {token.PLUS, "+"},
// {token.LPAREN, "("},
// {token.RPAREN, ")"},
// {token.LBRACE, "{"},
// {token.RBRACE, "}"},
// {token.COMMA, ","},
// {token.SEMICOLON, ";"},
// {token.EOF, ""},
