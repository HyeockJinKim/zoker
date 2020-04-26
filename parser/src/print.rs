use crate::ast;
use std::ops::Add;

pub struct PrintAST {
    repr: String,
    size: usize,
    left_margin: usize,
    right_margin: usize,
    children: Vec<PrintAST>,
}

fn get_margin(repr_size: usize, children_size: usize) -> (usize, usize) {
    if repr_size > children_size {
        let margin = repr_size - children_size;
        let right = margin / 2;
        (margin - right, right)
    } else {
        (0, 0)
    }
}

pub fn program_to_str(node: &ast::Program) -> PrintAST {
    match node {
        ast::Program::GlobalStatements(stmts) => {
            let children = stmts
                .iter()
                .map(|stmt| stmt_to_str(&stmt.node))
                .collect::<Vec<_>>();
            let repr = String::from("[ Program ] ");
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
    }
}

fn name_from_identifier(identifier: &ast::Expression) -> Option<String> {
    if let ast::ExpressionType::Identifier { value } = &identifier.node {
        Some(value.clone())
    } else {
        None
    }
}

pub fn stmt_to_str(node: &ast::StatementType) -> PrintAST {
    match node {
        ast::StatementType::FunctionStatement {
            function_name: id,
            parameters: params,
            statement: stmt,
        } => {
            let name = name_from_identifier(&id).unwrap();
            let repr = String::from("[ Function Statement: ")
                .add(name.as_str())
                .add(" ] ");
            let parameters = expr_to_str(&params.node);
            let statement = stmt_to_str(&stmt.node);
            let children = vec![parameters, statement];
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::StatementType::ContractStatement {
            contract_name: name,
            members: stmts,
        } => {
            let name = name_from_identifier(&name).unwrap();
            let repr = String::from("[ Contract Statement: ")
                .add(name.as_str())
                .add(" ] ");
            let member = stmt_to_str(&stmts.node);
            let children = vec![member];
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::StatementType::InitializerStatement {
            variable_type: var_type,
            data_location: loc,
            variable: var_name,
            default: default_val,
        } => {
            let repr = String::from("[ Initializer Statement ] ");
            let variable_type = type_to_str(&var_type);
            let mut children = vec![variable_type];
            if let Some(location) = loc {
                let data_location = specifier_to_str(location);
                children.push(data_location);
            }
            let variable_name = expr_to_str(&var_name.node);
            children.push(variable_name);
            if let Some(default) = default_val {
                let default_value = expr_to_str(&default.node);
                children.push(default_value);
            }
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::StatementType::CompoundStatement {
            statements: stmts,
            return_value: returns,
        } => {
            let mut children = stmts
                .iter()
                .map(|stmt| stmt_to_str(&stmt.node))
                .collect::<Vec<_>>();
            let repr = String::from("[ Compound Statement ] ");
            if let Some(return_value) = returns {
                children.push(expr_to_str(&return_value.node))
            }
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::StatementType::MemberStatement { statements: stmts } => {
            let children = stmts
                .iter()
                .map(|stmt| stmt_to_str(&stmt.node))
                .collect::<Vec<_>>();
            let repr = String::from("[ Member Statement ] ");
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::StatementType::Expression { expression: expr } => expr_to_str(&expr.node),
    }
}

pub fn expr_to_str(node: &ast::ExpressionType) -> PrintAST {
    match node {
        ast::ExpressionType::AssignExpression {
            left: l,
            operator: op,
            right: r,
        } => {
            let left = expr_to_str(&l.node);
            let operator = operator_to_str(op);
            let right = expr_to_str(&r.node);
            let repr = String::from("[ AssignExpression ] ");
            let children_size = left.size + operator.size + right.size;
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![left, operator, right],
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::TernaryExpression {
            condition: cond,
            expr1,
            expr2,
        } => {
            let condition = expr_to_str(&cond.node);
            let expression1 = expr_to_str(&expr1.node);
            let expression2 = expr_to_str(&expr2.node);
            let repr = String::from("[ TernaryExpression ] ");
            let children_size = condition.size + expression1.size + expression2.size;
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![condition, expression1, expression2],
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::BinaryExpression {
            left: l,
            operator: op,
            right: r,
        } => {
            let left = expr_to_str(&l.node);
            let operator = operator_to_str(op);
            let right = expr_to_str(&r.node);
            let repr = String::from("[ BinaryExpression ] ");
            let children_size = left.size + operator.size + right.size;
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![left, operator, right],
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::FunctionCallExpression {
            function_name: id,
            arguments: args,
        } => {
            let function_name = expr_to_str(&id.node);
            let arguments = expr_to_str(&args.node);
            let repr = String::from("[ Function Call Expression ] ");
            let children_size = function_name.size + arguments.size;
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![function_name, arguments],
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::ForEachExpression {
            iterator: iter,
            vector: vec,
            statement: stmt,
            else_statement: else_stmt,
        } => {
            let iterator = expr_to_str(&iter.node);
            let vector = expr_to_str(&vec.node);
            let statement = stmt_to_str(&stmt.node);
            let repr;
            let size;
            let children;
            if let Some(else_statement) = else_stmt {
                let else_statement = stmt_to_str(&else_statement.node);
                repr = String::from("[ For-else Expression ] ");
                let children_size =
                    iterator.size + vector.size + statement.size + else_statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![iterator, vector, statement, else_statement];
            } else {
                repr = String::from("[ For Expression ] ");
                let children_size = iterator.size + vector.size + statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![iterator, vector, statement];
            }

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::IfExpression {
            condition: cond,
            if_statement: if_stmt,
            else_statement: else_stmt,
        } => {
            let condition = expr_to_str(&cond.node);
            let if_statement = stmt_to_str(&if_stmt.node);
            let repr;
            let size;
            let children;
            if let Some(else_statement) = else_stmt {
                let else_statement = stmt_to_str(&else_statement.node);
                repr = String::from("[ If-else Expression ] ");
                let children_size = condition.size + if_statement.size + else_statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![condition, if_statement, else_statement];
            } else {
                repr = String::from("[ If Expression ] ");
                let children_size = condition.size + if_statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![condition, if_statement];
            }

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::UnaryExpression {
            operator: op,
            expression: expr,
        } => {
            let operator = operator_to_str(&op);
            let expression = expr_to_str(&expr.node);
            let repr = String::from("[ UnaryExpression ] ");
            let children_size = operator.size + expression.size;
            let size = usize::max(repr.len(), children_size);
            let children = if &ast::Operator::PostfixPlusPlus == op
                || &ast::Operator::PostfixMinusMinus == op
            {
                vec![operator, expression]
            } else {
                vec![expression, operator]
            };

            PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            }
        }
        ast::ExpressionType::Parameters { parameters: params } => {
            let children = params
                .iter()
                .map(|param| stmt_to_str(&param.node))
                .collect::<Vec<_>>();
            let repr = String::from("[ Parameters Expression ] ");
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::Arguments { arguments: args } => {
            let children = args
                .iter()
                .map(|arg| expr_to_str(&arg.node))
                .collect::<Vec<_>>();
            let repr = String::from("[ Arguments Expression ] ");
            let children_size = children.iter().fold(0, |v, child| v + child.size);
            let size = usize::max(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children,
            };
            ast.add_children_margin();
            ast
        }
        ast::ExpressionType::Number { value: v } => {
            let repr = format!("[ Number : {} ] ", v);
            let size = repr.len();
            PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![],
            }
        }
        ast::ExpressionType::Identifier { value: v } => {
            let repr = format!("[ Identifier : {} ] ", v);
            let size = repr.len();
            PrintAST {
                repr,
                size,
                left_margin: 0,
                right_margin: 0,
                children: vec![],
            }
        }
    }
}

pub fn operator_to_str(node: &ast::Operator) -> PrintAST {
    match node {
        ast::Operator::Add => PrintAST {
            repr: String::from("[ binop : + ] "),
            size: 14,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Sub => PrintAST {
            repr: String::from("[ binop : - ] "),
            size: 14,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Mul => PrintAST {
            repr: String::from("[ binop : * ] "),
            size: 14,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Div => PrintAST {
            repr: String::from("[ binop : / ] "),
            size: 14,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Mod => PrintAST {
            repr: String::from("[ binop : % ] "),
            size: 14,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Pow => PrintAST {
            repr: String::from("[ op : pow ] "),
            size: 13,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Plus => PrintAST {
            repr: String::from("[ uop : + ] "),
            size: 12,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Minus => PrintAST {
            repr: String::from("[ uop : - ] "),
            size: 12,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Not => PrintAST {
            repr: String::from("[ uop : ! ] "),
            size: 12,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::PrefixPlusPlus => PrintAST {
            repr: String::from("[ pre-op : ++ ] "),
            size: 16,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::PrefixMinusMinus => PrintAST {
            repr: String::from("[ pre-op : -- ] "),
            size: 16,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::PostfixPlusPlus => PrintAST {
            repr: String::from("[ post-op : ++ ] "),
            size: 17,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::PostfixMinusMinus => PrintAST {
            repr: String::from("[ post-op : -- ] "),
            size: 17,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Assign => PrintAST {
            repr: String::from("[ assign-op : = ] "),
            size: 18,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitAndAssign => PrintAST {
            repr: String::from("[ assign-op : &= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitXorAssign => PrintAST {
            repr: String::from("[ assign-op : ^= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitOrAssign => PrintAST {
            repr: String::from("[ assign-op : |= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::LShiftAssign => PrintAST {
            repr: String::from("[ assign-op : <<= ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::RShiftAssign => PrintAST {
            repr: String::from("[ assign-op : >>= ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::AddAssign => PrintAST {
            repr: String::from("[ assign-op : += ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::SubAssign => PrintAST {
            repr: String::from("[ assign-op : -= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::MulAssign => PrintAST {
            repr: String::from("[ assign-op : *= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::DivAssign => PrintAST {
            repr: String::from("[ assign-op : /= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::ModAssign => PrintAST {
            repr: String::from("[ assign-op : %= ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Lt => PrintAST {
            repr: String::from("[ compare-op : < ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Gt => PrintAST {
            repr: String::from("[ compare-op : > ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Le => PrintAST {
            repr: String::from("[ compare-op : <= ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Ge => PrintAST {
            repr: String::from("[ compare-op : >= ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Eq => PrintAST {
            repr: String::from("[ compare-op : == ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::NotEq => PrintAST {
            repr: String::from("[ compare-op : != ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::And => PrintAST {
            repr: String::from("[ logical-op : && ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::Or => PrintAST {
            repr: String::from("[ logical-op : || ] "),
            size: 20,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitAnd => PrintAST {
            repr: String::from("[ bit-op : & ] "),
            size: 15,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitXor => PrintAST {
            repr: String::from("[ bit-op : ^ ] "),
            size: 15,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::BitOr => PrintAST {
            repr: String::from("[ bit-op : | ] "),
            size: 15,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::LShift => PrintAST {
            repr: String::from("[ shift-op : << ] "),
            size: 18,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Operator::RShift => PrintAST {
            repr: String::from("[ shift-op : >> ] "),
            size: 18,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
    }
}

fn specifier_to_str(node: &ast::Specifier) -> PrintAST {
    match node {
        ast::Specifier::Memory => PrintAST {
            repr: String::from("[ specifier : memory ] "),
            size: 23,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Specifier::Storage => PrintAST {
            repr: String::from("[ specifier : storage ] "),
            size: 24,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
    }
}

pub fn type_to_str(node: &ast::Type) -> PrintAST {
    match node {
        ast::Type::Uint256 => PrintAST {
            repr: String::from("[ type : uint256 ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::Int256 => PrintAST {
            repr: String::from("[ type : int256 ] "),
            size: 18,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::Bytes32 => PrintAST {
            repr: String::from("[ type : bytes32 ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::Bool => PrintAST {
            repr: String::from("[ type : bool ] "),
            size: 16,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::Bytes => PrintAST {
            repr: String::from("[ type : bytes ] "),
            size: 17,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::String => PrintAST {
            repr: String::from("[ type : string ] "),
            size: 18,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
        ast::Type::Address => PrintAST {
            repr: String::from("[ type : address ] "),
            size: 19,
            left_margin: 0,
            right_margin: 0,
            children: vec![],
        },
    }
}

impl PrintAST {
    pub fn str(&self) -> &String {
        &self.repr
    }
    pub fn print_ast(&self) -> String {
        let mut str = String::new();
        let mut nodes = vec![self.clone()];
        loop {
            let mut is_empty = false;
            let mut children = vec![];
            for node in &nodes {
                str.push_str(&node.node_str());
                if node.children.is_empty() {
                    let empty_ast = PrintAST {
                        repr: String::new(),
                        size: node.size,
                        left_margin: node.left_margin,
                        right_margin: node.right_margin,
                        children: vec![],
                    };
                    children.push(empty_ast);
                } else {
                    let child_vec = node.children.clone();
                    for child in child_vec {
                        is_empty = true;
                        children.push(child);
                    }
                }
            }
            str.push_str("\n");
            if !is_empty {
                break;
            }
            nodes = children;
        }

        str
    }

    fn add_children_margin(&mut self) {
        if !self.children.is_empty() {
            let children_size = self.children.iter().fold(0, |v, child| v + child.size);
            let (left_margin, right_margin) = get_margin(self.repr.len(), children_size);

            if let Some(child) = self.children.first_mut() {
                child.left_margin += left_margin;
            }
            if let Some(child) = self.children.last_mut() {
                child.right_margin += right_margin;
            }
        }
    }

    fn get_margin(&self) -> (String, String) {
        let margin_size = self.size - self.repr.len();
        let left_size = margin_size / 2;
        let right_size = margin_size - left_size;
        let left_margin = " ".repeat(left_size + self.left_margin);
        let right_margin = " ".repeat(right_size + self.right_margin);
        (left_margin, right_margin)
    }

    fn node_str(&self) -> String {
        if self.repr.is_empty() {
            let left = " ".repeat(self.left_margin);
            let right = " ".repeat(self.right_margin);
            format!("{}{}{}", left, " ".repeat(self.size), right)
        } else {
            let (left, right) = self.get_margin();
            format!("{}{}{}", left, &self.repr, right)
        }
    }
}

impl Clone for PrintAST {
    fn clone(&self) -> PrintAST {
        PrintAST {
            repr: String::from(self.str()),
            size: self.size,
            left_margin: self.left_margin,
            right_margin: self.right_margin,
            children: self.children.clone(),
        }
    }
}
