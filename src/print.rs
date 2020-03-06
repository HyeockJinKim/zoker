use crate::ast;
use std::convert::TryInto;

pub struct PrintAST {
    repr: String,
    size: usize,
    children: Vec<PrintAST>,
}

// fn program_to_str(node: &ast::Program) -> PrintAST {
//     let child = node.to_child();
//     match child.node {
//         ast::ExpressionType => expr_to_str(&child.node)
//     }
// }

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
            let s = String::from("[ AssignExpression ] ");
            let size = s.len().try_into().unwrap();

            PrintAST {
                repr: s,
                size: usize::max(size, left.size + operator.size + right.size),
                children: vec![left, operator, right],
            }
        }
        ast::ExpressionType::BinaryExpression {
            left: l,
            operator: op,
            right: r,
        } => {
            let left = expr_to_str(&l.node);
            let operator = operator_to_str(op);
            let right = expr_to_str(&r.node);
            let s = String::from("[ BinaryExpression ] ");
            let size = s.len().try_into().unwrap();

            PrintAST {
                repr: s,
                size: usize::max(size, left.size + operator.size + right.size),
                children: vec![left, operator, right],
            }
        }
        ast::ExpressionType::UnaryExpression {
            operator: op,
            expression: expr,
        } => {
            let operator = operator_to_str(&op);
            let expression = expr_to_str(&expr.node);
            let s = String::from("[ UnaryExpression ] ");
            let size = usize::max(s.len(), operator.size + expression.size);
            let children = if &ast::Operator::PostfixPlusPlus == op
                || &ast::Operator::PostfixMinusMinus == op
            {
                vec![operator, expression]
            } else {
                vec![expression, operator]
            };

            PrintAST {
                repr: s,
                size,
                children,
            }
        }
        ast::ExpressionType::Number { value: v } => {
            let s = format!("[ Number : {} ] ", v);
            let size = s.len();
            PrintAST {
                repr: s,
                size,
                children: vec![],
            }
        }
        ast::ExpressionType::Identifier { value: v } => {
            let s = format!("[ Identifier : {} ] ", v);
            let size = s.len();
            PrintAST {
                repr: s,
                size,
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
            children: vec![],
        },
        ast::Operator::Sub => PrintAST {
            repr: String::from("[ binop : - ] "),
            size: 14,
            children: vec![],
        },
        ast::Operator::Mul => PrintAST {
            repr: String::from("[ binop : * ] "),
            size: 14,
            children: vec![],
        },
        ast::Operator::Div => PrintAST {
            repr: String::from("[ binop : / ] "),
            size: 14,
            children: vec![],
        },
        ast::Operator::Mod => PrintAST {
            repr: String::from("[ binop : % ] "),
            size: 14,
            children: vec![],
        },
        ast::Operator::Pow => PrintAST {
            repr: String::from("[ op : pow ] "),
            size: 13,
            children: vec![],
        },
        ast::Operator::Plus => PrintAST {
            repr: String::from("[ uop : + ] "),
            size: 12,
            children: vec![],
        },
        ast::Operator::Minus => PrintAST {
            repr: String::from("[ uop : - ] "),
            size: 12,
            children: vec![],
        },
        ast::Operator::PrefixPlusPlus => PrintAST {
            repr: String::from("[ pre-op : ++ ] "),
            size: 16,
            children: vec![],
        },
        ast::Operator::PrefixMinusMinus => PrintAST {
            repr: String::from("[ pre-op : -- ] "),
            size: 16,
            children: vec![],
        },
        ast::Operator::PostfixPlusPlus => PrintAST {
            repr: String::from("[ post-op : ++ ] "),
            size: 17,
            children: vec![],
        },
        ast::Operator::PostfixMinusMinus => PrintAST {
            repr: String::from("[ post-op : -- ] "),
            size: 17,
            children: vec![],
        },
        ast::Operator::Assign => PrintAST {
            repr: String::from("[ assign-op : = ] "),
            size: 18,
            children: vec![],
        },
        ast::Operator::BitAndAssign => PrintAST {
            repr: String::from("[ assign-op : &= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::BitOrAssign => PrintAST {
            repr: String::from("[ assign-op : |= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::XorAssign => PrintAST {
            repr: String::from("[ assign-op : ^= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::LShiftAssign => PrintAST {
            repr: String::from("[ assign-op : <<= ] "),
            size: 20,
            children: vec![],
        },
        ast::Operator::RShiftAssign => PrintAST {
            repr: String::from("[ assign-op : >>= ] "),
            size: 20,
            children: vec![],
        },
        ast::Operator::AddAssign => PrintAST {
            repr: String::from("[ assign-op : += ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::SubAssign => PrintAST {
            repr: String::from("[ assign-op : -= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::MulAssign => PrintAST {
            repr: String::from("[ assign-op : *= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::DivAssign => PrintAST {
            repr: String::from("[ assign-op : /= ] "),
            size: 19,
            children: vec![],
        },
        ast::Operator::ModAssign => PrintAST {
            repr: String::from("[ assign-op : %= ] "),
            size: 19,
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
                        children: vec![],
                    };
                    children.push(empty_ast);
                } else {
                    for child in node.children.clone() {
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

    fn node_str(&self) -> String {
        if self.repr.is_empty() {
            " ".repeat(self.size)
        } else {
            let margin_size = (self.size - self.repr.len()) / 2;
            let margin = " ".repeat(margin_size);
            format!("{}{}{}", margin, &self.repr, margin)
        }
    }
}

impl Clone for PrintAST {
    fn clone(&self) -> PrintAST {
        PrintAST {
            repr: String::from(self.str()),
            size: self.size,
            children: self.children.clone(),
        }
    }
}
