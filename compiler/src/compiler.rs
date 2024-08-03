use anyhow::{bail, Ok, Result};
use bytes::BytesMut;
use code::instructions::Instructions;
use code::opcode::Opcode;
use eval::node::Node;
use eval::object::Object;
use lexer::token::Token;
use parser::{
    ast::{BlockStatement, Expression, Statement},
    program::Program,
};

#[derive(Default)]
pub struct Compiler {
    insturctions: BytesMut,
    constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            insturctions: BytesMut::default(),
            constants: Vec::new(),
        }
    }

    pub fn compile(&mut self, node: Node) -> Result<()> {
        match &node {
            Node::BlockStatement(s) => self.compile_block(s),
            Node::Program(p) => self.compile_program(p),
            Node::Statement(s) => self.compile_statement(s),
            Node::Expression(e) => self.compile_expression(e),
            Node::Object(_) => todo!(),
        }
    }
    fn compile_block(&mut self, block: &BlockStatement) -> Result<()> {
        for statement in &block.statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_program(&mut self, block: &Program) -> Result<()> {
        todo!()
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Let(_, _) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::ExpressionStatement(exprssion) => {
                self.compile_expression(exprssion)?;
                self.emit(Opcode::Pop, &[]);
            }
        };
        Ok(())
    }

    pub fn compile_expression(&mut self, exp: &Expression) -> Result<()> {
        match exp {
            Expression::Identifier(_) => todo!(),
            Expression::IntLiteral(i) => {
                let instruction = self.add_constant(Object::Int(*i));
                self.emit(Opcode::Constant, &[instruction]);
            }
            Expression::StringLiteral(_) => todo!(),
            Expression::Bool(true) => {
                self.emit(Opcode::True, &[]);
            }
            Expression::Bool(false) => {
                self.emit(Opcode::False, &[]);
            }
            Expression::PrefixExpression(token, right) => match token {
                Token::Bang => {
                    self.compile_expression(right)?;
                    self.emit(Opcode::Bang, &[]);
                }
                Token::Dash => {
                    self.compile_expression(right)?;
                    self.emit(Opcode::Minus, &[]);
                }
                t => bail!("{t} is an invalid token for a prefix exrpession"),
            },
            Expression::InfixExpression(opperator, left, right) => {
                match opperator {
                    Token::Plus => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::Add, &[]);
                    }
                    Token::Dash => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::Sub, &[]);
                    }
                    Token::Asterisk => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::Mul, &[]);
                    }
                    Token::ForwardSlash => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::Div, &[]);
                    }
                    Token::Equal => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::Equal, &[]);
                    }
                    Token::NotEqual => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::NotEqual, &[]);
                    }
                    Token::GreaterThan => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.emit(Opcode::GreaterThan, &[]);
                    }
                    Token::LessThan => {
                        self.compile_expression(right)?;
                        self.compile_expression(left)?;
                        self.emit(Opcode::GreaterThan, &[]);
                    }
                    t => bail!("{t} is an invalid infix opperator"),
                };
            }
            Expression::IfExpression(con, if_exp, else_exp) => {
                // self.compile(node)
            }
            Expression::FnExpression(_, _) => todo!(),
            Expression::CallExpression(_, _) => todo!(),
            Expression::Arrary(_) => todo!(),
            Expression::Map(_) => todo!(),
            Expression::IndexExpression(_, _) => todo!(),
        };

        Ok(())
    }

    fn add_constant(&mut self, object: impl Into<Object>) -> usize {
        self.constants.push(object.into());
        self.constants.len() - 1
    }

    fn emit(&mut self, opcode: Opcode, operands: &[usize]) -> usize {
        let instructions = opcode.make_with(operands);
        self.add_instruction(instructions)
    }

    fn add_instruction(&mut self, instruction: Instructions) -> usize {
        let position = self.insturctions.len();
        self.insturctions.extend(instruction.0);
        position
    }

    pub fn bytecode(self) -> ByteCode {
        ByteCode {
            instructions: self.insturctions.clone().into(),
            constants: self.constants,
        }
    }
}

pub struct ByteCode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
