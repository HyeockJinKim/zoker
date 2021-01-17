use num_bigint::BigUint;
use std::fmt;
use zoker_parser::ast::{Specifier, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Uint256,
    Int256,
    String,
    Address,
    Bytes32,
    Bool,
    None,
}

pub fn token_to_type(typ: &Type) -> SymbolType {
    match typ {
        Type::Uint256 => SymbolType::Uint256,
        Type::Int256 => SymbolType::Int256,
        Type::Bytes32 => SymbolType::Bytes32,
        Type::Bool => SymbolType::Bool,
        Type::Bytes => SymbolType::Bytes32,
        Type::String => SymbolType::String,
        Type::Address => SymbolType::Address,
    }
}

pub fn symbol_to_string(typ: &SymbolType) -> &str {
    match typ {
        SymbolType::Uint256 => "uint",
        SymbolType::Int256 => "int",
        SymbolType::String => "string",
        SymbolType::Address => "address",
        SymbolType::Bytes32 => "bytes",
        SymbolType::Bool => "bool",
        SymbolType::None => "null",
    }
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", symbol_to_string(self))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub name: String,
    pub functions: Vec<Function>,
}

impl Contract {
    pub fn new(name: String) -> Self {
        Contract {
            name,
            functions: vec![],
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn add_operation_all(&mut self, operations: Vec<Operation>) {
        self.functions
            .last_mut()
            .unwrap()
            .add_operations(operations);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Symbol>,
    pub operations: Vec<Operation>,
    pub returns: Vec<Symbol>,
    pub private_num: u32,
    pub public_num: u32,
}

impl Function {
    pub fn new(name: String, params: Vec<Symbol>, returns: Vec<Symbol>) -> Self {
        Function {
            name,
            params,
            operations: vec![],
            returns,
            private_num: 0,
            public_num: 0,
        }
    }

    pub fn add_operations(&mut self, operations: Vec<Operation>) {
        self.operations.extend(operations);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operation: OperationType,
}

impl Operation {
    pub fn new_symbol(symbol: Symbol) -> Self {
        Operation {
            operation: OperationType::Symbol { symbol },
        }
    }

    pub fn new_call(func: String, args: Vec<Operation>) -> Self {
        Operation {
            operation: OperationType::Call { func, args },
        }
    }

    pub fn new(operation: OperationType) -> Self {
        Operation { operation }
    }

    pub fn as_symbol(&self) -> Option<Symbol> {
        match &self.operation {
            OperationType::Symbol { symbol } => Some(symbol.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Add {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    Sub {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    Mul {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    Assign {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    For {
        iter: Box<Operation>,
        vector: Box<Operation>,
        stmts: Vec<Operation>,
    },
    If {
        cond: Box<Operation>,
        stmts: Vec<Operation>,
    },
    Else {
        cond: Box<Operation>,
        stmts: Vec<Operation>,
    },
    Return {
        ret: Box<Operation>,
    },
    Call {
        func: String,
        args: Vec<Operation>,
    },
    Symbol {
        symbol: Symbol,
    },
    Constant {
        value: BigUint,
    },
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolLocation {
    Unknown,
    Storage,
    Memory,
}

pub fn specifier_to_location(loc: &Specifier) -> SymbolLocation {
    match loc {
        Specifier::Memory => SymbolLocation::Memory,
        Specifier::Storage => SymbolLocation::Storage,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub num: u32,
    pub symbol_type: SymbolType,
    pub data_location: SymbolLocation,
    pub is_private: bool,
}

impl Symbol {
    pub fn new(
        name: String,
        num: u32,
        symbol_type: SymbolType,
        data_location: SymbolLocation,
        is_private: bool,
    ) -> Self {
        Symbol {
            name,
            num,
            symbol_type,
            data_location,
            is_private,
        }
    }

    pub fn new_type_symbol(symbol_type: SymbolType) -> Self {
        Symbol {
            name: String::new(),
            num: 0,
            symbol_type,
            data_location: SymbolLocation::Unknown,
            is_private: false,
        }
    }
}
