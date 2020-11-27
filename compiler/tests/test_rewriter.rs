use zoker_compiler::rewriter::rewrite_program;
use zoker_compiler::symbol::SymbolType;
use zoker_parser::parser;

#[test]
fn test_rewriting() {
    let source = "contract Test {\
           function add(private uint a, uint b) returns (uint) {\
             return a + b + 1;\
           }\
        }";
    let res = parser::parse_program(source);
    let program = res.unwrap();
    let res = rewrite_program(&program);
    assert!(res.is_ok());
    let contracts = res.unwrap();
    assert_eq!(contracts.len(), 1);
    assert_eq!(contracts[0].name, "Test");
    assert_eq!(contracts[0].functions[0].name, "add");

    assert_eq!(
        contracts[0].functions[0].params[0].symbol_type,
        SymbolType::Uint256
    );
    assert_eq!(
        contracts[0].functions[0].params[1].symbol_type,
        SymbolType::Uint256
    );
    assert_eq!(
        contracts[0].functions[0].returns[0].symbol_type,
        SymbolType::Uint256
    );
}

#[test]
fn test_rewriting_multiple_function() {
    let source = "contract Test {\
           function add(private uint a, uint b) returns (uint) {\
             return a + b + 1;\
           }\
           function add2(private uint a, uint b, uint c) returns (uint) {\
             return add(add(a + b) + c);\
           }\
        }";
    let res = parser::parse_program(source);
    let program = res.unwrap();
    let res = rewrite_program(&program);
    assert!(res.is_ok());
    let contracts = res.unwrap();
    println!("{:#?}", contracts[0]);
    assert_eq!(contracts[0].functions[0].params[0].num, 0);
    assert_eq!(contracts[0].functions[0].params[1].num, 0);

    assert_eq!(contracts[0].functions[1].params[0].num, 0);
    assert_eq!(contracts[0].functions[1].params[1].num, 0);
    assert_eq!(contracts[0].functions[1].params[2].num, 1);
}
