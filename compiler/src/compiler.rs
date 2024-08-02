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
                match expression {
                    Expression::InfixExpression(opperator, left, right) => {
                        self.compile(*left)?;
                        self.compile(*right)?;
                        match opperator {
                            Token::Plus => {
                                self.emit(Opcode::Add, &[]);
                            }
                            t => bail!("{t} is an invalid infix opperator"),
                        };
                    }
                    exp => self.compile(exp)?,
                };

                self.emit(Opcode::Pop, &[]);
            }
            Node::Statement(_) => todo!(),
            Node::Expression(Expression::IntLiteral(i)) => {
                let opperands = &[self.add_constant(Object::Int(i))];
                self.emit(Opcode::Constant, opperands);
            }
            Node::Expression(_) => todo!(),
        };

        Ok(())
    }
    fn add_constant(&mut self, object: impl Into<Object>) -> usize {
        self.constants.push(object.into());
        self.constants.len() - 1
    }

    fn emit(&mut self, opcode: Opcode, operands: &[usize]) -> usize {
        let instructions = opcode.make(operands);
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
