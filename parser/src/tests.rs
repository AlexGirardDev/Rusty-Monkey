#![allow(dead_code)]
use crate::ast::{Expression, Program, Statement};
use crate::parser::Parser;
use lexer::lexer::Lexer;
use lexer::token::Token;

#[test]
fn test_array_index_parse() {
    test_single_expression(
        "myarray[10]",
        Expression::IndexExpression(
            Expression::new("myarray").into(),
            Expression::new(10).into(),
        ),
    );
}

#[test]
fn test_map_parse() {
    test_single_expression(
        r#"{"foo":3,"bar":5}"#,
        Expression::Map(vec![
            (
                Expression::StringLiteral("foo".to_string()),
                Expression::IntLiteral(3),
            ),
            (
                Expression::StringLiteral("bar".to_string()),
                Expression::IntLiteral(5),
            ),
        ]),
    );
    test_single_expression(
        r#"{}"#,
        Expression::Map(Vec::new()),
    );


    test_single_expression(
                "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}",
        Expression::Map(vec![
            (Expression::StringLiteral("one".to_string()), Expression::InfixExpression(Token::Plus,Expression::IntLiteral(0).into(),Expression::IntLiteral(1).into())),
            (Expression::StringLiteral("two".to_string()), Expression::InfixExpression(Token::Dash,Expression::IntLiteral(10).into(),Expression::IntLiteral(8).into())),
            (Expression::StringLiteral("three".to_string()), Expression::InfixExpression(Token::ForwardSlash,Expression::IntLiteral(15).into(),Expression::IntLiteral(5).into())),
        ]),
    );
}

#[test]
fn test_array_literal_parse() {
    test_single_expression("[1,2]", Expression::Arrary(vec![1.into(), 2.into()]));
    test_single_expression(
        "[1,\"foo\"]",
        Expression::Arrary(vec![1.into(), Expression::StringLiteral("foo".into())]),
    );
    test_single_expression(
        "[1,\"foo\",2+3,3*4]",
        Expression::Arrary(vec![
            1.into(),
            Expression::StringLiteral("foo".into()),
            Expression::InfixExpression(Token::Plus, Box::new(2.into()), Box::new(3.into())),
            Expression::InfixExpression(Token::Asterisk, Box::new(3.into()), Box::new(4.into())),
        ]),
    );
}

#[test]
fn test_string_literal_parse() {
    test_single_expression("\"foobar\"", Expression::StringLiteral("foobar".into()));
    test_single_expression("\"foo bar\"", Expression::StringLiteral("foo bar".into()));
}

#[test]
fn test_fn_call_expressions() {
    let input = "add(1, 2 * 3, 4+5,\"test\")";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    p.check_and_print_errors(&program);
    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];
    if let Statement::ExpressionStatement(Expression::CallExpression(id, params)) = statement {
        if let Expression::Identifier(ident) = id.as_ref() {
            assert_eq!(ident, "add");
        } else {
            panic!("Expected if expression statement, got {}", statement);
        }

        assert_eq!(params.len(), 4);
        test_int_exp(&params[0], 1);
        test_infix_exp(&params[1], Token::Asterisk, 2, 3);
        test_infix_exp(&params[2], Token::Plus, 4, 5);
        test_string_literal_exp(&params[3], "test");
    }
}

