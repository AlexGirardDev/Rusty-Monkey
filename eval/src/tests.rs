use std::collections::hash_map::Values;
use std::os::linux::raw::ino_t;
use lexer::lexer::Lexer;
use parser::ast::Program;
use parser::parser::Parser;
use crate::evaluator::eval;
use crate::object::Object;

#[test]
fn test_eval_int_exp() {
    struct Test {
        input: String,
        expected_output: Object,
    }
    impl Test {
        pub fn new(input: &str, output: Object) -> Self {
            Test { input: String::from(input), expected_output: output }
        }
    }

    let tests: Vec<Test> = vec![
        Test::new("5", Object::Int(5)),
        Test::new("10", Object::Int(10)),
    ];
    for test in tests {
        let obj = test_eval(test.input);
        assert_eq!(obj, test.expected_output);
    }
}

fn test_eval(input: String) -> Object {
    let program = get_program(input);
    return match eval(program) {
        Some(o) => o,
        None => panic!("wowee")
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
