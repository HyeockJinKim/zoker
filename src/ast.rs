// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Program {
    Expression(Expression),
}

impl Program {
    pub fn to_child(&self) -> &Located<ExpressionType> {
        match self {
            Program::Expression(expr) => expr,
        }
    }
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
        operator: Operator,
        right: Box<Expression>,
    },
    UnaryExpression {
        operator: Operator,
        expression: Box<Expression>,
    },
    Number {
        value: i32,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    // Arithmetic Operator
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Power Operator
    Pow,

    // Unary Operator
    Plus,
    Minus,

    // Increment Operator
    PrefixPlusPlus,
    PrefixMinusMinus,
    PostfixPlusPlus,
    PostfixMinusMinus,
}
