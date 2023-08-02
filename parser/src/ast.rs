
// #[derive(Debug)]
// pub enum Node {
//     Statement(Statement),
//     Expression(Expression),
//     Program(Program),
// }

#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant,
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String
}

// pub trait Expression: Node {
//     fn expression_node(&self);
// }
//
// impl<T: Expression> Statement for LetStatment<T> {
//     fn statement_node(&self) {}
// }
// impl<T: Expression> Node for LetStatment<T> {}
// impl<T: Expression> Display for LetStatment<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.token.fmt(f)
//     }
// }
//
// struct Identifier {
//     token: Token,
//     value: String,
// }
//
// impl Statement for Identifier {
//     fn statement_node(&self) {}
// }
// impl Node for Identifier {}
// impl Display for Identifier {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.token.fmt(f)
//     }
// }
// // where
// //     T: Node,
// // {
// //     fn node(&self) -> &T {
// //         return &self.value;
// //     }
// //
// //     fn statement_node(&self) {}
// // }
