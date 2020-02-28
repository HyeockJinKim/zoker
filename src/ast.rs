// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Program {
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    // TODO: After customizing the lexer, get the location value.
    // pub location: Location,
    pub node: T,
}

pub type Expression = Located<ExpressionType>;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryExpression {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Number {
        value: i32,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic Operator
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Power Operator
    Pow,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,

    // Increment Operator
    PrefixPlusPlus,
    PrefixMinusMinus,
    PostfixPlusPlus,
    PostfixMinusMinus,
}
