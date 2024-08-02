use std::fmt::Debug;

use bytes::{BufMut, BytesMut};
use strum::{Display, FromRepr};

use crate::instructions::Instructions;

#[repr(u8)]
#[derive(Clone, Copy, FromRepr, Display)]
pub enum Opcode {
    Constant = 0,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
    Minus,
    Bang
}

impl Opcode {
    pub fn make(&self) -> Instructions {
        let mut instruction = BytesMut::with_capacity(1);
        instruction.put_u8(*self as u8);
        instruction.into()
    }

    pub fn make_with(&self, operands: &[usize]) -> Instructions {
        let def = self.definition();
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
            Opcode::Add => Definition::new("OpAdd", vec![]),
            Opcode::Pop => Definition::new("OpPop", vec![]),
            Opcode::Sub => Definition::new("OpSub", vec![]),
            Opcode::Mul => Definition::new("OpMul", vec![]),
            Opcode::Div => Definition::new("OpDiv", vec![]),
            Opcode::False => Definition::new("OpTrue", vec![]),
            Opcode::True => Definition::new("OpFalse", vec![]),
            Opcode::Equal => Definition::new("OpEqual", vec![]),
            Opcode::NotEqual => Definition::new("OpNotEqual", vec![]),
            Opcode::GreaterThan => Definition::new("OpGreaterThan", vec![]),
            Opcode::Bang => Definition::new("OpBang", vec![]),
            Opcode::Minus => Definition::new("OpMinus", vec![]),
        }
    }
}
impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        Opcode::from_repr(value).expect("expecing a valid opcode")
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
            2 => {
                assert!(
                    instructions.len() > offset + 1,
                    "insructions to short to read number? {}",
                    def.name
                );
                u16::from_be_bytes([instructions[offset], instructions[offset + 1]]) as usize
            }
            _ => todo!(),
        };
        offset += width;
    }

    (operands, offset)
}
