#![allow(dead_code)]
use crate::ast::{Expression, Program, Statement};
use crate::parser::Parser;
use lexer::lexer::Lexer;
use lexer::token::Token;

struct Test {
    input: String,
    expected_output: String,
}

trait TraitName {
    fn new(input: &str, output: &str) -> Self;
}

impl TraitName for Test {
    fn new(input: &str, output: &str) -> Self {
        Test {
            input: String::from(input),
            expected_output: String::from(output),
        }
    }
}

fn test_program(tests: Vec<Test>) {
    for test in tests {
        let mut p = Parser::new(Lexer::new(&test.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.to_string(), test.expected_output, "Actual,Expected")
    }
}

#[test]
fn test_fn_call_expressions() {
    let input = "add(1, 2 * 3, 4+5)";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    check_and_print_errors(&p, &program);
    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];
    if let Statement::ExpressionStatement(Expression::CallExpression(id, params)) = statement {
        if let Expression::Identifier(ident) = id.as_ref() {
            assert_eq!(ident, "add");
        } else {
            panic!("Expected if expression statement, got {}", statement);
        }

        assert_eq!(params.len(), 3);
        test_int_exp(&params[0], 1);
        test_infix_exp(&params[1], Token::Asterisk, Token::Int(2), Token::Int(3));
        test_infix_exp(&params[2], Token::Plus, Token::Int(4), Token::Int(5));
        // if let Statement::ExpressionStatement(e) = &block.statements[0] {
        //     test_infix_exp(e, Token::Plus, Token::Ident(String::from("x")), Token::Ident(String::from("y")));
        // } else {
        //     panic!("Expected if ident statement with , got {}", statement);
    }
}

#[test]
fn test_fn_expressions() {
    let input = "fn(x,y) { x + y; }";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    check_and_print_errors(&p, &program);
    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];
    if let Statement::ExpressionStatement(i) = statement {
        if let Expression::FnExpression(params, block) = i {
            assert_eq!(params.len(), 2);
            assert_eq!(String::from("x"), params[0]);
            assert_eq!(String::from("y"), params[1]);
            if let Statement::ExpressionStatement(e) = &block.statements[0] {
                test_infix_exp(
                    e,
                    Token::Plus,
                    Token::Ident(String::from("x")),
                    Token::Ident(String::from("y")),
                );
            } else {
                panic!("Expected if ident statement with , got {}", statement);
            }
        } else {
            panic!("Expected if expression statement, got {}", statement);
        }
    }
}

#[test]
fn test_if_expressions() {
    let input = "if (x < y) { x }";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    check_and_print_errors(&p, &program);

    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];

    if let Statement::ExpressionStatement(i) = statement {
        if let Expression::IfExpression(condition, if_exp, else_exp) = i {
            test_infix_exp(
                condition,
                Token::LessThan,
                Token::Ident(String::from("x")),
                Token::Ident(String::from("y")),
            );
            assert_eq!(if_exp.statements.len(), 1);
            if let Statement::ExpressionStatement(Expression::Identifier(ident)) =
                &if_exp.statements[0]
            {
                assert_eq!(ident, "x");
            } else {
                panic!("Expected if ident statement with , got {}", statement);
            }
            assert_eq!(else_exp, &None);
        } else {
            panic!("Expected if expression statement, got {}", statement);
        }
    }
}

