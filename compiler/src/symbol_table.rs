use indexmap::map::IndexMap;
use zoker_parser::ast;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SymbolType {
    Uint256,
    Int256,
    String,
    Address,
    Bytes,
    Bool,
}

#[derive(Clone, Copy, PartialEq)]
enum SymbolTableType {
    Contract,
    Function,
}

#[derive(Debug, Clone)]
pub enum SymbolScope {
    Storage,
    Memory,
}

#[derive(Debug, Clone)]
struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: SymbolScope,
}

#[derive(Clone)]
struct SymbolTable {
    pub name: String,
    pub table_type: SymbolTableType,
    pub symbols: IndexMap<String, Symbol>,
    pub sub_tables: Vec<SymbolTable>,
}

#[derive(Default)]
struct SymbolTableBuilder {
    pub tables: Vec<SymbolTable>,
}

fn make_symbol_tables(program: ast::Program) -> SymbolTable {
    SymbolTableBuilder::new().prepare_table(program).build()
}

impl SymbolTableBuilder {
    fn new() -> Self {
        SymbolTableBuilder { tables: vec![] }
    }

    fn prepare_table(mut self, program: ast::Program) -> Self {
        self
    }

    fn build(mut self) -> SymbolTable {
        self.tables.pop().unwrap()
    }
}
