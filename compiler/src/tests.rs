use bytes::Bytes;
use code::opcode::{
    self, read_operands,
    Opcode::{self, *},
};

use code::instructions::Instructions;
use eval::object::Object;
use parser::program::Program;

use crate::compiler::{ByteCode, Compiler};

#[test]
fn test_int_math() {
    let tests = vec![
        Test::new(
            "1+2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Add.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1-2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Sub.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1*2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Mul.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1/2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Div.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1;2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Pop.make(),
                Constant.make_with(&[1]),
                Pop.make(),
            ],
        ),
        Test::new(
            "-1",
            vec![1.into()],
            vec![Constant.make_with(&[0]), Minus.make(), Pop.make()],
        ),
    ];
    run_compiler_tests(&tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![Test::new(
        "if (true) {10};3333;",
        [10.into(), 3333.into()].into(),
        [
            // 0000
            True.make(),
            // 0001
            JumpNotTruthy.make_with(&[7]),
            // 0004
            Constant.make_with(&[0]),
            // 0007
            Pop.make(),
            // 0008
            Constant.make_with(&[1]),
            // 0011
            Pop.make(),
        ]
        .into(),
    )];
    run_compiler_tests(&tests);
}

#[test]
fn test_bools() {
    let tests = vec![
        Test::new("true", vec![], vec![True.make(), Pop.make()]),
        Test::new("true", vec![], vec![True.make(), Pop.make()]),
        Test::new(
            "1-2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Sub.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1*2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Mul.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1/2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Div.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1;2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Pop.make(),
                Constant.make_with(&[1]),
                Pop.make(),
            ],
        ),
        Test::new(
            "1>2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                GreaterThan.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1<2",
            vec![2.into(), 1.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                GreaterThan.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1==2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                Equal.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "1!=2",
            vec![1.into(), 2.into()],
            vec![
                Constant.make_with(&[0]),
                Constant.make_with(&[1]),
                NotEqual.make(),
                Pop.make(),
            ],
        ),
        Test::new(
            "true==false",
            vec![],
            vec![True.make(), False.make(), Equal.make(), Pop.make()],
        ),
        Test::new(
            "true!=false",
            vec![],
            vec![True.make(), False.make(), NotEqual.make(), Pop.make()],
        ),
        Test::new("!true", vec![], vec![True.make(), Bang.make(), Pop.make()]),
        Test::new(
            "-1",
            vec![1.into()],
            vec![Constant.make_with(&[0]), Minus.make(), Pop.make()],
        ),
    ];
    run_compiler_tests(&tests);
}

#[test]
fn test_instructions_string() {
    let instrucitons = vec![
        Add.make(),
        Constant.make_with(&[2]),
        Constant.make_with(&[65535]),
    ];
    let expected = r#"0000 OpAdd 
0001 OpConstant 2
0004 OpConstant 65535
"#;

    let actual = join_instruction(&instrucitons);
    assert_eq!(
        expected,
        format!("{actual}"),
        "Instructions wrongly formatted want={} got={}",
        expected,
        actual
    );
}

#[test]
fn test_read_operands() {
    let tests = vec![TestOperands {
        op: Constant,
        operands: vec![65535],
        bytes_read: 2,
    }];

    for TestOperands {
        op,
        operands,
        bytes_read,
    } in tests
    {
        let instruction = op.make_with(&operands);
        let def = op.definition();

        let (operands_read, n) = read_operands(&def, &instruction[1..]);

        assert_eq!(n, bytes_read, "n wrong. want={}, got={}", bytes_read, n);

        for (i, &want) in operands.iter().enumerate() {
            assert_eq!(
                operands_read[i], want,
                "operand wrong. want={}, got={}",
                want, operands_read[i]
            );
        }
    }
}

fn run_compiler_tests(tests: &[Test]) {
    for Test {
        input,
        expected_constants,
        expected_instructions,
    } in tests
    {
        eprintln!("Running test {input}");
        let program = Program::try_parse(input).expect("Erorr while trying to parse program");
        let mut compiler = Compiler::default();

        compiler
            .compile(program.into())
            .expect("Program should compile");
        let ByteCode {
            instructions,
            constants,
        } = compiler.bytecode();

        let instructions = &Instructions(instructions.clone());
        test_instuction(expected_instructions, instructions);
        test_constants(expected_constants, constants);
    }
}
fn join_instruction(input: &[Instructions]) -> Instructions {
    Bytes::from_iter(input.iter().flat_map(|m| m.0.clone())).into()
}

fn test_instuction(expected: &[Instructions], actual: &Instructions) {

    let expected = join_instruction(expected);

        eprintln!("EXPECTED");
        eprintln!("{}",expected);
        eprintln!("ACTUAL");
        eprintln!("{}",actual);
    assert_eq!(
        expected.len(),
        actual.len(),
        "instructions not the same length want={} got={} ",
        expected,
        actual,
    );

    for (i, b) in expected.iter().enumerate() {
        let actual_code = actual[i];
        assert_eq!(
            actual_code,
            *b,
            "wrong instruction at pos {i} \nwant={:?} \ngot= {:?}",
            expected.to_string(),
            actual.to_string()
        );
    }
}

fn test_constants(expected: &[Object], actual: Vec<Object>) {
    assert_eq!(
        expected.len(),
        actual.len(),
        "constants not the same length want={:?} got={:?}",
        expected,
        actual
    );
    for (i, b) in expected.iter().enumerate() {
        let actual = &actual[i];
        assert_eq!(
            actual, b,
            "wrong constant at pos {i} want={b} got={actual} "
        );
    }
}

struct TestOperands {
    op: Opcode,
    operands: Vec<usize>,
    bytes_read: usize,
}

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
