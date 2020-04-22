use num_bigint::BigUint;
use std::str::FromStr;
use zoker_parser::{ast, error, parser};

fn check_bin_expr_in_expr(
    expression: ast::Expression,
) -> Result<(Box<ast::Expression>, ast::Operator, Box<ast::Expression>), error::ParseError> {
    match expression.node {
        ast::ExpressionType::BinaryExpression {
            left: l,
            operator: op,
            right: r,
        } => Ok((l, op, r)),
        _ => Err(error::ParseError {
            location: expression.location,
            error: error::ParseErrorType::InvalidToken,
        }),
    }
}

fn check_number_in_expression(expression: ast::Expression) -> Result<BigUint, error::ParseError> {
    match expression.node {
        ast::ExpressionType::Number { value: v } => Ok(v),
        _ => Err(error::ParseError {
            location: expression.location,
            error: error::ParseErrorType::InvalidToken,
        }),
    }
}

#[test]
fn test_arithmetic_expression_ast1() {
    let expr = parser::parse_expression("22 * 44 + 66").unwrap();

    // Obtain an AST of "(22 * 44) + 66".
    let bin_expr = check_bin_expr_in_expr(expr);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Test l:expr, op:*, r:66
    assert_eq!(bin_expr.1, ast::Operator::Add);

    // Check 66
    let r = check_number_in_expression(*bin_expr.2);
    assert!(r.is_ok());

    assert_eq!(r.unwrap(), BigUint::from_str("66").unwrap());

    let bin_expr = check_bin_expr_in_expr(*bin_expr.0);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Check 22
    let l = check_number_in_expression(*bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), BigUint::from_str("22").unwrap());

    assert_eq!(bin_expr.1, ast::Operator::Mul);

    // Check 44
    let r = check_number_in_expression(*bin_expr.2);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), BigUint::from_str("44").unwrap());
}

#[test]
fn test_arithmetic_expression_ast2() {
    let expr = parser::parse_expression("66 + 22 * 44").unwrap();

    // Obtain an AST of "66 + (22 * 44)".
    let bin_expr = check_bin_expr_in_expr(expr);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Test l:66, op:*, r:expr
    assert_eq!(bin_expr.1, ast::Operator::Add);

    // Check 66
    let l = check_number_in_expression(*bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), BigUint::from_str("66").unwrap());

    let bin_expr = check_bin_expr_in_expr(*bin_expr.2);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Check 22
    let l = check_number_in_expression(*bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), BigUint::from_str("22").unwrap());

    assert_eq!(bin_expr.1, ast::Operator::Mul);

    // Check 44
    let r = check_number_in_expression(*bin_expr.2);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), BigUint::from_str("44").unwrap());
}

#[test]
fn test_lexer() {
    let source = "uint i = 3 ; ";
    let res = parser::parse_program(source);
    assert!(res.is_ok());
}

#[test]
fn test_bigint() {
    let bigint = parser::parse_expression("1_021_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000_000");
    let bigint = check_number_in_expression(bigint.unwrap());
    assert!(bigint.is_ok());
    assert_eq!(bigint.unwrap(), BigUint::from_str("1021000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap())
}
