use crate::error::{CompileError, CompileErrorType};
use crate::symbol::{Symbol, SymbolLocation, SymbolTableType, SymbolType, SymbolUsage};
use crate::type_checker::get_type;
use indexmap::map::IndexMap;
use std::ops::Add;
use zoker_parser::ast;
use zoker_parser::location::Location;

type SymbolTableResult = Result<SymbolType, CompileError>;
type AnalysisResult = Result<(), CompileError>;

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

#[derive(Default)]
struct SymbolAnalyzer {
    tables: Vec<AnalysisTable>,
}

struct AnalysisTable {
    pub map: IndexMap<String, Symbol>,
    pub typ: SymbolTableType,
}

impl AnalysisTable {
    fn new(map: IndexMap<String, Symbol>, typ: SymbolTableType) -> Self {
        AnalysisTable { map, typ }
    }
}

pub fn make_symbol_tables(program: &ast::Program) -> Result<SymbolTable, CompileError> {
    SymbolTableBuilder::new().prepare_table(program)?.build()
}

impl SymbolAnalyzer {
    fn analyze_symbol_table(&mut self, table: &SymbolTable) -> AnalysisResult {
        let sub_tables = &table.sub_tables;

        self.tables
            .push(AnalysisTable::new(table.symbols.clone(), table.table_type));

        for sub_table in sub_tables {
            self.analyze_symbol_table(sub_table)?;
        }
        let mut analysis_table = self.tables.pop().unwrap();

        for value in analysis_table.map.values_mut() {
            self.analyze_symbol(value)?;
        }
        Ok(())
    }

