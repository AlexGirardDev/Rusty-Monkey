use code::code::Instructions;
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

    pub fn compile(&self, _node: Node) {}
    pub fn bytecode(&self) -> ByteCode {
        ByteCode {
            instructions: &self.insturctions,
            constants: &self.constants,
        }
    }
}

pub struct ByteCode<'a> {
    instructions: &'a Instructions,
    constants: &'a [Object],
}
