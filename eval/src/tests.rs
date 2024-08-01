use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::Environment;
use crate::eval::eval;
use crate::eval_error::EvalError;
use crate::node::Node;
use crate::object::{HashKey, HashPair, Object};
use colored::Colorize;
use lexer::lexer::Lexer;
use lexer::token::Token;
use parser::ast::{Expression, Statement};
use parser::parser::Parser;
use parser::program::Program;

#[test]
fn test_hash_index_expression() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new(r#"{"foo":5}["foo"]"#, 5),
        SingleValueTest::new("{\"foo\": 5}[\"foo\"]", 5),
        SingleValueTest::new("{\"foo\": 5}[\"bar\"]", Object::Null),
        SingleValueTest::new("let key = \"foo\"; {\"foo\": 5}[key]", 5),
        SingleValueTest::new("{}[\"foo\"]", Object::Null),
        SingleValueTest::new("{5: 5}[5]", 5),
        SingleValueTest::new("{true: 5}[true]", 5),
        SingleValueTest::new("{false: 5}[false]", 5),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_hash_literal() {
    let input = r#"
let two = "two";
{

  "one": 10 - 9,
  two: 1 + 1,
  "thr" + "ee": 6 / 2,
  4: 4,
  true: 5,
  false: 6
}"#;

    let test = test_eval(input).unwrap();
    let mut hash: HashMap<HashKey, HashPair> = HashMap::new();
    add_hash_item(&mut hash, Object::String(String::from("one")), 1);
    add_hash_item(&mut hash, Object::String(String::from("two")), 2);
    add_hash_item(&mut hash, Object::String(String::from("three")), 3);
    add_hash_item(&mut hash, 4, 4);
    add_hash_item(&mut hash, true, 5);
    add_hash_item(&mut hash, false, 6);
    assert_eq!(test, Object::Hash(hash).into());
}
fn add_hash_item(
    hash: &mut HashMap<HashKey, HashPair>,
    key: impl Into<Object>,
    value: impl Into<Object>,
) {
    let key: Object = key.into();
    let value: Object = value.into();

    let hash_key: HashKey = key.hash_key().unwrap();
    hash.insert(
        hash_key,
        HashPair {
            key: key.into(),
            value: value.into(),
        },
    );
    // hash
}

#[test]
fn test_array_accessing() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("[1,2][0]", 1),
        SingleValueTest::new("[1, 2, 3][0]", 1),
        SingleValueTest::new("[1, 2, 3][1]", 2),
        SingleValueTest::new("let i = 0; [1][i]", 1),
        SingleValueTest::new("[1, 2, 3][1 + 1];", 3),
        SingleValueTest::new("let myArray = [1, 2, 3]; myArray[2];", 3),
        SingleValueTest::new(
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            6,
        ),
        SingleValueTest::new(
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];",
            2,
        ),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_complex_fns() {
    let tests: Vec<SingleValueTest> = vec![SingleValueTest::new(
        "
    let map = fn(arr, f) {\
    let iter = fn(arr, accumulated) {\
    if (len(arr) == 0) {\
      accumulated\
    } else {\
      iter(rest(arr), push(accumulated, f(first(arr))));\
    }\
    };\
    iter(arr, []);\
    };\
    let double = fn(x) { x * 2 };\
    map([1,2,3,4],double);\
    ",
        Object::Array(
            [2, 4, 6, 8]
                .iter()
                .map(|x| Rc::new(Object::Int(*x)))
                .collect(),
        ),
    )];
    SingleValueTest::test(tests);
}

#[test]
fn test_builtin_fns() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("len(\"foo\");", 3),
        SingleValueTest::new("len(\"\");", 0),
        SingleValueTest::new("len([1,2]);", 2),
        SingleValueTest::new("len([]);", 0),
        SingleValueTest::new("first([1,2]);", 1),
        SingleValueTest::new("last([1,2]);", 2),
        SingleValueTest::new("let arr = [1]; first(arr) == last(arr)", true),
        SingleValueTest::new(
            "rest([1,2,3,4])",
            Object::Array((2..=4).map(|x| Rc::new(Object::Int(x))).collect()),
        ),
        SingleValueTest::new("len(rest(rest(rest(rest([1,2,3,4])))))", 0),
        SingleValueTest::new(
            "push([1,2,3,4],5)",
            Object::Array((1..=5).map(|x| Rc::new(Object::Int(x))).collect()),
        ),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_string_operations() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("\"foo\"", "foo"),
        SingleValueTest::new("\"foo\"+\"bar\"", "foobar"),
        SingleValueTest::new("\"foo\"==\"foo\"", true),
        SingleValueTest::new("\"foo\"==\"Foo\"", false),
        SingleValueTest::new("\"foo\"!=\"bar\"", true),
    ];
    SingleValueTest::test(tests);
}

