use crate::vm::Vm;
use compiler::compiler;
use compiler::Compiler;
use eval::object::Object;
use parser::program::Program;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![Test::new("1", 1), Test::new("2", 2), Test::new("1+2", 3)];

    run_vm_tests(&tests);
}

fn run_vm_tests(tests: &[Test]) {
    for Test { 
        input,
        expected_value,
    } in tests
    {
        dbg!(input,expected_value);

        let program = Program::try_parse(input).expect("Erorr while trying to parse program");
        eprintln!("Parsed program {}", program);
        let mut comp = Compiler::new();
        comp.compile(program).expect("Program should compile");

        let mut vm = Vm::new(comp.bytecode());
        vm.run().expect("vm should run without errors");
        let stack_element = vm.stack_top().expect("expeced value from stack");
        assert_eq!(stack_element.as_ref(), expected_value,"stack element is not the same want={} got={}", expected_value, stack_element);
    }
}

struct Test {
    input: String,
    expected_value: Object,
}

impl Test {
    fn new(input: impl Into<String>, expected_value: impl Into<Object>) -> Self {
        Self {
            input: input.into(),
            expected_value: expected_value.into(),
        }
    }
}
