use std::fmt::Debug;

use bytes::{BufMut, BytesMut};
use strum::{Display, FromRepr};

use crate::instructions::Instructions;

#[repr(u8)]
#[derive(Clone, Copy, FromRepr, Display)]
pub enum Opcode {
    Constant = 0,
}

impl Opcode {
    pub fn make(&self, operands: &[usize]) -> Instructions {
        let def = self.definition();
        eprintln!("making opcode {self} {:?}", operands);
        let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;
        let mut instruction = BytesMut::with_capacity(instruction_len);
        instruction.put_u8(*self as u8);
        for (i, op) in operands.iter().enumerate() {
            let width = def.operand_widths[i];
            match width {
                2 => instruction.put_u16(*op as u16),
                _ => todo!(),
            };
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
            2 => u16::from_be_bytes([instructions[offset], instructions[offset + 1]]) as usize,
            _ => todo!(),
        };
        offset += width;
    }

    (operands, offset)
}
