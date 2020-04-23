use zoker_parser::ast::ExpressionType::{BinaryExpression, IfExpression};
use zoker_parser::location::Location;
use zoker_parser::parser;

#[test]
fn test_location_new() {
    let loc = Location::new(3, 7);
    assert_eq!(loc.row(), 3);
    assert_eq!(loc.column(), 7);
}

#[test]
fn test_location_reset() {
    let mut loc = Location::new(3, 7);
    loc.reset();
    assert_eq!(loc.row(), 0);
    assert_eq!(loc.column(), 0);
}

#[test]
fn test_location_go_right() {
    let mut loc = Location::new(3, 7);
    loc.go_right();
    assert_eq!(loc.row(), 3);
    assert_eq!(loc.column(), 8);
}

#[test]
fn test_location_new_line() {
    let mut loc = Location::new(3, 7);
    loc.new_line();
    assert_eq!(loc.row(), 4);
    assert_eq!(loc.column(), 1);
}

#[test]
fn test_ast_location1() {
    let expr = parser::parse_expression("a + b - 32");
    assert!(expr.is_ok());
    let expr = expr.unwrap();
    assert_eq!(expr.location, Location::new(0, 7));
    if let BinaryExpression {
        left,
        operator: _,
        right,
    } = expr.node
    {
        assert_eq!(left.location, Location::new(0, 3));
        assert_eq!(right.location, Location::new(0, 9));
    } else {
        assert!(false);
    }
}

#[test]
fn test_ast_location2() {
    let expr = parser::parse_expression("if a < 2 {\na = 2;\nb = 2;\n}");
    assert!(expr.is_ok());
    let expr = expr.unwrap();
    assert_eq!(expr.location, Location::new(0, 1));
    if let IfExpression {
        condition,
        if_statement,
        else_statement,
    } = expr.node
    {
        assert_eq!(condition.location, Location::new(0, 6));
        assert_eq!(if_statement.location, Location::new(0, 10));
        assert!(else_statement.is_none())
    } else {
        assert!(false);
    }
}
