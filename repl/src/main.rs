use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::parser::Parser;

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
            println!(">>");
            std::io::stdin().read_line(&mut line).unwrap();
            let mut l = Lexer::new(&line);
            let mut tok = l.next_token();
            while tok != Token::Eof {
                println!("{}", tok);
                tok = l.next_token();
            }
        }
    }
}
