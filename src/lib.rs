use crate::lexer::lexer::Lexer;

pub mod lexer;

fn main() {
    let lex =Lexer::new(String::from("WOWEE"));
}