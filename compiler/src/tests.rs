use code::code::{Instructions, Opcode};
use eval::object::Object;

struct Test {
    input: String,
    expected_constants: Vec<Object>,
    expected_instructions: Vec<Instructions>,
}

impl Test {
    fn new(
        input: impl Into<String>,
        expected_constants: Vec<Object>,
        expected_instructions: Vec<Instructions>,
    ) -> Self {
        Self {
            input: input.into(),
            expected_constants,
            expected_instructions,
        }
    }
}

#[test]
fn compiler_test() {
    let tests = vec![Test::new(
        "1+2",
        vec![1.into(), 2.into()],
        vec![Opcode::Constant.make(&[1]), Opcode::Constant.make(&[2])],
    )];
    run_compiler_tests(&tests);
}

fn run_compiler_tests(tests: &[Test]) {
    for test in tests {

    }
}
