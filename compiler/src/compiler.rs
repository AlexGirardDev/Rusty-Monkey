use anyhow::{bail, Ok, Result};
use bytes::BytesMut;
use code::instructions::Instructions;
use code::opcode::Opcode;
use eval::node::Node;
use eval::object::Object;
use lexer::token::Token;
use parser::ast::{BlockStatement, Expression, Statement};

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

    pub fn compile(&mut self, node: impl Into<Node>) -> Result<()> {
        let node: Node = node.into();
        match node {
            Node::BlockStatement(BlockStatement { statements }) => {
                for statement in statements {
                    self.compile(statement)?;
                }
            }
            Node::Program(_) => todo!(),
            Node::Object(_) => todo!(),
            Node::Statement(Statement::ExpressionStatement(expression)) => {
                self.compile(expression)?;
                self.emit(Opcode::Pop, &[]);
            }
            Node::Statement(_) => todo!(),
            Node::Expression(Expression::IntLiteral(i)) => {
                let opperands = &[self.add_constant(Object::Int(i))];
                self.emit(Opcode::Constant, opperands);
            }
            Node::Expression(exp) => {
                match exp {
                    Expression::Identifier(_) => todo!(),
                    Expression::IntLiteral(_) => todo!(),
                    Expression::StringLiteral(_) => todo!(),
                    Expression::Bool(true) => {
                        self.emit(Opcode::True, &[]);
                    }
                    Expression::Bool(false) => {
                        self.emit(Opcode::False, &[]);
                    }
                    Expression::PrefixExpression(token, right) => match token {
                        Token::Bang => {
                            self.compile(*right)?;
                            self.emit(Opcode::Bang, &[]);
                        }
                        Token::Dash => {
                            self.compile(*right)?;
                            self.emit(Opcode::Minus, &[]);
                        }
                        t => bail!("{t} is an invalid token for a prefix exrpession"),
                    },
                    Expression::InfixExpression(opperator, left, right) => {
                        match opperator {
                            Token::Plus => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::Add, &[]);
                            }
                            Token::Dash => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::Sub, &[]);
                            }
                            Token::Asterisk => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::Mul, &[]);
                            }
                            Token::ForwardSlash => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::Div, &[]);
                            }
                            Token::Equal => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::Equal, &[]);
                            }
                            Token::NotEqual => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::NotEqual, &[]);
                            }
                            Token::GreaterThan => {
                                self.compile(*left)?;
                                self.compile(*right)?;
                                self.emit(Opcode::GreaterThan, &[]);
                            }
                            Token::LessThan => {
                                self.compile(*right)?;
                                self.compile(*left)?;
                                self.emit(Opcode::GreaterThan, &[]);
                            }
                            t => bail!("{t} is an invalid infix opperator"),
                        };
                    }
                    Expression::IfExpression(_, _, _) => todo!(),
                    Expression::FnExpression(_, _) => todo!(),
                    Expression::CallExpression(_, _) => todo!(),
                    Expression::Arrary(_) => todo!(),
                    Expression::Map(_) => todo!(),
                    Expression::IndexExpression(_, _) => todo!(),
                };
            }
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