#[test]
fn test_if_else_expressions() {
    let input = "if (x < y) { x } else { 10 }";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    check_and_print_errors(&p, &program);

    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];

    if let Statement::ExpressionStatement(i) = statement {
        if let Expression::IfExpression(condition, if_exp, else_exp) = i {
            test_infix_exp(
                condition,
                Token::LessThan,
                Token::Ident(String::from("x")),
                Token::Ident(String::from("y")),
            );
            assert_eq!(if_exp.statements.len(), 1);
            if let Statement::ExpressionStatement(Expression::Identifier(ident)) =
                &if_exp.statements[0]
            {
                assert_eq!(ident, "x");
            } else {
                panic!("Expected if ident statement with , got {}", statement);
            }

            if let Statement::ExpressionStatement(Expression::IntLiteral(i)) =
                &else_exp.as_ref().unwrap().statements[0]
            {
                assert_eq!(*i, 10);
            } else {
                panic!("Expected if ident statement with , got {}", statement);
            }
        } else {
            panic!("Expected if expression statement, got {}", statement);
        }
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
        Test::new(
            "3 + 4  * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        Test::new("1 + (2 + 3) +4 ", "((1 + (2 + 3)) + 4)"),
        Test::new("(5+5)*2", "((5 + 5) * 2)"),
        Test::new("2 / (5 + 5)", "(2 / (5 + 5))"),
        Test::new("!(true == true)", "(!(true == true))"),
        Test::new("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        Test::new(
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        Test::new(
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        // Test::new("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        // Test::new("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
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
        pub fn new(
            input: &str,
            left_value: Token,
            operator: Token,
            right_value: Token,
        ) -> InfixTest {
            InfixTest {
                input: String::from(input),
                operator,
                left_value,
                right_value,
            }
        }
    }

    let tests: Vec<InfixTest> = vec![
        InfixTest::new("5 - 5", 5.into(), Token::Dash, 5.into()),
        InfixTest::new("5 * 5", 5.into(), Token::Asterisk, 5.into()),
        InfixTest::new("5 / 5", 5.into(), Token::ForwardSlash, 5.into()),
        InfixTest::new("5 > 5", 5.into(), Token::GreaterThan, 5.into()),
        InfixTest::new("5 < 5", 5.into(), Token::LessThan, 5.into()),
        InfixTest::new("5 == 5",5.into(), Token::Equal, 5.into()),
        InfixTest::new("5 != 5",5.into(), Token::NotEqual, 5.into()),
        InfixTest::new(
            "true == true",
            true.into(),
            Token::Equal,
            true.into(),
        ),
        InfixTest::new(
            "true != false",
            true.into(),
            Token::NotEqual,
            Token::Bool(false),
        ),
        InfixTest::new(
            "false == false",
            Token::Bool(false),
            Token::Equal,
            Token::Bool(false),
        ),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        let statement = &program.statements[0];
        if let Statement::ExpressionStatement(exp) = statement {
            test_infix_exp(&exp, t.operator, t.left_value, t.right_value);
        }
    }
}

fn test_infix_exp(exp: &Expression, operator: Token, left_value: Token, right_value: Token) {
    if let Expression::InfixExpression(token, l_exp, r_exp) = exp {
        assert_eq!(token.clone(), operator);
        assert_eq!( l_exp.as_ref(), &token_to_expression(left_value));
        assert_eq!(r_exp.as_ref(), &token_to_expression(right_value));
    } else {
        panic!("Expected infix expression, got {}", exp);
    }
}
fn token_to_expression(token: Token) -> Expression {
    match &token {
        Token::Int(i) => Expression::IntLiteral(i.clone()),
        Token::Bool(b) => Expression::Bool(*b),
        Token::Ident(s) => Expression::Identifier(s.clone()),
        t => panic!("could not convert {} to an expression", ),
    }
}

fn test_int_exp(exp: &Expression, value: i64) {
    if let Expression::IntLiteral(i) = &exp {
        assert_eq!(*i, value);
    } else {
        panic!("Expected int literal exp, got {}", exp);
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
            PrefixTest {
                input: String::from(input),
                prefix,
                value,
            }
        }
    }
    let tests: Vec<PrefixTest> = vec![
        PrefixTest::new("!5", Token::Bang, 5.into()),
        PrefixTest::new("-15", Token::Dash, 15.into()),
        PrefixTest::new("!true", Token::Bang, true.into()),
        PrefixTest::new("!false", Token::Bang, false.into()),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        check_and_print_errors(&p, &program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::PrefixExpression(token, exp)) =
            &program.statements[0]
        {
            assert_eq!(token, &t.prefix);
            assert_eq!( exp.as_ref(), &token_to_expression(t.value));
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
        Statement::ExpressionStatement(i) => match i {
            Expression::Identifier(i) => {
                assert_eq!(i, "foobar");
            }
            s => {
                panic!("Expected identifier statement, got {:?}", s)
            }
        },
        _ => {
            panic!("Expected expression statement, got {:?}", statement);
        }
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
        Statement::ExpressionStatement(i) => match i {
            Expression::IntLiteral(i) => {
                assert_eq!(*i, 5);
            }
            s => {
                panic!("Expected int statement, got {:?}", s)
            }
        },
        _ => {
            panic!("Expected int, got {:?}", statement);
        }
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
    let expected_statements: Vec<String> =
        vec![String::from("x"), String::from("y"), String::from("foobar")];
    assert_eq!(program.statements.len(), expected_statements.len());
    for statement in &program.statements {
        println!("{}", statement);
    }
    for (i, value_name) in expected_statements.iter().enumerate() {
        let statement = &program.statements[i];
        let test_result = test_let_statement(&statement, value_name);
        if test_result.is_err() {
            panic!("{}", test_result.unwrap_err());
        }
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
        if test_result.is_err() {
            panic!("{}", test_result.unwrap_err());
        }
    }
}

fn test_let_statement(statement: &Statement, name: &str) -> Result<(), String> {
    match statement {
        Statement::Let(x, ..) => {
            // test to make sure its a let type
            assert_eq!(x, name); // test to make sure name is correct
        }
        _ => {
            panic!("Expected let statement, got {:?}", statement);
        }
    };

    return Ok(());
}

fn test_return_statement(statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Return(..) => {}
        _ => {
            panic!("Expected return statement, got {:?}", statement);
        }
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
