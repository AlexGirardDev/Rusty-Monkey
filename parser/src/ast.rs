use lexer::token::Token;

#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    ExpressionStatement(Expression),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Statement::Let(i, e) => write!(f, "let {} = {};", i, e),
            Statement::Return(e) => write!(f, "{} {};", Token::Return, e),
            Statement::ExpressionStatement(e) => write!(f, "{}", e)
        };
    }
}

#[derive(Debug)]
pub enum Expression {
    Constant,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::Constant => write!(f, "Constant expression???"),
        };
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.value);
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// #[derive(Debug)]
// pub enum Node {
//     Statement(Statement),
//     Expression(Expression),
//     Program(Program),
// }
