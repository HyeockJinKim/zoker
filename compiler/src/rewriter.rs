use crate::error::{RewriteError, RewriteErrorType};
use crate::symbol::{
    specifier_to_location, token_to_type, Contract, Function, Operation, OperationType, Symbol,
    SymbolLocation, SymbolType,
};
use indexmap::map::IndexMap;
use zoker_parser::ast;
use zoker_parser::ast::{ExpressionType, Operator, StatementType};
use zoker_parser::location::Location;

pub type RewriterResult<T> = Result<T, RewriteError>;

pub fn rewrite_program(ast: &ast::Program) -> RewriterResult<Vec<Contract>> {
    let mut rewriter = Rewriter::new();
    rewriter.compile_program(ast)?;
    Ok(rewriter.contracts)
}

#[derive(Debug, Clone)]
struct RewriterContext {
    public_map: IndexMap<String, Symbol>,
    private_map: IndexMap<String, Symbol>,
    public_num: u32,
    private_num: u32,
    operations: Vec<Vec<Operation>>,
}

impl RewriterContext {
    fn new() -> Self {
        RewriterContext {
            public_map: Default::default(),
            private_map: Default::default(),
            public_num: 0,
            private_num: 0,
            operations: vec![],
        }
    }

    fn add_variable(&mut self, name: String, symbol: Symbol, is_private: bool) {
        if is_private {
            self.private_map.insert(name, symbol);
        } else {
            self.public_map.insert(name, symbol);
        }
    }

    fn variable_num(&mut self, is_private: bool) -> u32 {
        if is_private {
            let num = self.private_num;
            self.private_num += 1;
            num
        } else {
            let num = self.public_num;
            self.public_num += 1;
            num
        }
    }
}

struct Rewriter {
    context: RewriterContext,
    pub contracts: Vec<Contract>,
}

impl Rewriter {
    fn new() -> Self {
        Rewriter {
            context: RewriterContext::new(),
            contracts: vec![],
        }
    }

    fn compile_program(&mut self, ast: &ast::Program) -> RewriterResult<()> {
        match ast {
            ast::Program::GlobalStatements(stmts) => self.compile_statements(stmts)?,
        }
        Ok(())
    }

