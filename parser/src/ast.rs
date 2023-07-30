use std::fmt::Display;

// use lexer::lexer::Lexer;
use lexer::token::Token;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Ast {}

pub trait Node: Display {}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

struct LetStatment<T: Expression> {
    token: Token,
    name: Identifier,
    value: T,
}

impl<T: Expression> Statement for LetStatment<T> {
    fn statement_node(&self) {}
}
impl<T: Expression> Node for LetStatment<T> {}
impl<T: Expression> Display for LetStatment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.token.fmt(f)
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Statement for Identifier {
    fn statement_node(&self) {}
}
impl Node for Identifier {}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.token.fmt(f)
    }
}
// where
//     T: Node,
// {
//     fn node(&self) -> &T {
//         return &self.value;
//     }
//
//     fn statement_node(&self) {}
// }