    fn analyze_symbol(&mut self, symbol: &Symbol) -> AnalysisResult {
        match symbol.role {
            SymbolUsage::Declared => {
                // No need to do anything.
            }
            SymbolUsage::Used => {
                let is_declared = self.tables.iter().any(|table| {
                    if let Some(sym) = table.map.get(&symbol.name) {
                        sym.role != SymbolUsage::Used
                    } else {
                        false
                    }
                });

                if !is_declared {
                    return Err(CompileError {
                        error: CompileErrorType::SyntaxError(format!(
                            "Variable {} is not declared, but used.",
                            symbol.name
                        )),
                        location: Default::default(),
                    });
                }
            }
        }
        Ok(())
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

    fn prepare_table(mut self, program: &ast::Program) -> Result<Self, CompileError> {
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
        Ok(SymbolType::None)
    }

    fn enter_global_statements(&mut self, statements: &[ast::Statement]) -> SymbolTableResult {
        for stmt in statements {
            self.enter_statement(stmt, &SymbolLocation::Memory)?;
        }
        Ok(SymbolType::None)
    }

    fn enter_block(
        &mut self,
        compound: &ast::Statement,
        location: &SymbolLocation,
    ) -> SymbolTableResult {
        if let ast::StatementType::CompoundStatement {
            statements,
            return_value,
        } = &compound.node
        {
            for stmt in statements {
                self.enter_statement(stmt, location)?;
            }
            if let Some(returns) = return_value {
                self.enter_expression(returns)
            } else {
                Ok(SymbolType::None)
            }
        } else {
            Ok(SymbolType::None)
        }
    }

    fn enter_statement(
        &mut self,
        statement: &ast::Statement,
        location: &SymbolLocation,
    ) -> SymbolTableResult {
        match &statement.node {
            ast::StatementType::Expression { expression: expr } => self.enter_expression(expr),
            ast::StatementType::FunctionStatement {
                function_name: func,
                parameters: params,
                statement: stmt,
            } => {
                let name = func.node.identifier_name().unwrap();

                self.enter_scope(name.clone(), SymbolTableType::Function);
                self.enter_expression(params)?;
                self.enter_block(stmt, &SymbolLocation::Unknown)?;
                self.exit_scope();
                let symbol = Symbol::new(
                    name.clone(),
                    SymbolUsage::Declared,
                    SymbolType::Function,
                    SymbolLocation::Storage,
                );

                self.tables.last_mut().unwrap().symbols.insert(name, symbol);
                Ok(SymbolType::None)
            }
            ast::StatementType::ContractStatement {
                contract_name: name,
                members: stmts,
            } => {
                let name = name.node.identifier_name().unwrap();
                let tables = self.tables.last_mut().unwrap();
                let symbol = Symbol::new(
                    name.clone(),
                    SymbolUsage::Declared,
                    SymbolType::Contract,
                    SymbolLocation::Storage,
                );
                tables.symbols.insert(name.clone(), symbol);

                self.enter_scope(name, SymbolTableType::Contract);
                self.enter_statement(stmts, location)?;
                self.exit_scope();
                Ok(SymbolType::None)
            }
            ast::StatementType::InitializerStatement {
                variable_type,
                data_location: loc,
                variable: var,
                default,
            } => {
                let typ = if let Some(data_location) = loc {
                    let data_location = match data_location {
                        ast::Specifier::Storage => SymbolLocation::Storage,
                        ast::Specifier::Memory => SymbolLocation::Memory,
                    };
                    self.register_identifier(var, variable_type, &data_location)
                } else {
                    self.register_identifier(var, variable_type, location)
                };
                if let Some(expr) = default {
                    self.enter_expression(expr)?;
                }
                Ok(typ)
            }
            ast::StatementType::CompoundStatement {
                statements: stmts,
                return_value: returns,
            } => {
                let number = self.compound_num.last_mut().unwrap();
                *number += 1;
                let name = String::from("#Compound_").add(&*(number).to_string());
                self.enter_scope(name, SymbolTableType::Local);
                for stmt in stmts {
                    self.enter_statement(stmt, location)?;
                }
                let ret = if let Some(expr) = returns {
                    self.enter_expression(expr)
                } else {
                    Ok(SymbolType::None)
                };
                self.exit_scope();
                ret
            }
            ast::StatementType::MemberStatement {
                statements: members,
            } => {
                for member in members {
                    self.enter_statement(member, &SymbolLocation::Storage)?;
                }
                Ok(SymbolType::None)
            }
        }
    }

    fn enter_expression(&mut self, expression: &ast::Expression) -> SymbolTableResult {
        match &expression.node {
            ast::ExpressionType::AssignExpression { left, right, .. } => {
                let left_type = self.enter_expression(left)?;
                let right_type = self.enter_expression(right)?;
                let err_msg = format!(
                    "In Assign Expression, both left and right variable must be of the same type. but {} type is not same as {}",
                    left_type, right_type
                );
                self.compare_type(left_type, right_type, right.location, err_msg)
            }
            ast::ExpressionType::TernaryExpression {
                condition,
                expr1,
                expr2,
            } => {
                let cond_type = self.enter_expression(condition)?;
                let expr1_type = self.enter_expression(expr1)?;
                let expr2_type = self.enter_expression(expr2)?;
                if cond_type != SymbolType::Bool {
                    return Err(CompileError {
                        error: CompileErrorType::TypeError(format!(
                            "condition type must be bool type, but {}",
                            cond_type
                        )),
                        location: condition.location,
                    });
                }
                let err_msg = format!(
                    "Ternary expression's return type should be the same, but {} type is not same as {}",
                    expr1_type, expr2_type
                );
                self.compare_type(expr1_type, expr2_type, expr1.location, err_msg)
            }
            ast::ExpressionType::BinaryExpression { left, right, .. } => {
                let left_type = self.enter_expression(left)?;
                let right_type = self.enter_expression(right)?;
                let err_msg = format!(
                    "In binary operations, both operands must be of the same type. But {} type is not same as {}",
                    left_type, right_type
                );
                self.compare_type(left_type, right_type, right.location, err_msg)
            }
            ast::ExpressionType::FunctionCallExpression {
                function_name,
                arguments,
            } => {
                self.enter_expression(function_name)?;
                self.enter_expression(arguments)?;
                // TODO: How to check function call type?
                Ok(SymbolType::None)
            }
            ast::ExpressionType::IfExpression {
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
                let if_type = self.enter_block(if_statement, &SymbolLocation::Unknown)?;
                self.exit_scope();

                if let Some(expr) = else_statement {
                    self.enter_scope(else_name, SymbolTableType::Local);
                    let else_type = self.enter_block(expr, &SymbolLocation::Unknown)?;
                    self.exit_scope();
                    let err_msg = format!(
                        "In if statement, both if block and else block must be of the same type., but {} type is not same as {}",
                        if_type, else_type
                    );
                    self.compare_type(if_type, else_type, if_statement.location, err_msg)
                } else {
                    Ok(SymbolType::None)
                }
            }
            ast::ExpressionType::ForEachExpression {
                iterator,
                vector,
                statement,
                else_statement,
            } => {
                // TODO: Check Iterable Type
                self.check_identifier(vector)?;
                let for_num = self.for_num.last_mut().unwrap();
                *for_num += 1;
                let for_name = String::from("#For_").add(&*(for_num).to_string());
                let else_name = String::from("#Else_").add(&*(for_num).to_string());
                self.enter_scope(for_name, SymbolTableType::Local);
                self.enter_expression(iterator)?;
                let for_type = self.enter_block(statement, &SymbolLocation::Unknown)?;
                self.exit_scope();
                if let Some(stmt) = else_statement {
                    self.enter_scope(else_name, SymbolTableType::Local);
                    let else_type = self.enter_block(stmt, &SymbolLocation::Unknown)?;
                    self.exit_scope();
                    let err_msg = format!(
                        "In For statement, both for block and else block must be of the same type. but {} type is not same as {}",
                        for_type, else_type
                    );
                    self.compare_type(for_type, else_type, statement.location, err_msg)
                } else {
                    Ok(for_type)
                }
            }
            ast::ExpressionType::UnaryExpression { expression, .. } => {
                self.enter_expression(expression)
            }
            ast::ExpressionType::Parameters { parameters } => {
                for parameter in parameters {
                    self.enter_statement(parameter, &SymbolLocation::Unknown)?;
                }
                Ok(SymbolType::Parameters)
            }
            ast::ExpressionType::Arguments { arguments } => {
                for argument in arguments {
                    self.enter_expression(argument)?;
                }
                Ok(SymbolType::Parameters)
            }
            ast::ExpressionType::Number { .. } => Ok(SymbolType::Uint256),
            ast::ExpressionType::Identifier { .. } => self.check_identifier(expression),
        }
    }

    fn compare_type(
        &self,
        left_type: SymbolType,
        right_type: SymbolType,
        location: Location,
        error: String,
    ) -> SymbolTableResult {
        if left_type == right_type {
            Ok(left_type)
        } else {
            Err(CompileError {
                error: CompileErrorType::TypeError(error),
                location,
            })
        }
    }

    fn check_identifier(&mut self, identifier: &ast::Expression) -> SymbolTableResult {
        let name = identifier.node.identifier_name().unwrap();
        let tables = self.tables.last_mut().unwrap();
        // TODO: Should check all scope tables.
        if tables.symbols.get(&name).is_none() {
            let symbol = Symbol::new(
                name.clone(),
                SymbolUsage::Used,
                SymbolType::Unknown,
                SymbolLocation::Unknown,
            );
            tables.symbols.insert(name, symbol);
            Ok(SymbolType::Unknown)
        } else {
            // TODO: Check Declared Variable?
            Ok(SymbolType::Unknown)
        }
    }

    fn register_identifier(
        &mut self,
        expr: &ast::Expression,
        typ: &ast::Type,
        loc: &SymbolLocation,
    ) -> SymbolType {
        let name = expr.node.identifier_name().unwrap();
        // TODO: Check for symbol already in table.
        let symbol_type = get_type(typ);
        let data_location = if loc != &SymbolLocation::Unknown {
            loc.clone()
        } else {
            self.default_location(symbol_type)
        };
        let symbol = Symbol::new(
            name.clone(),
            SymbolUsage::Declared,
            symbol_type,
            data_location,
        );
        let tables = self.tables.last_mut().unwrap();
        tables.symbols.insert(name, symbol);
        symbol_type
    }

    fn default_location(&self, typ: SymbolType) -> SymbolLocation {
        match typ {
            SymbolType::Unknown => SymbolLocation::Unknown,
            SymbolType::Contract => SymbolLocation::Storage,
            SymbolType::Function => SymbolLocation::Storage,
            SymbolType::Parameters => SymbolLocation::Memory,
            SymbolType::Uint256 => SymbolLocation::Memory,
            SymbolType::Int256 => SymbolLocation::Memory,
            SymbolType::String => SymbolLocation::Storage,
            SymbolType::Address => SymbolLocation::Memory,
            SymbolType::Bytes32 => SymbolLocation::Storage,
            SymbolType::Bytes => SymbolLocation::Storage,
            SymbolType::Bool => SymbolLocation::Memory,
            SymbolType::None => SymbolLocation::Unknown,
        }
    }

    fn build(mut self) -> Result<SymbolTable, CompileError> {
        let table = self.tables.pop().unwrap();
        let mut analyzer = SymbolAnalyzer::default();
        analyzer.analyze_symbol_table(&table)?;
        Ok(table)
    }
}
