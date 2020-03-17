use crate::ast;

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

pub fn stmt_to_str(node: &ast::StatementType) -> PrintAST {
    match node {
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
            let (left_margin, right_margin) = get_margin(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
                children: vec![left, operator, right],
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
            let (left_margin, right_margin) = get_margin(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
                children: vec![left, operator, right],
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
            let children_size;
            let size;
            let children;
            if let Some(else_statement) = else_stmt {
                let else_statement = stmt_to_str(&else_statement.node);
                repr = String::from("[ If-else Expression ] ");
                children_size = condition.size + if_statement.size + else_statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![condition, if_statement, else_statement];
            } else {
                repr = String::from("[ If Expression ] ");
                children_size = condition.size + if_statement.size;
                size = usize::max(repr.len(), children_size);
                children = vec![condition, if_statement];
            }
            let (left_margin, right_margin) = get_margin(repr.len(), children_size);

            let mut ast = PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
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
            let (left_margin, right_margin) = get_margin(repr.len(), children_size);

            PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
                children,
            }
        }
        ast::ExpressionType::Number { value: v } => {
            let repr = format!("[ Number : {} ] ", v);
            let size = repr.len();
            let left_margin = 0;
            let right_margin = 0;
            PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
                children: vec![],
            }
        }
        ast::ExpressionType::Identifier { value: v } => {
            let repr = format!("[ Identifier : {} ] ", v);
            let size = repr.len();
            let left_margin = 0;
            let right_margin = 0;
            PrintAST {
                repr,
                size,
                left_margin,
                right_margin,
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
