use crate::eval::eval;
use crate::eval_error::EvalError;
use crate::node::Node;
use crate::object::Object;
use colored::Colorize;
use lexer::lexer::Lexer;
use parser::ast::Program;
use parser::parser::Parser;

#[test]
fn test_error_exp() {
    let tests: Vec<ErrorTest> = vec![
        ErrorTest::new_type_missmatch("5+true", 5, true),
        ErrorTest::new_type_missmatch("5+true;5", 5, true),
        ErrorTest::new_invalid_operation("true+true;5", true, "+", true),
        // SingleValueTest::new("return 2*5;9;", 10),
        // SingleValueTest::new("9; return 2*5; 9;", 10),
        // SingleValueTest::new("if (10>1) { if (10>1) { return 10;} return 1;}", 10),
    ];
    ErrorTest::test(tests);
}

#[test]
fn test_return_exp() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("return 10;", 10),
        SingleValueTest::new("return 10; 9;", 10),
        SingleValueTest::new("return 2*5;9;", 10),
        SingleValueTest::new("9; return 2*5; 9;", 10),
        SingleValueTest::new("if (10>1) { if (10>1) { return 10;} return 1;}", 10),
    ];
    SingleValueTest::test(tests);
}
#[test]
fn test_if_else_exp() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("if(true){10}", 10),
        SingleValueTest::new("if(false){11}", Object::Null),
        SingleValueTest::new("if (true) {10}", 10),
        SingleValueTest::new("if (false) {10}", Object::Null),
        SingleValueTest::new("if (1) {10}", 10),
        SingleValueTest::new("if (1<2) {10}", 10),
        SingleValueTest::new("if (1<2) { 10} else {20}", 10),
        SingleValueTest::new("if (1>2) {10} else {20}", 20),
        SingleValueTest::new("if (1>=1) {10} else {100}", 10),
        SingleValueTest::new("if (1<=1) {return 10;} else {100}", 10),
        SingleValueTest::new("if(true){11}", 11),
        SingleValueTest::new("if(true){11}", 11),
        SingleValueTest::new("if(true){11}", 11),
    ];
    SingleValueTest::test(tests);
}
#[test]
fn test_eval_bang_operator_exp() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("!true", false),
        SingleValueTest::new("!false", true),
        SingleValueTest::new("!5", false),
        SingleValueTest::new("!!true", true),
        SingleValueTest::new("!!!!!!!!!!!!!!!!!!!!!!!!false", false),
        SingleValueTest::new("!!5", true),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_eval_int_exp() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("5", 5),
        SingleValueTest::new("10", 10),
        SingleValueTest::new("5 + 5 + 5 + 5 - 10", 10),
        SingleValueTest::new("2*2*2*2*2", 32),
        SingleValueTest::new("-50+100+-50", 0),
        SingleValueTest::new("5+5+5+5-10", 10),
        SingleValueTest::new("2*2*2*2*2", 32),
        SingleValueTest::new("-50+100+ -50", 0),
        SingleValueTest::new("5*2+10", 20),
        SingleValueTest::new("5+2*10", 25),
        SingleValueTest::new("20 + 2 * -10", 0),
        SingleValueTest::new("50/2 * 2 +10", 60),
        SingleValueTest::new("2*(5+10)", 30),
        SingleValueTest::new("3*3*3+10", 37),
        SingleValueTest::new("3*(3*3)+10", 37),
        SingleValueTest::new("(5+10*2+15/3)*2+-10", 50),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_eval_bool_exp() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("true", Object::Bool(true)),
        SingleValueTest::new("false", Object::Bool(false)),
        SingleValueTest::new("1<2", true),
        SingleValueTest::new("1>2", false),
        SingleValueTest::new("1<1", false),
        SingleValueTest::new("1>1", false),
        SingleValueTest::new("1==1", true),
        SingleValueTest::new("(1<2) == true", true),
        SingleValueTest::new("(1>2) == true", false),
        SingleValueTest::new("(1<2) != true", false),
        SingleValueTest::new("(1>2) != true", true),
    ];
    SingleValueTest::test(tests);
}

fn test_eval(input: impl Into<String>) -> Result<Object, EvalError> {
    let program = get_program(input.into());
    eval(Node::Program(program))
}

fn get_program(input: String) -> Program {
    let mut p = Parser::new(Lexer::new(&input));
    let program = p.parse_program();
    p.check_and_print_errors(&program);
    return program;
}

// fn test_int_object(object: Object, value: i64) {
//     if let Object::Int(i) = object {
//         assert_eq!(i, value);
//     } else { panic!("Expected int but got {} ", object) }
// }

struct SingleValueTest {
    input: String,
    expected_output: Object,
}

struct ErrorTest {
    input: String,
    expected_output: EvalError,
}

impl SingleValueTest {
    pub fn new(input: &str, output: impl Into<Object>) -> Self {
        SingleValueTest {
            input: String::from(input),
            expected_output: output.into(),
        }
    }
    pub fn test(tests: Vec<SingleValueTest>) {
        for test in tests {
            let obj = test_eval(&test.input).unwrap();
            assert_eq!(
                obj,
                test.expected_output,
                "Input: {}",
                test.input.bright_yellow()
            );
        }
    }
}

// impl From<(Object, String, Object)> for ErrorTest {
//     fn from(lhs: impl Into<Object>, opp: impl  Into<String>,  rhs : impl Into<Object>){
//
//
//         // add code here
//     }
// }

impl ErrorTest {
    pub fn new(input: &str, output: EvalError) -> Self {
        ErrorTest {
            input: String::from(input),
            expected_output: output,
        }
    }

    pub fn new_type_missmatch(input: &str, lhs: impl Into<Object>, rhs: impl Into<Object>) -> Self {
        ErrorTest {
            input: String::from(input),
            expected_output: EvalError::TypeMismatch(lhs.into(), rhs.into()),
        }
    }

    pub fn new_invalid_operation(
        input: &str,
        lhs: impl Into<Object>,
        opp: impl Into<String>,
        rhs: impl Into<Object>,
    ) -> Self {
        ErrorTest {
            input: String::from(input),
            expected_output: EvalError::InvalidOperator(lhs.into(), opp.into(), rhs.into()),
        }
    }

    // pub fn new_invalid_prefix(input: &str, Into<Object>, opp: impl Into<String>,rhs: impl Into<Object> ) -> Self {
    //     ErrorTest {
    //         input: String::from(input),
    //         expected_output: EvalError::InvalidOperator(lhs.into(),opp.into(), rhs.into()),
    //     }
    // }
    pub fn test(tests: Vec<ErrorTest>) {
        for test in tests {
            let e = test_eval(&test.input).unwrap_err();
            assert_eq!(e, test.expected_output, "Input: {}", test.input);
        }
    }
}
