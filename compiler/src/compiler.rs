use zoker_bytecode::bytecode::{CodeObject, Instruction, Register, Constant, NameScope, RegisterType};
use crate::symbol_table::{SymbolTable, make_symbol_tables};
use zoker_parser::ast;
use crate::error::CompileError;
use indexmap::map::IndexMap;
use num_bigint::{BigUint, BigInt};
use crate::compiler::Scope::Contract;
use std::ops::Add;

type CompileResult<T> = Result<T, CompileError>;

pub fn compile_program(ast: ast::Program) -> CompileResult<Vec<CodeObject>> {
    let symbol_table = make_symbol_tables(&ast)?;
    let mut compiler = Compiler::new();
    compiler.compile_program(&ast, symbol_table)?;
    Ok(compiler.code_blocks)
}

#[derive(Debug, Clone)]
struct CompileContext {
    namespace: Vec<String>,
    scope: NameScope,
    register_number: u32,
}

impl CompileContext {
    fn new() -> Self {
        CompileContext {
            namespace: vec![],
            register_number: 0,
            scope: NameScope::Global,
        }
    }
}

struct Compiler {
    context: CompileContext,
    code_blocks: Vec<CodeObject>,
    symbol_tables: Vec<SymbolTable>,
    registers: Vec<IndexMap<String, Register>>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            context: CompileContext::new(),
            code_blocks: vec![],
            symbol_tables: vec![],
            registers: vec![],
        }
    }

    fn compile_program(&mut self, ast: &ast::Program, symbol_table: SymbolTable) -> CompileResult<()> {
        self.symbol_tables.push(symbol_table);
        self.code_blocks.push(CodeObject::new(String::from("$"), NameScope::Global, 0));
        match ast {
            ast::Program::GlobalStatements(stmts) => self.compile_global_statements(stmts)?,
        }
        Ok(())
    }

    fn compile_global_statements(&mut self, statements: &[ast::Statement]) -> CompileResult<()> {
        for statement in statements {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_block(&mut self, stmt: &ast::Statement) -> CompileResult<()> {
        if let ast::StatementType::CompoundStatement {
            statements,
            return_value,
        } = &stmt.node
        {
            for stmt in statements {
                self.compile_statement(stmt)?;
            }
            if let Some(returns) = return_value {
                self.compile_expression(returns)?;
            }
        }
        Ok(())
    }

    fn add_code_block(&mut self, name: String) {
        let namespace = self.context.namespace.iter().fold(String::new(), |prev, name| prev.add(name).add("::"));
        let code_block = CodeObject::new(
            namespace.add(&name),
            self.context.scope.clone(),
            self.context.register_number,
        );
        self.code_blocks.push(code_block);
    }

    fn pop_code_block(&mut self) {
        let code_block = self.code_blocks.pop().unwrap();
        self.code_blocks.last_mut().unwrap().sub_code.push(code_block);
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.code_blocks.last_mut().unwrap().instructions.push(instruction);
    }

    fn enter_scope(&mut self, scope: NameScope) -> NameScope {
        let tables = self.symbol_tables.last_mut().unwrap();
        let table = tables.sub_tables.remove(0);
        self.symbol_tables.push(table);

        self.registers.push(Default::default());
        let prev_scope = self.context.scope.clone();
        self.context.scope = scope;

        prev_scope
    }

    fn exit_scope(&mut self, scope: NameScope) {
        self.context.scope = scope;
        self.symbol_tables.pop();
        self.registers.pop();
    }

    fn compile_statement(&mut self, statement: &ast::Statement) -> CompileResult<Option<Register>> {
        match &statement.node {
            ast::StatementType::FunctionStatement {
                function_name: func,
                parameters: params,
                statement: stmt,
            } => {
                let func_name = func.node.identifier_name().unwrap();
                let prev_scope = self.enter_scope(NameScope::Local);
                self.add_code_block(func_name.clone());

                // TODO: For Function call, Parameter should be processed.
                self.compile_expression(params)?;
                self.compile_statement(stmt)?;

                self.pop_code_block();
                self.exit_scope(prev_scope);
                Ok(None)
            }
            ast::StatementType::ContractStatement {
                contract_name: name,
                members,
            } => {
                let contract_name = name.node.identifier_name().unwrap();
                self.context.namespace.push(contract_name);
                let prev_scope = self.enter_scope(NameScope::Contract);
                self.compile_statement(members);
                self.exit_scope(prev_scope);
                self.context.namespace.pop();
                Ok(None)
            }
            ast::StatementType::InitializerStatement {
                variable_type,
                data_location: _,
                variable,
                default,
            } => {
                let name = variable.node.identifier_name().unwrap();
                // TODO: Check Data location here, not in symbol table.
                let default_value = if let Some(var) = default {
                    let reg = self.compile_expression(var);
                    reg?.value
                } else {
                    self.default_constant(variable_type)
                };
                self.register_name(&name, default_value);
                if let Some(value) = default {
                    self.registers.last_mut().get_mut(&name).unwrap();
                }
                Ok(None)
            }
            ast::StatementType::CompoundStatement {
                statements,
                return_value,
            } => {
                // TODO: Only Compound Statement Return Register.
                //  Should we change compound statement to expression?
                let prev_scope = self.enter_scope(NameScope::Local);
                for statement in statements {
                    self.compile_statement(statement);
                }
                let ret = if let Some(returns) = return_value {
                    self.compile_expression(returns)
                } else {
                    None
                };
                self.exit_scope(prev_scope);
                ret
            }
            ast::StatementType::MemberStatement {
                statements: members,
            } => {
                for member in members {
                    self.compile_statement(member);
                }
                Ok(None)
            }
            ast::StatementType::Expression {
                expression: expr,
            } => self.compile_expression(expr),
        }
    }

    fn compile_expression(&mut self, expression: &ast::Expression) -> CompileResult<Register> {
        match &expression.node {
            ast::ExpressionType::AssignExpression {
                left,
                operator,
                right,
            } => {
                let left = self.compile_expression(left)?;
                let right = self.compile_expression(right)?;

                // TODO: Correctly place Instruction.
                match operator {
                    ast::Operator::Assign => {},
                    ast::Operator::BitAndAssign => {},
                    ast::Operator::BitXorAssign => {},
                    ast::Operator::BitOrAssign => {},
                    ast::Operator::LShiftAssign => {},
                    ast::Operator::RShiftAssign => {},
                    ast::Operator::AddAssign => {},
                    ast::Operator::SubAssign => {},
                    ast::Operator::MulAssign => {},
                    ast::Operator::DivAssign => {},
                    ast::Operator::ModAssign => {},
                    _ => unreachable!(),
                };
                Ok(left)
            }
            ast::ExpressionType::TernaryExpression {
                condition: cond,
                expr1,
                expr2,
            } => {
                let condition= self.compile_expression(cond)?;

            }
            ast::ExpressionType::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left = self.compile_expression(left)?;
                let right = self.compile_expression(right)?;

                match operator {
                    ast::Operator::Add => {},
                    ast::Operator::Sub => {},
                    ast::Operator::Mul => {},
                    ast::Operator::Div => {},
                    ast::Operator::Mod => {},
                    ast::Operator::Pow => {},
                    ast::Operator::Lt => {},
                    ast::Operator::Le => {},
                    ast::Operator::Gt => {},
                    ast::Operator::Ge => {},
                    ast::Operator::Eq => {},
                    ast::Operator::NotEq => {},
                    ast::Operator::And => {},
                    ast::Operator::Or => {},
                    ast::Operator::BitAnd => {},
                    ast::Operator::BitXor => {},
                    ast::Operator::BitOr => {},
                    ast::Operator::LShift => {},
                    ast::Operator::RShift => {},
                    _ => unreachable!(),
                }
            }
            ast::ExpressionType::FunctionCallExpression {
                function_name: name,
                arguments: args,
            } => {
                // TODO: For Function call, Function definition should be processed.
            }
            ast::ExpressionType::IfExpression {
                condition: cond,
                if_statement: if_stmt,
                else_statement: else_stmt,
            } => {
                // TODO: Correctly place Instruction.
                let condition= self.compile_expression(cond)?;
                let if_statement = self.compile_statement(if_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    let else_statement = self.compile_statement(else_stmt)?;
                }
            }
            ast::ExpressionType::ForEachExpression {
                iterator: iter,
                vector: vec,
                statement: stmt,
                else_statement: else_stmt,
            } => {
                // TODO: Correctly place Instruction.
                //  Must check vector size. (Vector size MUST be static.)
            }
            ast::ExpressionType::UnaryExpression {
                operator,
                expression: expr,
            } => {
                let expression = self.compile_expression(expr)?;
                match operator {
                    ast::Operator::Plus => {},
                    ast::Operator::Minus => {},
                    ast::Operator::Not => {},
                    ast::Operator::PrefixPlusPlus => {},
                    ast::Operator::PrefixMinusMinus => {},
                    ast::Operator::PostfixPlusPlus => {},
                    ast::Operator::PostfixMinusMinus => {},
                    _ => unreachable!(),
                }
            }
            ast::ExpressionType::Parameters { parameters: params } => {

            }
            ast::ExpressionType::Arguments { arguments: args } => {

            }
            ast::ExpressionType::Number { value } => {
                // TODO: Check Already used Number.
                let register = Register::new(
                    value.to_str_radix(10),
                    RegisterType::Constant,
                    self.context.register_number,
                    Constant::Uint { limit: 8, val: value.clone() }
                );
                self.context.register_number += 1;
                Ok(register)
            }
            ast::ExpressionType::Identifier { value } => {
                let mut reg = None;
                for register_map in self.registers.iter().rev() {
                    if let Some(register) = register_map.get(value) {
                        reg = Some(register);
                        break;
                    }
                }
                if let Some(register) = reg {
                    Ok(register.clone())
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn register_name(&mut self, name: &String, default: Constant) {
        self.registers.last_mut().unwrap().insert(
            name.clone(),
            Register::new(
                name.clone(),
                RegisterType::Variable,
                self.context.register_number,
                default,
            ),
        );
        self.context.register_number += 1;
    }

    fn default_constant(&mut self, typ: &ast::Type) -> Constant {
        match typ {
            ast::Type::Uint256 => Constant::Uint { limit: 8, val: Default::default() },
            ast::Type::Int256 => Constant::Int { limit: 8, val: Default::default() },
            ast::Type::Bytes32 => Constant::Bytes { limit: 5, val: vec![0, 0, 0, 0] },
            ast::Type::Bool => Constant::Bool { limit: 1, val: false },
            ast::Type::Bytes => Constant::Bytes { limit: 5, val: vec![0, 0, 0, 0] },
            ast::Type::String => Constant::String { limit: 2, val: "".to_string() },
            ast::Type::Address => Constant::Address { limit: 8, val: Default::default() },
        }
    }
}
