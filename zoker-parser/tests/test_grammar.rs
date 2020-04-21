use zoker_parser::parser;

#[test]
fn test_if_statement_parser() {
    assert!(parser::parse_expression("if a < 2 { 1 } else { 0 }").is_ok());
    assert!(parser::parse_expression("b = if a < 2 { 1 } else { 0 }").is_ok());
}

#[test]
fn test_assign_expression_parser() {
    assert!(parser::parse_expression("a = 1").is_ok());
    assert!(parser::parse_expression("a = b").is_ok());
    assert!(parser::parse_expression("2 = 11 - 3").is_err());
    assert!(parser::parse_expression("a = 11 - 3").is_ok());
    assert!(parser::parse_expression("a = 3%2").is_ok());
    assert!(parser::parse_expression("a = b = 3").is_ok());
    assert!(parser::parse_expression("a = 3 = 2").is_err());
}

#[test]
fn test_arithmetic_expression1_parser() {
    assert!(parser::parse_expression("11+3").is_ok());
    assert!(parser::parse_expression("11+3**2").is_ok());
    assert!(parser::parse_expression("2**2-3").is_ok());
    assert!(parser::parse_expression("3%2").is_ok());
    assert!(parser::parse_expression("2+1").is_ok());
    assert!(parser::parse_expression("2-1").is_ok());
}

#[test]
fn test_arithmetic_expression2_parser() {
    assert!(parser::parse_expression("11*3").is_ok());
    assert!(parser::parse_expression("2/3").is_ok());
    assert!(parser::parse_expression("3%2").is_ok());
}

#[test]
fn test_power_expression_parser() {
    assert!(parser::parse_expression("11**3").is_ok());
    assert!(parser::parse_expression("2**(3+1)").is_ok());
    assert!(parser::parse_expression("(22+1)**2").is_ok());
    assert!(parser::parse_expression("2**1++").is_err());
    assert!(parser::parse_expression("2**-1").is_err());
}

#[test]
fn test_unary_expression_parser() {
    assert!(parser::parse_expression("+22").is_ok());
    assert!(parser::parse_expression("-1").is_ok());
    assert!(parser::parse_expression("++22").is_ok());
    assert!(parser::parse_expression("--22").is_ok());
    assert!(parser::parse_expression("22++1").is_err());
}

#[test]
fn test_value_parser() {
    assert!(parser::parse_expression("a").is_ok());
    assert!(parser::parse_expression("123").is_ok());
    assert!(parser::parse_expression("(12)").is_ok());
    assert!(parser::parse_expression("(((123)))").is_ok());
    assert!(parser::parse_expression("(12321+121*5+1)").is_ok());
    assert!(parser::parse_expression("(((22))").is_err());
}

#[test]
fn test_terminal_parser() {
    assert!(parser::parse_expression("121").is_ok());
    assert!(parser::parse_expression("a").is_ok());
    assert!(parser::parse_expression("a_1").is_ok());
    assert!(parser::parse_expression("A9").is_ok());
}
