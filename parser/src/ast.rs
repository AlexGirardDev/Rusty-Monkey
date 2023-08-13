use lexer::token::Token;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant,
    Identifier(Identifier),
    IntLiteral(i32),
    Bool(bool),
    PrefixExpression(Token, Box<Expression>),
    InfixExpression(Token, Box<Expression>, Box<Expression>),
    IfExpression(Box<Expression>, BlockStatement, Option<BlockStatement>),
    FnExpression(Vec<Identifier>, BlockStatement),
}


impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expression::Constant => write!(f, "Constant expression???"),
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::IntLiteral(i) => write!(f, "{}", i),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::PrefixExpression(op, e) => write!(f, "({}{})", op, e),
            Expression::InfixExpression(op, l_exp, r_exp) => write!(f, "({} {} {})", l_exp, op, r_exp),
            Expression::IfExpression(cond, if_block, else_block) => {
                match else_block {
                    Some(e) => write!(f, "if {} {} else {}", cond, if_block, e),
                    None => write!(f, "if {} {}", cond, if_block)
                }
            }
            Expression::FnExpression(idents, blk) => { write!(f, "fn ({} ) {}", idents.join(", "), blk) }
        };
    }
}

impl Expression {
    pub fn from(token: Token) -> Option<Expression> {
        return match token {
            Token::Int(i) => Some(Expression::IntLiteral(i)),
            Token::Bool(b) => Some(Expression::Bool(b)),
            Token::Ident(s) => Some(Expression::Identifier(s.clone())),
            _ => None,
        };
    }
}

pub type Identifier = String;


pub type Program = BlockStatement;

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        return Ok(());
    }
}

pub struct Precedence;

impl Precedence {
    pub const LOWEST: i8 = 0;
    pub const EQUALS: i8 = 1;
    pub const LESS_GREATER: i8 = 2;
    pub const SUM: i8 = 3;
    pub const PRODUCT: i8 = 4;
    pub const PREFIX: i8 = 5;
    pub const CALL: i8 = 6;

    pub fn from(token: &Token) -> i8 {
        return match token {
            Token::Equal | Token::NotEqual => Precedence::EQUALS,
            Token::LessThan | Token::GreaterThan => Precedence::LESS_GREATER,
            Token::Plus | Token::Dash => Precedence::SUM,
            Token::Asterisk | Token::ForwardSlash => Precedence::PRODUCT,
            _ => Precedence::LOWEST
        };
    }
}


// #[derive(Debug)]
// pub enum Node {
//     Statement(Statement),
//     Expression(Expression),
//     Program(Program),
// }
