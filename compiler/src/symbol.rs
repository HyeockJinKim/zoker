use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Unknown,
    Contract,
    Function,
    Tuple(Vec<SymbolType>),
    Uint256,
    Int256,
    String,
    Address,
    Bytes32,
    Bytes,
    Bool,
    None,
}

pub fn vec_to_type(vec: Vec<SymbolType>) -> SymbolType {
    if vec.is_empty() {
        SymbolType::None
    } else if vec.len() == 1 {
        vec[0].clone()
    } else {
        SymbolType::Tuple(vec)
    }
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SymbolType::Unknown => write!(f, "Unknown Type"),
            SymbolType::Contract => write!(f, "Contract"),
            SymbolType::Function => write!(f, "Function"),
            SymbolType::Tuple(_) => write!(f, "Tuple"),
            SymbolType::Uint256 => write!(f, "Uint256"),
            SymbolType::Int256 => write!(f, "Int256"),
            SymbolType::String => write!(f, "String"),
            SymbolType::Address => write!(f, "Address"),
            SymbolType::Bytes32 => write!(f, "Bytes32"),
            SymbolType::Bytes => write!(f, "Bytes"),
            SymbolType::Bool => write!(f, "Bool"),
            SymbolType::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolUsage {
    Used,
    Declared,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolTableType {
    Global,
    Contract,
    Function,
    Scope,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolLocation {
    Unknown,
    Storage,
    Memory,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_location: SymbolLocation,
    pub role: SymbolUsage,
}

impl Symbol {
    pub fn new(
        name: String,
        role: SymbolUsage,
        symbol_type: SymbolType,
        data_location: SymbolLocation,
    ) -> Self {
        Symbol {
            name,
            symbol_type,
            data_location,
            role,
        }
    }
}