#[test]
fn test_closures() {
    let tests: Vec<SingleValueTest> = vec![SingleValueTest::new(
        "
  let newAdder = fn(x) { \
             fn(y) { x + y }; \
             }; \
             let addTwo = newAdder(2); \
             addTwo(2);",
        4,
    )];
    SingleValueTest::test(tests);
}

#[test]
fn test_function_application() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("let identity=fn(x){x;}; identity(5);", 5),
        SingleValueTest::new("let identity=fn(x){return x;}; identity(5);", 5),
        SingleValueTest::new("let double=fn(x){x*2;}; double(5);", 10),
        SingleValueTest::new("let add = fn(x, y) { x+y;}; add(5,5) + (0)", 10),
        SingleValueTest::new("let add=fn(x,y){x+y;}; add(5+5, add(5,5));", 20),
        SingleValueTest::new("fn(x){x;}(5)", 5),
    ];
    SingleValueTest::test(tests);
}
#[test]
fn test_function_object() {
    match test_eval("fn(x) {x +2}") {
        Ok(obj) => {
            if let Object::Function(ident, blk, _) = obj.as_ref() {
                assert_eq!(ident.len(), 1);
                assert_eq!(ident[0], "x");
                assert_eq!(blk.statements.len(), 1);
                match &blk.statements[0] {
                    Statement::ExpressionStatement(Expression::InfixExpression(
                        Token::Plus,
                        l,
                        r,
                    )) => {
                        if let Expression::Identifier(ident) = l.as_ref() {
                            assert_eq!(ident, "x");
                        } else {
                            panic!("Expected x ident but got {l}");
                        }
                        if let Expression::IntLiteral(i) = r.as_ref() {
                            assert_eq!(*i, 2);
                        } else {
                            panic!("Expected 2 ident but got {r}");
                        }
                    }
                    e => panic!("Expected ExpressionStatement with Infix expression, got {e}"),
                }
            } else {
                panic!("expected fn but got {obj}");
            }
        }
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_let_statements() {
    let tests: Vec<SingleValueTest> = vec![
        SingleValueTest::new("let a=5;a;", 5),
        SingleValueTest::new("let a=5*5; a;", 25),
        SingleValueTest::new("let a=5;a", 5),
        SingleValueTest::new("let a=5;(a)", 5),
        SingleValueTest::new("let a=5; let b=a; b;", 5),
        SingleValueTest::new("let a=5; let b=a; let c=a+b+5; c;", 15),
    ];
    SingleValueTest::test(tests);
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
        SingleValueTest::new("(5)", 5),
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

#[test]
fn test_error_exp() {
    let tests: Vec<ErrorTest> = vec![
        ErrorTest::new_type_missmatch("5+true", 5, true),
        ErrorTest::new("[0][1]", EvalError::IndexOutOfBounds { max: 1, index: 1 }),
        ErrorTest::new("[0][-1]", EvalError::IndexOutOfBounds { max: 1, index: -1 }),
    ];
    ErrorTest::test(tests);
}

fn test_eval(input: impl Into<String>) -> Result<Rc<Object>, EvalError> {
    let program = get_program(input.into());
    let env = Environment::new_with_builtin();
    eval(Node::Program(program), &Rc::new(RefCell::new(env)))
}

fn get_program(input: String) -> Program {
    let mut p = Parser::new(Lexer::new(&input));
    let program = p.parse_program();
    p.check_and_print_errors(&program);
    program
}

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
            match test_eval(&test.input) {
                Ok(obj) => {
                    assert_eq!(
                        *obj,
                        test.expected_output,
                        "Input: {}",
                        test.input.bright_yellow()
                    );
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}
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
            expected_output: EvalError::TypeMismatch(
                lhs.into().to_string(),
                rhs.into().to_string(),
            ),
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
            expected_output: EvalError::InvalidOperator(
                lhs.into().to_string(),
                opp.into(),
                rhs.into().to_string(),
            ),
        }
    }

    pub fn test(tests: Vec<ErrorTest>) {
        for test in tests {
            let e = test_eval(&test.input).unwrap_err();

            assert_eq!(e, test.expected_output, "Input: {}", test.input);
        }
    }
}
