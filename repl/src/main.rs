use colored::Colorize;
use compiler::compiler::Compiler;
use eval::{environment::Environment, object::Object};
use lexer::lexer::Lexer;
use parser::parser::Parser;
use std::{cell::RefCell, io::Write, rc::Rc};
use vm::vm::Vm;

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
        loop {
            let mut line = String::new();
            print!("{prompt}");
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut parser = Parser::new(Lexer::new(&line));
            let program = parser.parse_program();
            let mut comp = Compiler::new();
            comp.compile(program.into()).expect("program didn't compile :( ");
            let mut vm = Vm::new(comp.bytecode());
            vm.run().expect("vm couldn't run");

            match vm.last_popped_stack_element().as_ref() {
                Object::Null => (),
                out => println!("{out}"),
            };
            // }
            // if parser.parse_errors.is_empty() {
            //     match eval(program, &env).unwrap().as_ref() {
            //         Object::Null => (),
            //         out => println!("{out}"),
            //     };
            // } else {
            //     println!("Ruh Roh, looks like we ran into some errors while parsing");
            //     for e in parser.parse_errors {
            //         println!("{}", e);
            //     }
            // }
        }
    }
}
