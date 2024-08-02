use std::ops::{Deref, DerefMut};

use bytes::{Bytes, BytesMut};
use itertools::Itertools;

use crate::code::{read_operands, Opcode};

#[derive(Default, Debug)]
pub struct Instructions(pub Bytes);

impl Instructions {
    pub fn read_u16(&self, offset:usize )->u16{
        u16::from_be_bytes([self[offset], self[offset+1]])
    }
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut offset = 0;

        while offset < self.len() {
            let opcode = Opcode::from_repr(self[offset]).unwrap();
            let def = opcode.definition();
            let (opperands, size) = read_operands(&def, &self[offset + 1..]);

            writeln!(
                f,
                "{:04} {} {}",
                offset,
                def.name,
                opperands.iter().join(" ")
            )?;

            offset += 1 + size;
        }
        Ok(())
    }
}
impl Deref for Instructions {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Instructions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Bytes> for Instructions {
    fn from(value: Bytes) -> Self {
        Instructions(value)
    }
}

impl From<BytesMut> for Instructions {
    fn from(value: BytesMut) -> Self {
        Instructions(value.into())
    }
}

