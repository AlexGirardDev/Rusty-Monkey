use lexer::lexer::Lexer;
use lexer::token::Token;
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
    struct Test<'a> {
        input: &'a str,
        operator: Token,
        left_value: i32,
        right_value: i32,
    }
    let tests: Vec<Test> = vec![
        Test { input: "5 + 5", operator: Token::Plus, left_value: 5, right_value: 5 },
        Test { input: "5 - 5", operator: Token::Dash, left_value: 5, right_value: 5 },
        Test { input: "5 * 5", operator: Token::Asterisk, left_value: 5, right_value: 5 },
        Test { input: "5 / 5", operator: Token::ForwardSlash, left_value: 5, right_value: 5 },
        Test { input: "5 > 5", operator: Token::GreaterThan, left_value: 5, right_value: 5 },
        Test { input: "5 < 5", operator: Token::LessThan, left_value: 5, right_value: 5 },
        Test { input: "5 == 5", operator: Token::Equal, left_value: 5, right_value: 5 },
        Test { input: "5 != 5", operator: Token::NotEqual, left_value: 5, right_value: 5 },
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::InfixExpression(token, l_exp, r_exp)) = &program.statements[0] {
            assert_eq!(token, &t.operator);
            if let Expression::IntLiteral(i) = l_exp.as_ref() {
                assert_eq!(i, &t.left_value, "left value check");
            }
            if let Expression::IntLiteral(i) = r_exp.as_ref() {
                assert_eq!(i, &t.right_value, "right value check");
            }
        }
    }
}

struct PrefixTest {
    input: String,
    operator: Token,
    number: i32,

}

#[test]
fn test_prefix_expressions() {
    let tests: Vec<PrefixTest> = vec![
        PrefixTest { input: String::from("!5"), operator: Token::Bang, number: 5 },
        PrefixTest { input: String::from("-15"), operator: Token::Dash, number: 15 },
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::PrefixExpression(token, exp)) = &program.statements[0] {
            assert_eq!(token, &t.operator);
            if let Expression::IntLiteral(i) = exp.as_ref() {
                assert_eq!(i, &t.number);
            }
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
