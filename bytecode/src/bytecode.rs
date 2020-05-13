use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    // Limit is bit number.
    Uint { limit: u32, val: BigUint },
    Int { limit: u32, val: BigInt },
    Bool { limit: u32, val: bool },
    String { limit: u32, val: String },
    Bytes { limit: u32, val: Vec<u8> },
    Address { limit: u32, val: BigUint },
    Code { code: Box<CodeObject> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeObject {
    pub name: String,
    pub scope: NameScope,
    pub start_register: u32,
    pub instructions: Vec<Instruction>,
    pub sub_code: Vec<CodeObject>,
}

impl CodeObject {
    pub fn new(name: String, scope: NameScope, start_register: u32) -> Self {
        CodeObject {
            name,
            scope,
            start_register,
            instructions: vec![],
            sub_code: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NameScope {
    Global,
    Contract,
    Local,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegisterType {
    Constant,
    Variable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Register {
    pub name: String,
    pub typ: RegisterType,
    pub value: Constant,
    pub number: u32,
}

impl Register {
    pub fn new(name: String, typ: RegisterType, number: u32, value: Constant) -> Self {
        Register {
            name,
            typ,
            value,
            number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    // Load Constant to Register
    LoadConst { constant: u32, reg: u32 },

    // Binary Operation ( Left register, Right register, Return register )
    Add { left: u32, right: u32 },
    Sub { left: u32, right: u32 },
    Mul { left: u32, right: u32 },
    Div { left: u32, right: u32 },
    Mod { left: u32, right: u32 },
    BitAnd { left: u32, right: u32 },
    BitOr { left: u32, right: u32 },
    BitXor { left: u32, right: u32 },
    LShift { left: u32, right: u32 },
    RShift { left: u32, right: u32 },
    And { left: u32, right: u32 },
    Or { left: u32, right: u32 },
    Lt { left: u32, right: u32 },
    Le { left: u32, right: u32 },
    Eq { left: u32, right: u32 },
    Gt { left: u32, right: u32 },
    Ge { left: u32, right: u32 },
    NotEq { left: u32, right: u32 },

    // Condition Expression
    If { condition: u32 },
    For { reg: u32, vector: u32 },
    Else,
    End,
}
