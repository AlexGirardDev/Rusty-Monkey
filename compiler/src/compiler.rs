use anyhow::{Ok, Result};
use code::code::Definition;
use code::instructions::Instructions;
use eval::node::Node;
use eval::object::Object;

#[derive(Default)]
pub struct Compiler {
    insturctions: Instructions,
    constants: Vec<Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            insturctions: Instructions::default(),
            constants: Vec::new(),
        }
    }

    pub fn compile(&mut self, _node: impl Into<Node>) -> Result<()> {
        Ok(())
    }
    pub fn bytecode(&self) -> ByteCode {
        ByteCode {
            instructions: &self.insturctions,
            constants: &self.constants,
        }
    }
}

pub struct ByteCode<'a> {
    pub instructions: &'a Instructions,
    pub constants: &'a [Object],
}
pub fn read_operands(def:&Definition, instructions:Instructions) ->(Vec<usize>, usize) {

    let mut operands = vec![0;def.operand_widths.len()];

    todo!()
}
