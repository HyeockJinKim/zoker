use zoker_compiler::symbol_table;
use zoker_compiler::symbol_table::{Symbol, SymbolScope, SymbolType, SymbolUsage};
use zoker_parser::parser;

#[test]
fn test_print_ternary_expression() {
    let num = parser::parse_program("uint a = 3; int b; c = a + b;").unwrap();
    let table = symbol_table::make_symbol_tables(&num);
    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.name, String::from("#Global"));
    assert!(table.sub_tables.is_empty());
    let a = Symbol {
        name: "a".to_string(),
        symbol_type: SymbolType::Uint256,
        scope: SymbolScope::Unknown,
        role: SymbolUsage::Declared,
    };
    let b = Symbol {
        name: "b".to_string(),
        symbol_type: SymbolType::Int256,
        scope: SymbolScope::Unknown,
        role: SymbolUsage::Declared,
    };
    let c = Symbol {
        name: "c".to_string(),
        symbol_type: SymbolType::Unknown,
        scope: SymbolScope::Unknown,
        role: SymbolUsage::Used,
    };
    assert_eq!(table.symbols.get("a").unwrap(), &a);
    assert_eq!(table.symbols.get("b").unwrap(), &b);
    assert_eq!(table.symbols.get("c").unwrap(), &c);
}
