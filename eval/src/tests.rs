use crate::eval::eval;
use crate::object::Object;
use lexer::lexer::Lexer;
use parser::ast::Program;
use parser::parser::Parser;

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

fn test_eval(input: String) -> Object {
    let program = get_program(input);
    return match eval(program) {
        Ok(o) => o,
        Err(e) => panic!("{}", e),
    };
}

fn get_program(input: String) -> Program {
    let mut p = Parser::new(Lexer::new(&input));
    let program = p.parse_program();
    check_and_print_errors(&p, &program);
    return program;
}

// fn test_int_object(object: Object, value: i64) {
//     if let Object::Int(i) = object {
//         assert_eq!(i, value);
//     } else { panic!("Expected int but got {} ", object) }
// }
fn check_and_print_errors(parser: &Parser, program: &Program) {
    if !parser.parse_errors.is_empty() {
        println!("==============");
        println!("=Parse errors=");
        println!("==============");
        for parse_error in &parser.parse_errors {
            println!(" - {}", parse_error);
        }

        println!("============");
        println!("=Statements=");
        println!("============");
        println!(" - {}", program);
        panic!("ruh roh, program had errors")
    }
}

struct SingleValueTest {
    input: String,
    expected_output: Object,
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
            let obj = test_eval(test.input);
            assert_eq!(obj, test.expected_output);
        }
    }
}
