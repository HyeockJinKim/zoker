#[cfg(test)]
#[macro_use]
extern crate assert_matches;

use zoker_parser::{ast, error, zoker};

fn check_bin_expr_in_expr(
    expression: ast::ExpressionType,
) -> Result<
    (
        ast::ExpressionType,
        ast::BinaryOperator,
        ast::ExpressionType,
    ),
    error::ParseError,
> {
    match expression {
        ast::ExpressionType::BinaryExpression {
            left: l,
            operator: op,
            right: r,
        } => Ok((l.node, op, r.node)),
        _ => Err(error::ParseError {
            error: error::ParseErrorType::UnrecognizedToken,
        }),
    }
}

fn check_number_in_expression(expression: ast::ExpressionType) -> Result<i32, error::ParseError> {
    match expression {
        ast::ExpressionType::Number { value: v } => Ok(v),
        _ => Err(error::ParseError {
            error: error::ParseErrorType::UnrecognizedToken,
        }),
    }
}

#[test]
fn test_arithmetic_expression_ast1() {
    use zoker::ArithmeticExpression1Parser as parser;
    let expr = parser::new().parse("22 * 44 + 66").unwrap();

    // Obtain an AST of "(22 * 44) + 66".
    let bin_expr = check_bin_expr_in_expr(expr.node);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Test l:expr, op:*, r:66
    assert_matches!(bin_expr.1, ast::BinaryOperator::Add);

    // Check 66
    let l = check_number_in_expression(bin_expr.2);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), 66);

    let bin_expr = check_bin_expr_in_expr(bin_expr.0);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Check 22
    let l = check_number_in_expression(bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), 22);

    assert_matches!(bin_expr.1, ast::BinaryOperator::Mul);

    // Check 44
    let r = check_number_in_expression(bin_expr.2);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 44);
}

#[test]
fn test_arithmetic_expression_ast2() {
    use zoker::ArithmeticExpression1Parser as parser;
    let expr = parser::new().parse("66 + 22 * 44").unwrap();

    // Obtain an AST of "66 + (22 * 44)".
    let bin_expr = check_bin_expr_in_expr(expr.node);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Test l:66, op:*, r:expr
    assert_matches!(bin_expr.1, ast::BinaryOperator::Add);

    // Check 66
    let l = check_number_in_expression(bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), 66);

    let bin_expr = check_bin_expr_in_expr(bin_expr.2);
    assert!(bin_expr.is_ok());
    let bin_expr = bin_expr.unwrap();

    // Check 22
    let l = check_number_in_expression(bin_expr.0);
    assert!(l.is_ok());
    assert_eq!(l.unwrap(), 22);

    assert_matches!(bin_expr.1, ast::BinaryOperator::Mul);

    // Check 44
    let r = check_number_in_expression(bin_expr.2);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 44);
}
