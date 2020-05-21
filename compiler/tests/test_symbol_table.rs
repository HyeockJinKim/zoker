use zoker_compiler::error::{CompileError, CompileErrorType};
use zoker_compiler::symbol::{Symbol, SymbolLocation, SymbolType, SymbolUsage};
use zoker_compiler::symbol_table;
use zoker_parser::parser;

#[test]
fn test_symbol_table_uint_expression() {
    let num = parser::parse_program("uint a = 3; int b; uint c = a + b;").unwrap();
    let table = symbol_table::make_symbol_tables(&num);
    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.name, String::from("#Global"));
    assert!(table.sub_tables.is_empty());
    let a = Symbol {
        name: "a".to_string(),
        symbol_type: SymbolType::Uint256,
        data_location: SymbolLocation::Memory,
        role: SymbolUsage::Declared,
    };
    let b = Symbol {
        name: "b".to_string(),
        symbol_type: SymbolType::Int256,
        data_location: SymbolLocation::Memory,
        role: SymbolUsage::Declared,
    };
    let c = Symbol {
        name: "c".to_string(),
        symbol_type: SymbolType::Uint256,
        data_location: SymbolLocation::Memory,
        role: SymbolUsage::Declared,
    };
    assert_eq!(table.symbols.get("a").unwrap(), &a);
    assert_eq!(table.symbols.get("b").unwrap(), &b);
    assert_eq!(table.symbols.get("c").unwrap(), &c);
}

#[test]
fn test_undeclared_used_expression1() {
    let num = parser::parse_program("uint a = 3; int b; c = a + b;").unwrap();
    let table = symbol_table::make_symbol_tables(&num);
    assert!(table.is_err())
}

#[test]
fn test_undeclared_used_expression2() {
    let num = parser::parse_program("contract ABC { uint a; function f() { b = a; } }").unwrap();
    let table = symbol_table::make_symbol_tables(&num);
    assert!(table.is_err());
    assert_eq!(
        table.err().unwrap(),
        CompileError {
            error: CompileErrorType::SyntaxError(String::from(
                "Variable b is not declared, but used."
            )),
            location: Default::default(),
        }
    );
}
