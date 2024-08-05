use std::fmt::Display;
use std::{clone, mem};

use anyhow::{bail, Ok, Result};
use bytes::BytesMut;
use code::opcode::Opcode;
use code::{instructions::Instructions, opcode};
use eval::node::Node;
use eval::object::Object;
use lexer::token::Token;
use parser::{
    ast::{BlockStatement, Expression, Statement},
    program::Program,
};

#[derive(Clone)]
struct EmittedInstruction {
    opcode: Opcode,
    position: usize,
}

impl EmittedInstruction {
    fn new(opcode: Opcode, position: usize) -> Self {
        Self { opcode, position }
    }
}
impl Display for EmittedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.opcode, self.position)
    }
}

pub struct Compiler {
    insturctions: BytesMut,
    constants: Vec<Object>,
    last_instruction: EmittedInstruction,
    previous_instruction: EmittedInstruction,
}

impl Compiler {
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

    fn compile_program(&mut self, _block: &Program) -> Result<()> {
        todo!();
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
            Expression::IfExpression(con, consequence, else_exp) => {
                self.compile_expression(con)?;
                let jump_not_truthy_pos = self.emit(Opcode::JumpNotTruthy, &[9999]);

                self.compile_block(consequence)?;
                if self.last_insruction_is_pop() {
                    self.remove_last_pop();
                }

                match else_exp {
                    Some(exp) => {
                        let jump_pos = self.emit(Opcode::Jump, &[9999]);
                        let after_consequence_position = self.insturctions.len();
                        self.change_operand(jump_not_truthy_pos, after_consequence_position);
                        self.compile_block(exp)?;

                        if self.last_insruction_is_pop() {
                            self.remove_last_pop();
                        }
                        self.change_operand(jump_pos, self.insturctions.len());
                    }
                    None => {
                        let after_consequence_position = self.insturctions.len();
                        self.change_operand(jump_not_truthy_pos, after_consequence_position);
                    }
                }
            }
            Expression::FnExpression(_, _) => todo!(),
            Expression::CallExpression(_, _) => todo!(),
            Expression::Arrary(_) => todo!(),
            Expression::Map(_) => todo!(),
            Expression::IndexExpression(_, _) => todo!(),
        };

        Ok(())
    }
    fn debug_print(&self) {
        if true {
            return;
        }
        eprintln!("Length :{}", self.insturctions.len());
        eprintln!("Prev :{}", self.previous_instruction);
        eprintln!("Last :{}", self.last_instruction);
        eprintln!("{}", Instructions::from(self.insturctions.clone()));
    }

    fn change_operand(&mut self, position: usize, operand: usize) {
        let opcode: Opcode = self.insturctions[position].into();
        let instruction = opcode.make_with(&[operand]);
        self.replace_instruction(position, &instruction)
    }

    fn replace_instruction(&mut self, position: usize, instruction: &[u8]) {
        self.insturctions[position..position + instruction.len()].copy_from_slice(instruction);
    }

    fn last_insruction_is_pop(&self) -> bool {
        matches!(self.last_instruction.opcode, Opcode::Pop)
    }

    fn remove_last_pop(&mut self) {
        self.insturctions
            .resize(self.last_instruction.position, 0x0);
        self.last_instruction = self.previous_instruction.clone();
    }

    fn add_constant(&mut self, object: impl Into<Object>) -> usize {
        self.constants.push(object.into());
        self.constants.len() - 1
    }

    fn emit(&mut self, opcode: Opcode, operands: &[usize]) -> usize {
        let instructions = opcode.make_with(operands);
        let pos = self.add_instruction(instructions);
        self.set_last_instruction(opcode, pos);
        self.debug_print();
        pos
    }

    fn set_last_instruction(&mut self, opcode: Opcode, position: usize) {
        //this is some cute code to avoid having to clone
        //really i'm just taking the new value and putting it in last_instruction
        //and then putting last inte previous and getting rid of previous
        self.previous_instruction = EmittedInstruction::new(opcode, position);
        mem::swap(&mut self.previous_instruction, &mut self.last_instruction);
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

impl Default for Compiler {
    fn default() -> Self {
        Self {
            insturctions: BytesMut::default(),
            constants: Vec::new(),
            last_instruction: EmittedInstruction {
                opcode: Opcode::NoOp,
                position: 0,
            },
            previous_instruction: EmittedInstruction {
                opcode: Opcode::NoOp,
                position: 0,
            },
        }
    }
}

pub struct ByteCode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}
