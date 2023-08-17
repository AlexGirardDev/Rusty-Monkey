use eval::eval::eval;
use lexer::lexer::Lexer;
use lexer::token::Token;
use lexer::token::Token::Int;
use parser::parse_error::ParserError::ParserError;
use parser::parser::Parser;
use std::io::Write;
use std::os::linux::raw::stat;

fn main() {
    Repl::start();
}

struct Repl {}

impl Repl {
    fn start() {
        // let parser = Parser {};
        println!(
            "Hello {}! This is the Monkey programming language!",
            whoami::realname()
        );
        println!("Feel free to type in commands");
        loop {
            let mut line = String::new();
            print!(">>>");
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut parser = Parser::new(Lexer::new(&line));
            let program = parser.parse_program();
            if parser.parse_errors.is_empty() {
                println!("{}", eval(program).unwrap());
            } else {
                println!("Ruh Roh, looks like we ran into some errors while parsing");
                for e in parser.parse_errors {
                    println!("{}", e);
                }
            }
        }
    }
}
