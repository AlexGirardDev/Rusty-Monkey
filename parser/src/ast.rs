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
    Identifier(Identifier),
    IntLiteral(i32),
    PrefixExpression(Token, Box<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::Constant => write!(f, "Constant expression???"),
            Expression::Identifier(i) => write!(f, "{};", i),
            Expression::IntLiteral(i) => write!(f, "{};", i),
            Expression::PrefixExpression(p, e) => write!(f, "{}{}", p, e),
        };
    }
}

pub type Identifier = String;


#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        return Ok(());
    }
}

pub struct Precdenece;

impl Precdenece {
    pub const LOWEST: i8 = 0;
    pub const EQUALS: i8 = 1;
    pub const LESS_GREATER: i8 = 2;
    pub const SUM: i8 = 3;
    pub const PRODUCT: i8 = 4;
    pub const PREFIX: i8 = 5;
    pub const CALL: i8 = 6;
}


// #[derive(Debug)]
// pub enum Node {
//     Statement(Statement),
//     Expression(Expression),
//     Program(Program),
// }
