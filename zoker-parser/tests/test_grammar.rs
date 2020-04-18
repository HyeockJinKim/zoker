use zoker_parser::zok;

#[test]
fn test_if_statement_parser() {
    use zok::ExpressionParser as parser;

    // assert!(parser::new().parse("if a < 2 { a = 1; } else { a = 0; }").is_ok());
    assert!(parser::new().parse("if a < 2 { 1 } else { 0 }").is_ok());
    assert!(parser::new().parse("b = if a < 2 { 1 } else { 0 }").is_ok());
}

#[test]
fn test_assign_expression_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("a = 1").is_ok());
    assert!(parser::new().parse("a = b").is_ok());
    assert!(parser::new().parse("2 = 11 - 3").is_err());
    assert!(parser::new().parse("a = 11 - 3").is_ok());
    assert!(parser::new().parse("a = 3%2").is_ok());
    assert!(parser::new().parse("a = b = 3").is_ok());
    assert!(parser::new().parse("a = 3 = 2").is_err());
}

#[test]
fn test_arithmetic_expression1_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("11+3").is_ok());
    assert!(parser::new().parse("11+3**2").is_ok());
    assert!(parser::new().parse("2**2-3").is_ok());
    assert!(parser::new().parse("3%2").is_ok());
    assert!(parser::new().parse("2+1").is_ok());
    assert!(parser::new().parse("2-1").is_ok());
}

#[test]
fn test_arithmetic_expression2_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("11*3").is_ok());
    assert!(parser::new().parse("2/3").is_ok());
    assert!(parser::new().parse("3%2").is_ok());
}

#[test]
fn test_power_expression_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("11**3").is_ok());
    assert!(parser::new().parse("2**(3+1)").is_ok());
    assert!(parser::new().parse("(22+1)**2").is_ok());
    assert!(parser::new().parse("2**1++").is_err());
    assert!(parser::new().parse("2**-1").is_err());
}

#[test]
fn test_unary_expression_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("+22").is_ok());
    assert!(parser::new().parse("-1").is_ok());
    assert!(parser::new().parse("++22").is_ok());
    assert!(parser::new().parse("--22").is_ok());
    assert!(parser::new().parse("22++1").is_err());
}

#[test]
fn test_value_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("a").is_ok());
    assert!(parser::new().parse("123").is_ok());
    assert!(parser::new().parse("(12)").is_ok());
    assert!(parser::new().parse("(((123)))").is_ok());
    assert!(parser::new().parse("(12321+121*5+1)").is_ok());
    assert!(parser::new().parse("(((22))").is_err());
}

#[test]
fn test_terminal_parser() {
    use zok::ExpressionParser as parser;

    assert!(parser::new().parse("121").is_ok());
    assert!(parser::new().parse("a").is_ok());
    assert!(parser::new().parse("a_1").is_ok());
    assert!(parser::new().parse("A9").is_ok());
}
