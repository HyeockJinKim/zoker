use indexmap::map::IndexMap;
use std::ops::Add;
use zoker_parser::ast;
use zoker_parser::ast::{ExpressionType, StatementType, Type};
use zoker_parser::location::Location;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SymbolType {
    Unknown,
    // Contract,
    Function,
    Uint256,
    Int256,
    String,
    Address,
    Bytes32,
    Bytes,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolUsage {
    Used,
    Declared,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SymbolTableType {
    Global,
    // Contract,
    Function,
    Local,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolScope {
    Unknown,
    Storage,
    Memory,
}

#[derive(Debug)]
pub struct SymbolTableError {
    error: String,
    location: Location,
}

type SymbolTableResult = Result<(), SymbolTableError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: SymbolScope,
    pub role: SymbolUsage,
}

#[derive(Clone)]
pub struct SymbolTable {
    pub name: String,
    pub table_type: SymbolTableType,
    pub symbols: IndexMap<String, Symbol>,
    pub sub_tables: Vec<SymbolTable>,
}

#[derive(Default)]
struct SymbolTableBuilder {
    pub if_num: Vec<u32>,
    pub for_num: Vec<u32>,
    pub compound_num: Vec<u32>,
    pub tables: Vec<SymbolTable>,
}

pub fn make_symbol_tables(program: &ast::Program) -> Result<SymbolTable, SymbolTableError> {
    SymbolTableBuilder::new().prepare_table(program)?.build()
}

fn name_from_expression(expr: &ast::Expression) -> Option<String> {
    if let ast::ExpressionType::Identifier { value } = &expr.node {
        Some(value.clone())
    } else {
        None
    }
}

impl SymbolTableBuilder {
    fn new() -> Self {
        SymbolTableBuilder {
            if_num: vec![],
            for_num: vec![],
            compound_num: vec![],
            tables: vec![],
        }
    }

    fn prepare_table(mut self, program: &ast::Program) -> Result<Self, SymbolTableError> {
        self.enter_program(program)?;
        Ok(self)
    }

    fn enter_scope(&mut self, name: String, table_type: SymbolTableType) {
        self.if_num.push(0);
        self.for_num.push(0);
        self.compound_num.push(0);
        self.tables.push(SymbolTable {
            name,
            table_type,
            symbols: Default::default(),
            sub_tables: vec![],
        });
    }

    fn exit_scope(&mut self) {
        self.if_num.pop();
        self.for_num.pop();
        self.compound_num.pop();
        let table = self.tables.pop().unwrap();
        self.tables.last_mut().unwrap().sub_tables.push(table);
    }

    fn enter_program(&mut self, program: &ast::Program) -> SymbolTableResult {
        self.enter_scope(String::from("#Global"), SymbolTableType::Global);
        match program {
            ast::Program::GlobalStatements(stmts) => {
                self.enter_global_statements(stmts)?;
            }
        }
        Ok(())
    }

    fn enter_global_statements(&mut self, statements: &[ast::Statement]) -> SymbolTableResult {
        for stmt in statements {
            self.enter_statement(stmt)?;
        }
        Ok(())
    }

    fn enter_block(&mut self, compound: &ast::Statement) -> SymbolTableResult {
        if let ast::StatementType::CompoundStatement {
            statements,
            return_value,
        } = &compound.node
        {
            for stmt in statements {
                self.enter_statement(stmt)?;
            }
            if let Some(returns) = return_value {
                self.enter_expression(returns)?;
            }
        }
        Ok(())
    }

    fn enter_statement(&mut self, statement: &ast::Statement) -> SymbolTableResult {
        match &statement.node {
            ast::StatementType::Expression { expression: expr } => self.enter_expression(expr)?,
            StatementType::FunctionStatement {
                function_name: func,
                parameters: params,
                statement: stmt,
            } => {
                let name = name_from_expression(func).unwrap();
                let tables = self.tables.last_mut().unwrap();
                let symbol = Symbol::new(name.clone(), SymbolUsage::Declared, SymbolType::Function);
                tables.symbols.insert(name.clone(), symbol);

                self.enter_scope(name, SymbolTableType::Function);
                self.enter_expression(params)?;
                self.enter_block(stmt)?;
                self.exit_scope();
            }
            StatementType::InitializerStatement {
                variable_type,
                variable: var,
                default,
            } => {
                self.register_identifier(var, variable_type);
                if let Some(expr) = default {
                    self.enter_expression(expr)?;
                }
            }
            StatementType::CompoundStatement {
                statements: stmts,
                return_value: returns,
            } => {
                let number = self.compound_num.last_mut().unwrap();
                *number += 1;
                let name = String::from("#Compound_").add(&*(number).to_string());
                self.enter_scope(name, SymbolTableType::Local);
                for stmt in stmts {
                    self.enter_statement(stmt)?;
                }
                if let Some(expr) = returns {
                    self.enter_expression(expr)?;
                }
                self.exit_scope();
            }
        }
        Ok(())
    }

    fn enter_expression(&mut self, expression: &ast::Expression) -> SymbolTableResult {
        match &expression.node {
            ast::ExpressionType::AssignExpression {
                left,
                operator: _,
                right,
            } => {
                self.enter_expression(left)?;
                self.enter_expression(right)?;
            }
            ExpressionType::TernaryExpression {
                condition,
                expr1,
                expr2,
            } => {
                self.enter_expression(condition)?;
                self.enter_expression(expr1)?;
                self.enter_expression(expr2)?;
            }
            ExpressionType::BinaryExpression {
                left,
                operator: _,
                right,
            } => {
                self.enter_expression(left)?;
                self.enter_expression(right)?;
            }
            ExpressionType::FunctionCallExpression {
                function_name,
                arguments,
            } => {
                self.enter_expression(function_name)?;
                self.enter_expression(arguments)?;
            }
            ExpressionType::IfExpression {
                condition,
                if_statement,
                else_statement,
            } => {
                self.enter_expression(condition)?;
                let if_num = self.if_num.last_mut().unwrap();
                *if_num += 1;
                let if_name = String::from("#If_").add(&*(if_num).to_string());
                let else_name = String::from("#Else_").add(&*(if_num).to_string());
                self.enter_scope(if_name, SymbolTableType::Local);
                self.enter_block(if_statement)?;
                self.exit_scope();

                if let Some(expr) = else_statement {
                    self.enter_scope(else_name, SymbolTableType::Local);
                    self.enter_block(expr)?;
                    self.exit_scope();
                }
            }
            ExpressionType::ForEachExpression {
                iterator,
                vector,
                statement,
                else_statement,
            } => {
                self.check_identifier(vector);
                let for_num = self.for_num.last_mut().unwrap();
                *for_num += 1;
                let for_name = String::from("#For_").add(&*(for_num).to_string());
                let else_name = String::from("#Else_").add(&*(for_num).to_string());
                self.enter_scope(for_name, SymbolTableType::Local);
                self.enter_expression(iterator)?;
                self.enter_block(statement)?;
                self.exit_scope();
                if let Some(stmt) = else_statement {
                    self.enter_scope(else_name, SymbolTableType::Local);
                    self.enter_block(stmt)?;
                    self.exit_scope();
                }
            }
            ExpressionType::UnaryExpression {
                operator: _,
                expression,
            } => {
                self.enter_expression(expression)?;
            }
            ExpressionType::Parameters { parameters: params } => {
                for param in params {
                    self.enter_statement(param)?;
                }
            }
            ExpressionType::Arguments { arguments: args } => {
                for arg in args {
                    self.enter_expression(arg)?;
                }
            }
            ExpressionType::Number { .. } => {}
            ExpressionType::Identifier { .. } => {
                self.check_identifier(expression);
            }
        }
        Ok(())
    }

    fn check_identifier(&mut self, identifier: &ast::Expression) {
        let name = name_from_expression(identifier).unwrap();
        let tables = self.tables.last_mut().unwrap();
        if tables.symbols.get_mut(&name).is_none() {
            let symbol = Symbol::new(name.clone(), SymbolUsage::Used, SymbolType::Unknown);
            tables.symbols.insert(name, symbol);
        } else {
            // TODO: Check Undeclared Variable.
        }
    }

    fn register_identifier(&mut self, expr: &ast::Expression, typ: &ast::Type) {
        let name = name_from_expression(expr).unwrap();
        // TODO: Check for symbol already in table.
        let symbol_type = match typ {
            ast::Type::String => SymbolType::String,
            Type::Uint256 => SymbolType::Uint256,
            Type::Int256 => SymbolType::Int256,
            Type::Bytes32 => SymbolType::Bytes32,
            Type::Bool => SymbolType::Bool,
            Type::Bytes => SymbolType::Bytes,
            Type::Address => SymbolType::Address,
        };

        let symbol = Symbol::new(name.clone(), SymbolUsage::Declared, symbol_type);
        let tables = self.tables.last_mut().unwrap();
        tables.symbols.insert(name, symbol);
    }

    fn build(mut self) -> Result<SymbolTable, SymbolTableError> {
        Ok(self.tables.pop().unwrap())
    }
}

impl Symbol {
    fn new(name: String, role: SymbolUsage, symbol_type: SymbolType) -> Self {
        Symbol {
            name,
            symbol_type,
            scope: SymbolScope::Unknown,
            role,
        }
    }
}
