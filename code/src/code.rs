
pub type Instructions = Vec<u8>;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Opcode {
    Constant = 0,
}

impl Opcode {
    pub fn make(&self, operands: &[usize]) -> Vec<u8> {
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
        instruction
    }
    pub fn definition(&self) -> Definition {
        match self {
            Opcode::Constant => Definition::new("Constant", vec![2]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Definition {
    name: String,
    operand_widths: Vec<usize>,
}

impl Definition {
    pub fn new(name: &str, operand_widths: Vec<usize>) -> Self {
        Definition {
            name: name.to_string(),
            operand_widths,
        }
    }
}
