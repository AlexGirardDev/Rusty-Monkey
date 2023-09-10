use colored::Colorize;
use eval::object::Object;
use eval::{environment::Environment, eval::eval};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{cell::RefCell, io::Write, rc::Rc};

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
        let prompt = ">>>".green();

        println!("Feel free to type in commands");
        let env = Rc::new(RefCell::new(Environment::new_with_builtin()));
        loop {
            let mut line = String::new();
            print!("{prompt}");
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut parser = Parser::new(Lexer::new(&line));
            let program = parser.parse_program();
            if parser.parse_errors.is_empty() {
                match eval(program, &env).unwrap().as_ref() {
                    Object::Null => (),
                    out => println!("{out}"),
                };
            } else {
                println!("Ruh Roh, looks like we ran into some errors while parsing");
                for e in parser.parse_errors {
                    println!("{}", e);
                }
            }
        }
    }
}
