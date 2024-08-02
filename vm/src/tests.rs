use crate::vm::Vm;
use compiler::compiler;
use compiler::Compiler;
use eval::object::Object;
use parser::program::Program;

#[test]
fn test_integer_arithmetic() {
    let tests = [
        Test::new("1", 1),
        Test::new("2", 2),
        Test::new("1+2", 3),
        Test::new("1 * 2", 2),
        Test::new("4 / 2", 2),
        Test::new("50 / 2 * 2 + 10 - 5", 55),
        Test::new("5 + 5 + 5 + 5 - 10", 10),
        Test::new("2 * 2 * 2 * 2 * 2", 32),
        Test::new("5 * 2 + 10", 20),
        Test::new("5 + 2 * 10", 25),
        Test::new("5 * (2 + 10)", 60),
        Test::new("1 < 2", true),
        Test::new("1 > 2", false),
        Test::new("1 < 1", false),
        Test::new("1 > 1", false),
        Test::new("1 == 1", true),
        Test::new("1 != 1", false),
        Test::new("1 == 2", false),
        Test::new("1 != 2", true),
        Test::new("true == true", true),
        Test::new("false == false", true),
        Test::new("true == false", false),
        Test::new("true != false", true),
        Test::new("false != true", true),
        Test::new("(1 < 2) == true", true),
        Test::new("(1 < 2) == false", false),
        Test::new("(1 > 2) == true", false),
        Test::new("(1 > 2) == false", true),
    ];
    run_vm_tests(&tests);
}

fn run_vm_tests(tests: &[Test]) {
    for Test {
        input,
        expected_value,
    } in tests
    {
        let program = Program::try_parse(input).expect("Erorr while trying to parse program");
        let mut comp = Compiler::new();
        comp.compile(program).expect("Program should compile");

        let mut vm = Vm::new(comp.bytecode());
        vm.run().expect("vm should run without errors");
        let stack_element = vm.last_popped_stack_element();
        assert_eq!(
            stack_element.as_ref(),
            expected_value,
            " stack element is not the same want={} got={} Input = {}",
            expected_value,
            stack_element,
            input,
        );
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
