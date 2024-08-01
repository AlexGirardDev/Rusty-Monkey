use std::fmt::Debug;

use strum::FromRepr;

use crate::instructions::Instructions;

#[repr(u8)]
#[derive(Clone, Copy, FromRepr)]
pub enum Opcode {
    Constant = 0,
}

impl Opcode {
    pub fn make(&self, operands: &[usize]) -> Instructions {
        let def = self.definition();
        let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;
        let mut instruction: Vec<u8> = vec![0; instruction_len];
        instruction[0] = *self as u8;
        let mut offset = 1;
        for (i, op) in operands.iter().enumerate() {
            let width = def.operand_widths[i];
            match width {
                2 => instruction[offset..].copy_from_slice(&(*op as u16).to_be_bytes()),
                _ => todo!(),
            };
            offset += width;
        }
       instruction.into()
    }
    pub fn definition(&self) -> Definition {
        match self {
            Opcode::Constant => Definition::new("OpConstant", vec![2]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<usize>,
}

impl Definition {
    pub fn new(name: &str, operand_widths: Vec<usize>) -> Self {
        Definition {
            name: name.to_string(),
            operand_widths,
        }
    }
}
pub fn read_operands(def: &Definition, instructions: &[u8]) -> (Vec<usize>, usize) {
    let mut operands: Vec<usize> = vec![0; def.operand_widths.len()];

    let mut offset = 0;
    for (i, width) in def.operand_widths.iter().enumerate() {
        operands[i] = match width {
            2 => u16::from_be_bytes([instructions[offset+1], instructions[offset + 2]]) as usize,
            _ => todo!(),
        };
        offset += width;
    }

    (operands, offset)
}
