use crate::error::{CompileError, CompileErrorType};
use crate::symbol::SymbolType;
use indexmap::map::IndexMap;
use zoker_parser::ast;

type TypeResult = Result<(), CompileError>;

pub struct ContractSignature {
    pub name: String,
    pub variables: IndexMap<String, SymbolType>,
    pub functions: Vec<FunctionSignature>,
}

pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<SymbolType>,
    pub returns: Vec<SymbolType>,
}

struct TypePreChecker {
    signatures: Vec<ContractSignature>,
}

pub fn type_check(program: &ast::Program) -> Result<Vec<ContractSignature>, CompileError> {
    let mut checker = TypePreChecker::new();
    checker.type_check(program)?;
    Ok(checker.signatures)
}

pub fn get_type(typ: &ast::Type) -> SymbolType {
    match typ {
        ast::Type::String => SymbolType::String,
        ast::Type::Uint256 => SymbolType::Uint256,
        ast::Type::Int256 => SymbolType::Int256,
        ast::Type::Bytes32 => SymbolType::Bytes32,
        ast::Type::Bool => SymbolType::Bool,
        ast::Type::Bytes => SymbolType::Bytes,
        ast::Type::Address => SymbolType::Address,
    }
}

impl TypePreChecker {
    fn new() -> Self {
        TypePreChecker { signatures: vec![] }
    }

    fn type_check(&mut self, program: &ast::Program) -> TypeResult {
        match program {
            ast::Program::GlobalStatements(stmts) => {
                for stmt in stmts {
                    self.check_statement(stmt)?;
                }
            }
        }

        Ok(())
    }

    fn check_statement(&mut self, statement: &ast::Statement) -> TypeResult {
        match &statement.node {
            ast::StatementType::FunctionStatement {
                function_name,
                parameters,
                ..
            } => {
                let name = function_name.node.identifier_name().unwrap();
                let params = self.get_params(parameters)?;
                self.signatures
                    .last_mut()
                    .unwrap()
                    .functions
                    .push(FunctionSignature {
                        name,
                        params,
                        // TODO: should add return type check!
                        returns: vec![],
                    });
                Ok(())
            }
            ast::StatementType::ContractStatement {
                contract_name,
                members,
            } => {
                let name = contract_name.node.identifier_name().unwrap();
                self.signatures.push(ContractSignature {
                    name,
                    variables: Default::default(),
                    functions: vec![],
                });
                self.check_statement(members)?;
                Ok(())
            }
            ast::StatementType::MemberStatement { statements } => {
                for statement in statements {
                    self.check_statement(statement)?;
                }
                Ok(())
            }
            ast::StatementType::InitializerStatement {
                variable_type,
                variable,
                ..
            } => {
                let name = variable.node.identifier_name().unwrap();
                let var_type = get_type(variable_type);
                self.signatures
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, var_type);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn get_params(&self, init: &ast::Expression) -> Result<Vec<SymbolType>, CompileError> {
        match &init.node {
            ast::ExpressionType::Parameters { parameters } => {
                let mut params = vec![];
                for param in parameters {
                    params.push(self.get_init_type(param)?);
                }

                Ok(params)
            }
            _ => Err(CompileError {
                error: CompileErrorType::TypeError(String::from(
                    "Function parameter Must be Parameters type",
                )),
                location: init.location,
            }),
        }
    }

    fn get_init_type(&self, statement: &ast::Statement) -> Result<SymbolType, CompileError> {
        if let ast::StatementType::InitializerStatement { variable_type, .. } = &statement.node {
            Ok(get_type(variable_type))
        } else {
            Err(CompileError {
                error: CompileErrorType::TypeError(String::from(
                    "Parameter Must be init statement",
                )),
                location: statement.location,
            })
        }
    }
}