    fn compile_statements(&mut self, statements: &[ast::Statement]) -> RewriterResult<()> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &ast::Statement) -> RewriterResult<()> {
        match &statement.node {
            StatementType::FunctionStatement {
                function_name,
                parameters,
                statement,
                returns,
            } => {
                self.context = RewriterContext::new();
                let name = function_name.node.identifier_name().unwrap();
                let params = self.compile_param_symbols(parameters)?;
                let ret = if let Some(return_type) = returns {
                    self.compile_param_symbols(return_type)?
                } else {
                    vec![]
                };
                let function = Function::new(name, params, ret);
                self.current_contract().add_function(function);

                self.enter_scope();
                self.compile_statement(statement)?;
                self.add_operation_all();
            }
            StatementType::ContractStatement {
                contract_name,
                members,
            } => {
                let name = contract_name.node.identifier_name().unwrap();
                let contract = Contract::new(name);
                self.add_contract(contract);
                self.compile_statement(members)?;
            }
            StatementType::InitializerStatement {
                variable_type,
                is_private,
                data_location,
                variable,
                default,
            } => {
                let typ = token_to_type(variable_type);
                let loc = if let Some(location) = data_location {
                    specifier_to_location(location)
                } else {
                    SymbolLocation::Unknown
                };
                if let Some(identifier) = variable {
                    let name = identifier.node.identifier_name().unwrap();
                    self.init_variable(name.clone(), typ, loc, *is_private);
                    let symbol = self.get_variable(&name);
                    let left = Operation::new_symbol(symbol);

                    let operation = if let Some(var) = default {
                        self.compile_expression(var)?;
                        let right = self.pop_operation();
                        Operation::new(OperationType::Assign {
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    } else {
                        left
                    };
                    self.push_operation(operation);
                } else {
                    self.push_operation(Operation::new_symbol(Symbol::new_type_symbol(typ)))
                }
            }
            StatementType::CompoundStatement {
                statements,
                return_value,
            } => {
                self.enter_scope();
                self.compile_statements(statements)?;
                if let Some(returns) = return_value {
                    self.compile_expression(returns)?;
                }
                let operations = self.exit_scope();
                self.push_operation_all(operations);
            }
            StatementType::MemberStatement { statements } => {
                self.compile_statements(statements)?;
            }
            StatementType::ReturnStatement { ret } => {
                if let Some(returns) = ret {
                    self.compile_expression(returns)?;
                    let ret = self.pop_operation();
                    let operation = Operation::new(OperationType::Return { ret: Box::new(ret) });
                    self.push_operation(operation);
                }
            }
            StatementType::Expression { expression } => {
                self.compile_expression(expression)?;
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: &ast::Expression) -> RewriterResult<()> {
        match &expression.node {
            ExpressionType::AssignExpression {
                left,
                operator,
                right,
            } => {
                self.compile_expression(left)?;
                let left = self.pop_operation();
                self.compile_expression(right)?;
                let right = self.pop_operation();
                let op = match operator {
                    Operator::Assign => OperationType::Assign {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    _ => {
                        return Err(RewriteError {
                            error: RewriteErrorType::UnsupportedError,
                            location: Location::new(0, 0),
                        })
                    }
                };
                let operation = Operation::new(op);
                self.push_operation(operation);
            }
            ExpressionType::BinaryExpression {
                left,
                operator,
                right,
            } => {
                self.compile_expression(left)?;
                let left = self.pop_operation();
                self.compile_expression(right)?;
                let right = self.pop_operation();
                let op = match operator {
                    Operator::Add => OperationType::Add {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    Operator::Sub => OperationType::Sub {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    _ => {
                        return Err(RewriteError {
                            error: RewriteErrorType::UnsupportedError,
                            location: Location::new(0, 0),
                        })
                    }
                };
                let operation = Operation::new(op);
                self.push_operation(operation);
            }
            ExpressionType::FunctionCallExpression {
                function_name,
                arguments,
            } => {
                let name = function_name.node.identifier_name().unwrap();
                let args = self.compile_params(arguments)?;
                let operation = Operation::new_call(name, args);
                self.push_operation(operation);
            }
            ExpressionType::IfExpression {
                condition,
                if_statement,
                else_statement,
            } => {
                self.compile_expression(condition)?;
                let cond = self.pop_operation();
                self.enter_scope();
                self.compile_statement(if_statement)?;
                let stmts = self.exit_scope();
                let operation = Operation::new(OperationType::If {
                    cond: Box::new(cond.clone()),
                    stmts,
                });
                self.push_operation(operation);
                if let Some(else_stmt) = else_statement {
                    self.enter_scope();
                    self.compile_statement(else_stmt)?;
                    let stmts = self.exit_scope();
                    self.push_operation(Operation::new(OperationType::Else {
                        cond: Box::new(cond),
                        stmts,
                    }));
                };
            }
            ExpressionType::ForEachExpression {
                iterator,
                vector,
                statement,
                else_statement,
            } => {
                let iter_name = iterator.node.identifier_name().unwrap();
                self.iter_variable(iter_name.as_str());
                let symbol = self.get_variable(iter_name.as_str());

                self.compile_expression(vector)?;
                let vector_operation = self.pop_operation();
                self.enter_scope();
                self.compile_statement(statement)?;
                let stmts = self.exit_scope();
                let operation = Operation::new(OperationType::For {
                    iter: Box::new(Operation::new_symbol(symbol)),
                    vector: Box::new(vector_operation),
                    stmts,
                });
                self.push_operation(operation);
                if let Some(else_stmt) = else_statement {
                    return Err(RewriteError {
                        error: RewriteErrorType::UnsupportedError,
                        location: else_stmt.location,
                    });
                }
            }
            ExpressionType::Number { value } => {
                self.push_operation(Operation::new(OperationType::Constant {
                    value: value.clone(),
                }));
            }
            ExpressionType::Identifier { value } => {
                let symbol = self.get_variable(value);
                self.push_operation(Operation::new_symbol(symbol));
            }
            ExpressionType::Parameters { .. } => {
                return Err(RewriteError {
                    error: RewriteErrorType::Unreachable,
                    location: Location::new(0, 0),
                });
            }
            ExpressionType::Arguments { .. } => {
                return Err(RewriteError {
                    error: RewriteErrorType::Unreachable,
                    location: Location::new(0, 0),
                });
            }
            ExpressionType::UnaryExpression { .. } => {
                return Err(RewriteError {
                    error: RewriteErrorType::UnsupportedError,
                    location: Location::new(0, 0),
                });
            }
            ExpressionType::Tuple { .. } => {
                return Err(RewriteError {
                    error: RewriteErrorType::UnsupportedError,
                    location: Location::new(0, 0),
                });
            }
            ExpressionType::TernaryExpression { .. } => {
                return Err(RewriteError {
                    error: RewriteErrorType::UnsupportedError,
                    location: Location::new(0, 0),
                });
            }
        }
        Ok(())
    }

    fn compile_params(&mut self, expression: &ast::Expression) -> RewriterResult<Vec<Operation>> {
        match &expression.node {
            ExpressionType::Parameters { parameters } => {
                self.enter_scope();
                for parameter in parameters {
                    self.compile_statement(parameter)?;
                }
                Ok(self.exit_scope())
            }
            ExpressionType::Arguments { arguments } => {
                self.enter_scope();
                for argument in arguments {
                    self.compile_expression(argument)?;
                }
                Ok(self.exit_scope())
            }
            _ => Err(RewriteError {
                error: RewriteErrorType::Unreachable,
                location: Location::new(0, 0),
            }),
        }
    }

    fn compile_param_symbols(&mut self, params: &ast::Expression) -> RewriterResult<Vec<Symbol>> {
        Ok(self
            .compile_params(params)?
            .iter()
            .map(|operation| operation.as_symbol().unwrap())
            .collect::<Vec<Symbol>>())
    }

    fn add_contract(&mut self, contract: Contract) {
        self.contracts.push(contract);
    }

    fn current_contract(&mut self) -> &mut Contract {
        self.contracts.last_mut().unwrap()
    }

    fn enter_scope(&mut self) {
        self.context.operations.push(vec![]);
    }

    fn exit_scope(&mut self) -> Vec<Operation> {
        self.context.operations.pop().unwrap()
    }

    fn push_operation_all(&mut self, operations: Vec<Operation>) {
        self.context
            .operations
            .last_mut()
            .unwrap()
            .extend(operations);
    }

    fn push_operation(&mut self, operation: Operation) {
        self.context.operations.last_mut().unwrap().push(operation);
    }

    fn pop_operation(&mut self) -> Operation {
        self.context.operations.last_mut().unwrap().pop().unwrap()
    }

    fn add_operation_all(&mut self) {
        let operation = self.exit_scope();
        self.current_contract().add_operation_all(operation);
    }

    fn init_variable(
        &mut self,
        name: String,
        typ: SymbolType,
        loc: SymbolLocation,
        is_private: bool,
    ) {
        let symbol = Symbol::new(
            name.clone(),
            self.context.variable_num(is_private),
            typ,
            loc,
            is_private,
        );
        self.context.add_variable(name, symbol, is_private);
    }

    fn iter_variable(&mut self, name: &str) {
        let symbol = Symbol::new(
            name.to_string(),
            self.context.variable_num(false),
            SymbolType::Uint256,
            SymbolLocation::Memory,
            false,
        );
        self.context.add_variable(name.to_string(), symbol, false);
    }

    fn get_variable(&self, name: &str) -> Symbol {
        if let Some(symbol) = self.context.private_map.get(name) {
            symbol.clone()
        } else {
            self.context.public_map.get(name).unwrap().clone()
        }
    }
}
