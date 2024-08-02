use std::{default, rc::Rc};

use anyhow::{bail, Context, Ok, Result};
use bytes::Buf;
use code::{code::Opcode, instructions::Instructions};
use compiler::compiler::ByteCode;
use eval::object::Object;
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
        Self {
            insturctions: byte_code.instructions,
            constants: byte_code.constants.into_iter().map(Rc::new).collect_vec(),
            stack: Vec::with_capacity(Vm::STACKSIZE),
            sp: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut ip = 0;
        dbg!(&self.insturctions);
        while ip < self.insturctions.len() {
            match self.insturctions[ip].into() {
                Opcode::Constant => {
                    eprintln!("pushing to stack");
                    let const_index = self.insturctions.read_u16(ip + 1);
                    self.push_const(const_index)?;
                    ip += 2;
                }
            }
            ip += 1;
        }
        Ok(())
    }
    fn push(&mut self, value: Rc<Object>) -> Result<()> {
        if self.sp > Vm::STACKSIZE {
            bail!("stack overflow");
        }
        self.stack.push(value);
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

    pub fn stack_top(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp - 1).cloned()
    }
}
