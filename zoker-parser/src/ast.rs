use num_bigint::BigUint;

use crate::location::Location;

// https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq)]
pub enum Program {
    GlobalStatements(Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub struct Located<T> {
    pub location: Location,
    pub node: T,
}

pub type Statement = Located<StatementType>;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    // Global Statement
    FunctionStatement {
        function_name: Box<Expression>,
        parameters: Box<Expression>,
        statement: Box<Statement>,
    },
    // Local Statement
    CompoundStatement {
        statements: Vec<Statement>,
        return_value: Option<Box<Expression>>,
    },
    Expression {
        expression: Box<Expression>,
    },
}

pub type Expression = Located<ExpressionType>;

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    AssignExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    InitializerExpression {
        variable_type: Type,
        variable: Box<Expression>,
        default: Option<Box<Expression>>,
    },
    FunctionCallExpression {
        function_name: Box<Expression>,
        arguments: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        if_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    ForEachExpression {
        iterator: Box<Expression>,
        vector: Box<Expression>,
        statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    UnaryExpression {
        operator: Operator,
        expression: Box<Expression>,
    },
    Parameters {
        parameters: Vec<Expression>,
    },
    Arguments {
        arguments: Vec<Expression>,
    },
    Number {
        value: BigUint,
    },
    Identifier {
        value: String,
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
    Not,

    // Increment Operator
    PrefixPlusPlus,
    PrefixMinusMinus,
    PostfixPlusPlus,
    PostfixMinusMinus,

    // Assign operator
    Assign,

    // Augmented Assign Operator
    BitAndAssign,
    BitXorAssign,
    BitOrAssign,
    LShiftAssign,
    RShiftAssign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,

    // Comparison Operator
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    NotEq,

    // Logical Operator
    And,
    Or,

    // Bit Operator
    BitAnd,
    BitXor,
    BitOr,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    // Static size
    Uint256,
    Int256,
    Bytes32,
    Bool,

    // Dynamic size
    Bytes,
    String,
    Address,
    // To be supported..
    // Mapping,
    // Var,
}
