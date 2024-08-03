use std::rc::Rc;

use anyhow::{bail, Context, Ok, Result};
use code::{instructions::Instructions, opcode::Opcode};
use compiler::compiler::ByteCode;
use eval::object::{Object, ObjectComparison};
use itertools::Itertools;

pub struct Vm {
    insturctions: Instructions,
    constants: Vec<Rc<Object>>,
    stack: Vec<Rc<Object>>,
    sp: usize,
}

impl Vm {
    const STACKSIZE: usize = 2048;

    pub fn new(byte_code: ByteCode) -> Self {
        let stack = (0..Vm::STACKSIZE)
            .map(|_| Rc::new(Object::Null))
            .collect_vec();
        Self {
            insturctions: byte_code.instructions,
            constants: byte_code.constants.into_iter().map(Rc::new).collect_vec(),
            stack,
            sp: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut ip = 0;
        while ip < self.insturctions.len() {
            match self.insturctions[ip].into() {
                Opcode::Constant => {
                    let const_index = self.insturctions.read_u16(ip + 1);
                    self.push_const(const_index)?;
                    ip += 2;
                }
                Opcode::Add => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push((left.as_ref() + right.as_ref())?)?;
                }
                Opcode::Sub => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push((left.as_ref() - right.as_ref())?)?;
                }
                Opcode::Mul => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push((left.as_ref() * right.as_ref())?)?;
                }
                Opcode::Div => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push((left.as_ref() / right.as_ref())?)?;
                }
                Opcode::Pop => {
                    self.pop()?;
                }
                Opcode::True => {
                    self.push(Object::Bool(true))?;
                }
                Opcode::False => {
                    self.push(Object::Bool(false))?;
                }
                Opcode::Equal => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(Object::Bool(left == right))?;
                }
                Opcode::NotEqual => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    self.push(Object::Bool(left != right))?;
                }
                Opcode::GreaterThan => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    let result =
                        Object::eval_obj_comparison(left, right, ObjectComparison::GreaterThan)?;

                    self.push(result)?;
                }
                Opcode::Minus => {
                    let right = self.pop()?;

                    let result = match right.as_ref() {
                        Object::Int(i) => Object::Int(-*i),
                        t => bail!("{t} is not a valid type for - prefix"),
                    };
                    self.push(result)?;
                }
                Opcode::Bang => {
                    let right = self.pop()?;
                    let result = match right.as_ref() {
                        Object::Bool(b) => !*b,
                        Object::Null => true,
                        _ => false,
                    };
                    self.push(Object::Bool(result))?;
                }
                Opcode::Jump => todo!(),
                Opcode::JumpNotTruthy => todo!(),

            }
            ip += 1;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Rc<Object>> {
        if self.sp > Vm::STACKSIZE {
            bail!("stack overflow");
        }
        let item = self
            .stack
            .get(self.sp - 1)
            .context("tried to pop the stack");
        self.sp -= 1;
        item.cloned()
    }

    fn push(&mut self, value: impl Into<Rc<Object>>) -> Result<()> {
        if self.sp > Vm::STACKSIZE {
            bail!("stack overflow");
        }
        self.stack[self.sp] = value.into();
        self.sp += 1;
        Ok(())
    }

    fn push_const(&mut self, index: impl Into<usize>) -> Result<()> {
        let constant = self
            .constants
            .get(index.into())
            .context("couldn't find constant")?
            .clone();
        self.push(constant)?;
        Ok(())
    }

    pub fn last_popped_stack_element(&self) -> Rc<Object> {
        self.stack[self.sp].clone()
    }
}
