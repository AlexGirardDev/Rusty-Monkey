use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Lexer {
            ch: 0,
            read_position: 0,
            input: input.as_bytes(),
            position: 0,
        };

        lex.read_char();
        return lex;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let token = match self.ch {
            b'=' => match self.peak_char() {
                b'=' => {
                    self.read_char();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParent,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBracket,
            b'}' => Token::RBracket,
            b'!' => match self.peak_char() {
                b'=' => {
                    self.read_char();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },
            b'-' => Token::Dash,
            b'/' => Token::ForwardSlash,
            b'*' => Token::Asterisk,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            ch => {
                if ch.is_ascii_digit() {
                    return Token::Int(self.read_int());
                } else if ch.is_ascii_alphabetic() {
                    let ident = self.read_ident();
                    return match ident.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        "return" => Token::Return,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "if" => Token::If,
                        "else" => Token::Else,
                        _ => Token::Ident(ident),
                    };
                }
                Token::Eof
            }
        };
        self.read_char();
        return token;
    }

    fn read_int(&mut self) -> i64 {
        let position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[position..self.position])
            .to_string()
            .parse::<i64>()
            .expect("expected int to parse but failed??");
    }
    fn read_ident(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
    }

    fn eat_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 7
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&mut self) -> u8 {
        self.input[self.read_position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let input = "=+(){},;";
        let mut lex = Lexer::new(&input);
        let expected_stuff: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParent,
            Token::LBracket,
            Token::RBracket,
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
        let input =
            "let five = 5;\
            let ten = 10;\
            let add = fn (x, y)\
            {\
                x + y;\
            };\
\
            let result = add(five, ten);\
            !-/*5;\
            5 < 10 > 5;\
if (5 < 10) {
    return true;
} else {
    return false;
}`
10 == 10;
10 != 9;
`";
        let mut lex = Lexer::new(input);

        let expected_stuff: Vec<Token> = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParent,
            Token::LBracket,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBracket,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
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
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::RParent,
            Token::LBracket,
            Token::Return,
            Token::Bool(true),
            Token::Semicolon,
            Token::RBracket,
            Token::Else,
            Token::LBracket,
            Token::Return,
            Token::Bool(false),
            Token::Semicolon,
            Token::RBracket,
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
}

