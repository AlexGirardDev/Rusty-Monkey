use crate::lexer::token::*;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            ch: 0,
            read_position: 0,
            input: input.into_bytes(),
            position: 0,
        };

        lex.read_char();
        return lex;
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        println!("{}", self.position);
        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            ch => {
                if ch.is_ascii_digit() {
                    return Token::Int(self.read_int());
                } else if ch.is_ascii_alphabetic() {
                    let ident = self.read_ident();
                    return match ident.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        _ => { Token::Ident(ident) }
                    };
                }
                Token::Eof
            }
        };
        self.read_char();
        return token;
    }

    fn read_int(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[position..self.position]).to_string();
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
        println!("{}", self.ch as char);
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
        let input = String::from("=+(){},;");
        let mut lex = Lexer::new(input);
        let expected_stuff: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::LSquirly,
            Token::RSquirly,
            Token::Comma,
            Token::Semicolon,
        ];
        for stuff in expected_stuff
        {
            let token = lex.next_token();
            assert_eq!(token, stuff);
        }
    }

    #[test]
    fn TestNextToken()
    {
        let input = String::from("let five = 5;\
            let ten = 10;\
            let add = fn (x, y)\
            {\
                x + y;\
            };\
\
            let result = add(five, ten);\
            ");
        let mut lex = Lexer::new(input);

        let expected_stuff: Vec<Token> = vec![
            { Token::Let },
            { Token::Ident(String::from("five")) },
            { Token::Assign },
            { Token::Int(String::from("5")) },
            { Token::Semicolon },
            { Token::Let },
            { Token::Ident(String::from("ten")) },
            { Token::Assign },
            { Token::Int(String::from("10")) },
            { Token::Semicolon },
            { Token::Let },
            { Token::Ident(String::from("add")) },
            { Token::Assign },
            { Token::Function },
            { Token::Lparen },
            { Token::Ident(String::from("x")) },
            { Token::Comma },
            { Token::Ident(String::from("y")) },
            { Token::Rparen },
            { Token::LSquirly },
            { Token::Ident(String::from("x")) },
            { Token::Plus },
            { Token::Ident(String::from("y")) },
            { Token::Semicolon },
            { Token::RSquirly },
            { Token::Semicolon },
            { Token::Let },
            { Token::Ident(String::from("result")) },
            { Token::Assign },
            { Token::Ident(String::from("add")) },
            { Token::Lparen },
            { Token::Ident(String::from("five")) },
            { Token::Comma },
            { Token::Ident(String::from("ten")) },
            { Token::Rparen },
            { Token::Semicolon },
            { Token::Eof },
        ];
        for expected_token in expected_stuff
        {
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