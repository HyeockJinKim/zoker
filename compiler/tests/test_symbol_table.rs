use zoker_compiler::error::CompileError;
use zoker_compiler::error::CompileErrorType::TypeError;
use zoker_compiler::symbol::{Symbol, SymbolLocation, SymbolType, SymbolUsage};
use zoker_compiler::symbol_table;
use zoker_parser::location::Location;
use zoker_parser::parser;

#[test]
fn test_symbol_table_uint_expression() {
    let num = parser::parse_program("uint a = 3; uint b; uint c = a + b;").unwrap();
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
        symbol_type: SymbolType::Uint256,
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
            error: TypeError(String::from(
            "In Assign Expression, both left and right variable must be of the same type. but Unknown Type type is not same as Uint256"
            )),
            location: Location::new(0, 43)
        }
    );
}

#[test]
fn test_symbol_table_function_call() {
    let num = parser::parse_program(
        "contract A { function f() { uint a = 3; g(3); } function g(uint a) {} }",
    )
    .unwrap();
    let table = symbol_table::make_symbol_tables(&num);
    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.name, String::from("#Global"));
    assert_eq!(table.sub_tables.len(), 1);
    assert_eq!(table.sub_tables[0].sub_tables.len(), 2);
    let a = Symbol {
        name: "a".to_string(),
        symbol_type: SymbolType::Uint256,
        data_location: SymbolLocation::Memory,
        role: SymbolUsage::Declared,
    };
    assert_eq!(
        table.sub_tables[0].sub_tables[0].symbols.get("a").unwrap(),
        &a
    );
}