#[test]
fn test_fn_expressions() {
    let input = "fn(x,y) { x + y; }";
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    p.check_and_print_errors(&program);
    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];
    if let Statement::ExpressionStatement(i) = statement {
        if let Expression::FnExpression(params, block) = i {
            assert_eq!(params.len(), 2);
            assert_eq!(String::from("x"), params[0]);
            assert_eq!(String::from("y"), params[1]);
            if let Statement::ExpressionStatement(e) = &block.statements[0] {
                test_infix_exp(e, Token::Plus, "x", "y");
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
    p.check_and_print_errors(&program);

    assert_eq!(program.statements.len(), 1);
    let statement = &program.statements[0];

    if let Statement::ExpressionStatement(i) = statement {
        if let Expression::IfExpression(condition, if_exp, else_exp) = i {
            test_infix_exp(condition, Token::LessThan, "x", "y");
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
    p.check_and_print_errors(&program);

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
            input: impl Into<String>,
            left_value: impl Into<Token>,
            operator: impl Into<Token>,
            right_value: impl Into<Token>,
        ) -> InfixTest {
            InfixTest {
                input: input.into(),
                operator: operator.into(),
                left_value: left_value.into(),
                right_value: right_value.into(),
            }
        }
    }

    let tests: Vec<InfixTest> = vec![
        InfixTest::new("5 - 5", 5, Token::Dash, 5),
        InfixTest::new("5 * 5", 5, Token::Asterisk, 5),
        InfixTest::new("5 / 5", 5, Token::ForwardSlash, 5),
        InfixTest::new("5 > 5", 5, Token::GreaterThan, 5),
        InfixTest::new("5 >= 5", 5, Token::GreaterThanEqual, 5),
        InfixTest::new("5 < 5", 5, Token::LessThan, 5),
        InfixTest::new("5 <= 5", 5, Token::LessThanEqual, 5),
        InfixTest::new("5 == 5", 5, Token::Equal, 5),
        InfixTest::new("5 != 5", 5, Token::NotEqual, 5),
        InfixTest::new("true == true", true, Token::Equal, true),
        InfixTest::new("true != false", true, Token::NotEqual, false),
        InfixTest::new("false == false", false, Token::Equal, false),
        InfixTest::new(
            "\"foo\" == \"bar\"",
            Token::String("foo".to_owned()),
            Token::Equal,
            Token::String("bar".to_owned()),
        ),
        InfixTest::new(
            "\"foo\" + \"bar\"",
            Token::String("foo".to_owned()),
            Token::Plus,
            Token::String("bar".to_owned()),
        ),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        p.check_and_print_errors(&program);
        assert_eq!(program.statements.len(), 1);
        let statement = &program.statements[0];
        if let Statement::ExpressionStatement(exp) = statement {
            test_infix_exp(exp, t.operator, t.left_value, t.right_value);
        }
    }
}

fn test_infix_exp(
    exp: &Expression,
    operator: impl Into<Token>,
    left_value: impl Into<Token>,
    right_value: impl Into<Token>,
) {
    if let Expression::InfixExpression(token, l_exp, r_exp) = exp {
        assert_eq!(token.clone(), operator.into());
        assert_eq!(l_exp.as_ref(), &token_to_expression(left_value.into()));
        assert_eq!(r_exp.as_ref(), &token_to_expression(right_value.into()));
    } else {
        panic!("Expected infix expression, got {}", exp);
    }
}
fn token_to_expression(token: Token) -> Expression {
    match &token {
        Token::Int(i) => Expression::IntLiteral(*i),
        Token::Bool(b) => Expression::Bool(*b),
        Token::Ident(s) => Expression::Identifier(s.clone()),
        Token::String(s) => Expression::StringLiteral(s.clone()),
        t => panic!("could not convert {} to an expression", t),
    }
}

fn test_int_exp(exp: &Expression, value: i64) {
    if let Expression::IntLiteral(i) = &exp {
        assert_eq!(*i, value);
    } else {
        panic!("Expected int literal exp, got {}", exp);
    }
}

fn test_string_literal_exp(exp: &Expression, value: &str) {
    if let Expression::StringLiteral(s) = &exp {
        assert_eq!(s, value);
    } else {
        panic!("Expected string literal exp, got {}", exp);
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
        pub fn new(input: &str, prefix: Token, value: impl Into<Token>) -> PrefixTest {
            PrefixTest {
                input: String::from(input),
                prefix,
                value: value.into(),
            }
        }
    }
    let tests: Vec<PrefixTest> = vec![
        PrefixTest::new("!5", Token::Bang, 5),
        PrefixTest::new("-15", Token::Dash, 15),
        PrefixTest::new("!true", Token::Bang, true),
        PrefixTest::new("!false", Token::Bang, false),
    ];

    for t in tests {
        let mut p = Parser::new(Lexer::new(&t.input));
        let program: Program = p.parse_program();
        p.check_and_print_errors(&program);
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(Expression::PrefixExpression(token, exp)) =
            &program.statements[0]
        {
            assert_eq!(token, &t.prefix);
            assert_eq!(exp.as_ref(), &token_to_expression(t.value));
        }
    }
}

#[test]
fn test_identifier() {
    let input = "foobar";
    let mut p = Parser::new(Lexer::new(input));
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

    let mut p = Parser::new(Lexer::new(input));

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

    let mut p = Parser::new(Lexer::new(input));

    let program = p.parse_program();
    p.check_and_print_errors(&program);
    let expected_statements: Vec<String> =
        vec![String::from("x"), String::from("y"), String::from("foobar")];
    assert_eq!(program.statements.len(), expected_statements.len());
    for statement in &program.statements {
        println!("{}", statement);
    }
    for (i, value_name) in expected_statements.iter().enumerate() {
        let statement = &program.statements[i];
        let test_result = test_let_statement(statement, value_name);
        if test_result.is_err() {
            panic!("{}", test_result.unwrap_err());
        }
    }
}

#[test]
fn test_return_statements() {
    let input = "return (5)\
    return 10;\
    return 838383;
    ";
    let expected_count = 3;

    let statements = get_statements(input);
    assert_eq!(statements.len(), expected_count);
    for i in 0..expected_count {
        let statement = &statements[i];
        let test_result = test_return_statement(statement);
        if test_result.is_err() {
            panic!("{}", test_result.unwrap_err());
        }
    }
}

fn test_let_statement(statement: &Statement, name: &str) -> Result<(), String> {
    match statement {
        Statement::Let(x, ..) => {
            assert_eq!(x, name); // test to make sure name is correct
        }
        _ => {
            panic!("Expected let statement, got {:?}", statement);
        }
    };

    Ok(())
}

fn test_single_expression(input: &str, exp: Expression) {
    let statements = get_statements(input);
    dbg!(&statements);
    assert_eq!(statements.len(), 1,);
    let statement = &statements[0];
    if let Statement::ExpressionStatement(e) = statement {
        assert_eq!(*e, exp);
    } else {
        panic!("expected expression but got {statement}");
    }
}
fn get_statements(input: &str) -> Vec<Statement> {
    let mut p = Parser::new(Lexer::new(input));
    let program: Program = p.parse_program();
    p.check_and_print_errors(&program);
    program.statements
}

fn test_return_statement(statement: &Statement) -> Result<(), String> {
    match statement {
        Statement::Return(..) => {}
        _ => {
            panic!("Expected return statement, got {:?}", statement);
        }
    };

    Ok(())
}

struct Test {
    input: String,
    expected_output: String,
}

impl Test {
    fn new(input: impl Into<String>, output: impl Into<String>) -> Self {
        Test {
            input: input.into(),
            expected_output: output.into(),
        }
    }
}

fn test_program(tests: Vec<Test>) {
    for test in tests {
        let mut p = Parser::new(Lexer::new(&test.input));
        let program: Program = p.parse_program();
        p.check_and_print_errors(&program);
        assert_eq!(program.to_string(), test.expected_output, "Actual,Expected")
    }
}
