use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    // Limit is bit number.
    Uint { limit: u32, val: BigUint },
    Int { limit: u32, val: BigInt },
    Bool { limit: u32, val: bool },
    String { limit: u32, val: String },
    Bytes { limit: u32, val: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeObject {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NameScope {
    Global,
    Contract,
    Local,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataLocation {
    Storage,
    Memory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    // Load Constant to Register
    LoadConst { constant: DataType, reg: u32 },

    // Binary Operation ( Left register, Right register, Return register )
    Add { left: u32, right: u32, ret: u32 },
    Sub { left: u32, right: u32, ret: u32 },
    Mul { left: u32, right: u32, ret: u32 },
    Div { left: u32, right: u32, ret: u32 },
    Mod { left: u32, right: u32, ret: u32 },
    BitAnd { left: u32, right: u32, ret: u32 },
    BitOr { left: u32, right: u32, ret: u32 },
    BitXor { left: u32, right: u32, ret: u32 },
    LShift { left: u32, right: u32, ret: u32 },
    RShift { left: u32, right: u32, ret: u32 },
    And { left: u32, right: u32, ret: u32 },
    Or { left: u32, right: u32, ret: u32 },
    Lt { left: u32, right: u32, ret: u32 },
    Le { left: u32, right: u32, ret: u32 },
    Eq { left: u32, right: u32, ret: u32 },
    Gt { left: u32, right: u32, ret: u32 },
    Ge { left: u32, right: u32, ret: u32 },
    NotEq { left: u32, right: u32, ret: u32 },

    // Condition Expression
    If { condition: u32 },
    For { vector: u32 },
    Else,
    End,
}
