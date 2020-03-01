use zoker_parser::zoker;

#[test]
fn test_program_parser() {
    use zoker::ProgramParser as parser;

    assert!(parser::new().parse("120").is_ok());
    assert!(parser::new().parse("(22)").is_ok());
    assert!(parser::new().parse("(((22)))").is_ok());
    assert!(parser::new().parse("(((22))").is_err());
}

#[test]
fn test_arithmetic_expression1_parser() {
    use zoker::ArithmeticExpression1Parser as parser;

    assert!(parser::new().parse("11+3").is_ok());
    assert!(parser::new().parse("11+3**2").is_ok());
    assert!(parser::new().parse("2**2-3").is_ok());
    assert!(parser::new().parse("3%2").is_ok());
    assert!(parser::new().parse("2+1").is_ok());
    assert!(parser::new().parse("2-1").is_ok());
}

#[test]
fn test_arithmetic_expression2_parser() {
    use zoker::ArithmeticExpression2Parser as parser;

    assert!(parser::new().parse("11*3").is_ok());
    assert!(parser::new().parse("2/3").is_ok());
    assert!(parser::new().parse("3%2").is_ok());
    assert!(parser::new().parse("2+1").is_err());
    assert!(parser::new().parse("2-1").is_err());
}

#[test]
fn test_power_expression_parser() {
    use zoker::PowerExpressionParser as parser;

    assert!(parser::new().parse("11**3").is_ok());
    assert!(parser::new().parse("2**(3+1)").is_ok());
    assert!(parser::new().parse("(22+1)**2").is_ok());
    assert!(parser::new().parse("2**1++").is_err());
    assert!(parser::new().parse("2**-1").is_err());
}

#[test]
fn test_unary_expression_parser() {
    use zoker::UnaryExpressionParser as parser;

    assert!(parser::new().parse("+22").is_ok());
    assert!(parser::new().parse("-1").is_ok());
    assert!(parser::new().parse("++22").is_ok());
    assert!(parser::new().parse("--22").is_ok());
    assert!(parser::new().parse("22++1").is_err());
}

#[test]
fn test_value_parser() {
    use zoker::ValueParser as parser;

    assert!(parser::new().parse("123").is_ok());
    assert!(parser::new().parse("12+1").is_err());
    assert!(parser::new().parse("(12)").is_ok());
    assert!(parser::new().parse("(((123)))").is_ok());
    assert!(parser::new().parse("(12321+121*5+1)").is_ok());
    assert!(parser::new().parse("(((22))").is_err());
}

#[test]
fn test_terminal_parser() {
    use zoker::TerminalParser as parser;

    assert!(parser::new().parse("121").is_ok());
    assert!(parser::new().parse("121+1").is_err());
    assert!(parser::new().parse("(11)").is_err());
}
