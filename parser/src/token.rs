use num_bigint::BigUint;

/// Zoker source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    // Operator
    // Arithmetic Operator
    Mul,
    Div,
    Mod,
    // Power Operator
    Pow,
    // Shift operator
    LShift,
    RShift,
    // Unary Operator
    Plus,
    Minus,
    Not,
    // Increment Operator
    PlusPlus,
    MinusMinus,
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

    // Type
    // Static size
    Uint256,
    Int256,
    Bytes32,
    Bool,
    // Dynamic size
    Bytes,
    String,
    Address,

    // Keyword
    Function,
    Contract,
    Memory,
    Storage,
    If,
    Else,
    For,
    In,
    // Mark
    LPar,
    RPar,
    LBrace,
    RBrace,
    Semi,
    Comma,
    Question,
    Colon,
    // variable
    Num { number: BigUint },
    Identifier { name: String },
    Literal { literal: String },
    EOF,
}
