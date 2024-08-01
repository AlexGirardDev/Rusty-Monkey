use std::ops::{Deref, DerefMut};

use itertools::Itertools;

use crate::code::{read_operands, Opcode};

#[derive(Default, Debug)]
pub struct Instructions(pub Vec<u8>);

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;

        while i < self.len() {
            let opcode = Opcode::from_repr(self[i]).unwrap();
            let def = opcode.definition();
            let (opperands, size) = read_operands(&def, &self[i..]);

            writeln!(f, "{:04} {} {}", i, def.name, opperands.iter().join(" "))?;

            i += size + 1;
        }
        Ok(())
    }
}
impl Deref for Instructions {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Instructions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<u8>> for Instructions {
    fn from(value: Vec<u8>) -> Self {
        Instructions(value)
    }
}
