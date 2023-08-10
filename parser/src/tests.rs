use lexer::lexer::Lexer;
use lexer::token::Token;
use lexer::token::Token::{Bool, Int};
use crate::ast::{Expression, Program, Statement};
use crate::parse_error::TokenType::Dash;
use crate::parser::Parser;

struct Test {
    input: String,
    expected_output: String,
}

impl Test {
    pub fn new(input: &str, output: &str) -> Self {
        Test { input: String::from(input), expected_output: String::from(output) }
    }
}

fn test_program(tests: Vec<Test>) {
    for test in tests {
        let mut p = Parser::new(Lexer::new(&test.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.to_string(), test.expected_output)
    }
}

#[test]
fn test_bool_expressions() {
    let tests: Vec<Test> = vec![
        Test::new("true", "true"),
        Test::new("false", "false"),
        Test::new("3 > 5 == false", "((3 > 5) == false)"),
        Test::new("3 < 5 == true", "((3 < 5) == true)"),
    ];
    test_program(tests);
}

#[test]
fn test_op_prec_expressions() {
    let tests: Vec<Test> = vec![
        Test::new("-5 * b", "((-5) * b)"),
        Test::new("!-5", "(!(-5))"),
        Test::new("!-5 * 4", "((!(-5)) * 4)"),
        Test::new("a + b + c", "((a + b) + c)"),
        Test::new("a + b - c", "((a + b) - c)"),
        Test::new("a * b * c", "((a * b) * c)"),
        Test::new("a * b / c", "((a * b) / c)"),
        Test::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        Test::new("3+4; -5 * 5", "(3 + 4)((-5) * 5)"),
        Test::new("5>4==3<4", "((5 > 4) == (3 < 4))"),
        Test::new("3 + 4  * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
    ];
    test_program(tests);
}

#[test]
fn test_infix_expressions() {
    struct InfixTest {
        input: String,
        operator: Token,
        left_value: Token,
        right_value: Token,
    }
    impl InfixTest {
        pub fn new(input: &str, left_value: Token, operator: Token, right_value: Token) -> InfixTest {
            InfixTest { input: String::from(input), operator, left_value, right_value }
        }
    }

    let tests: Vec<InfixTest> = vec![
        InfixTest::new("5 - 5", Int(5), Token::Dash, Int(5)),
        InfixTest::new("5 * 5", Int(5), Token::Asterisk, Int(5)),
        InfixTest::new("5 / 5", Int(5), Token::ForwardSlash, Int(5)),
        InfixTest::new("5 > 5", Int(5), Token::GreaterThan, Int(5)),
        InfixTest::new("5 < 5", Int(5), Token::LessThan, Int(5)),
        InfixTest::new("5 == 5", Int(5), Token::Equal, Int(5)),
        InfixTest::new("5 != 5", Int(5), Token::NotEqual, Int(5)),
        InfixTest::new("true == true", Bool(true), Token::Equal, Bool(true)),
        InfixTest::new("true != false", Bool(true), Token::NotEqual, Bool(false)),
        InfixTest::new("false == false", Bool(false), Token::Equal, Bool(false)),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::InfixExpression(token, l_exp, r_exp)) = &program.statements[0] {
            assert_eq!(token, &t.operator);
            assert_eq!(l_exp.as_ref(), &Expression::from(&t.left_value).unwrap(), "left value check");
            assert_eq!(r_exp.as_ref(), &Expression::from(&t.right_value).unwrap(), "left value check");
        }
    }
}

#[test]
fn test_prefix_expressions() {
    struct PrefixTest {
        input: String,
        prefix: Token,
        value: Token,
    }
    impl PrefixTest {
        pub fn new(input: &str, prefix: Token, value: Token) -> PrefixTest {
            PrefixTest { input: String::from(input), prefix, value }
        }
    }
    let tests: Vec<PrefixTest> = vec![
        PrefixTest::new("!5", Token::Bang, Int(5)),
        PrefixTest::new("-15", Token::Dash, Int(15)),
        PrefixTest::new("!true", Token::Bang, Bool(true)),
        PrefixTest::new("!false", Token::Bang, Bool(false)),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::PrefixExpression(token, exp)) = &program.statements[0] {

            assert_eq!(token, &t.prefix);
            assert_eq!(exp.as_ref(), &Expression::from(&t.value).unwrap(), "left value check");
        }
    }
}

#[test]
fn test_identifier() {
    let input = "foobar";

    let mut p = Parser::new(Lexer::new(&input));

    let program: Program = p.parse_program();

    assert_eq!(program.statements.len(), 1);

    for statement in &program.statements {
        println!("{}", statement);
    }

    let statement = &program.statements[0];
    match statement {
        Statement::ExpressionStatement(i) => {
            match i {
                Expression::Identifier(i) => {
                    assert_eq!(i, "foobar");
                }
                s => { panic!("Expected identifier statement, got {:?}", s) }
            }
        }
        _ => { panic!("Expected expression statement, got {:?}", statement); }
    }
}

#[test]
fn test_int_literal() {
    let input = "5;";

    let mut p = Parser::new(Lexer::new(&input));

    let program: Program = p.parse_program();

    assert_eq!(program.statements.len(), 1);

    for statement in &program.statements {
        println!("{}", statement);
    }

    let statement = &program.statements[0];
    match statement {
        Statement::ExpressionStatement(i) => {
            match i {
                Expression::IntLiteral(i) => {
                    assert_eq!(*i, 5);
                }
                s => { panic!("Expected int statement, got {:?}", s) }
            }
        }
        _ => { panic!("Expected int, got {:?}", statement); }
    }
}

#[test]
fn test_let_statements() {
    let input = "let x = 5;\
    let y = 10;\
    let foobar = 838383;
    ";

    let mut p = Parser::new(Lexer::new(&input));

    let program = p.parse_program();
    check_and_print_errors(&p, &program);
    let expected_statements: Vec<String> = vec![
        String::from("x"),
        String::from("y"),
        String::from("foobar"),
    ];
    assert_eq!(program.statements.len(), expected_statements.len());
    for statement in &program.statements {
        println!("{}", statement);
    }
    for (i, value_name) in expected_statements.iter().enumerate() {
        let statement = &program.statements[i];
        let test_result = test_let_statement(&statement, value_name);
        if test_result.is_err() { panic!("{}", test_result.unwrap_err()); }
    }
}

#[test]
fn test_return_statements() {
    let input = "return 5;\
    return 10;\
    return 838383;
    ";
    let expected_count = 3;

    let mut p = Parser::new(Lexer::new(&input));

    let program = p.parse_program();
    check_and_print_errors(&p, &program);
    assert_eq!(program.statements.len(), expected_count);
    for i in 0..expected_count {
        let statement = &program.statements[i];
        let test_result = test_return_statement(&statement);
        if test_result.is_err() { panic!("{}", test_result.unwrap_err()); }
    }
}

fn test_let_statement(statement: &Statement, name: &str) -> Result<(), String> {
    match statement {
        Statement::Let(x, ..) => {// test to make sure its a let type
            assert_eq!(x, name);// test to make sure name is correct
        }
        _ => { panic!("Expected let statement, got {:?}", statement); }
    };

    return Ok(());
}

fn test_return_statement(statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Return(..) => {}
        _ => { panic!("Expected return statement, got {:?}", statement); }
    };

    return Ok(());
}

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
