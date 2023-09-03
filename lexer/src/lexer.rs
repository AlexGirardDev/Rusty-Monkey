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
        lex
    }
    pub fn get_input(&self) -> String {
        String::from_utf8(self.input.to_vec()).unwrap()
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
            b'"' => {
                Token::String(self.read_string_lit())
            }

            b'<' => match self.peak_char() {
                b'=' => {
                    self.read_char();
                    Token::LessThanEqual
                }
                _ => Token::LessThan,
            },
            b'>' => match self.peak_char() {
                b'=' => {
                    self.read_char();
                    Token::GreaterThanEqual
                }
                _ => Token::GreaterThan,
            },
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
        token
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

    fn read_string_lit(&mut self) -> String {
        self.read_char();
        let position = self.position;
        while self.ch != b'"' {
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
