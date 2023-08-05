use lexer::lexer::Lexer;
use lexer::token::Token;
use crate::ast::{Expression, Program, Statement};
use crate::parse_error::TokenType::Dash;
use crate::parser::Parser;

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
        println!("Parse errors:");
        println!("=============");
        for parse_error in &parser.parse_errors {
            println!("{}", parse_error);
        }

        println!("Statements:");
        println!("===========");
        println!("{}", program);
        panic!("ruh roh, program had errors")
    }
}
