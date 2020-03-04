use crate::ast;
use std::convert::TryInto;

pub struct PrintAST {
    repr: String,
    size: u32,
    children: Vec<PrintAST>,
}
//
// fn program_to_str(node: &ast::Program) -> PrintAST {
//     let child = node.to_child();
//     match child.node {
//         ast::ExpressionType => expr_to_str(&child.node)
//     }
// }

pub fn expr_to_str(node: &ast::ExpressionType) -> PrintAST {
    match node {
        ast::ExpressionType::Number { value: v } => {
            let s = format!("[ Number : {} ] ", v);
            let size = s.len().try_into().unwrap();
            PrintAST {
                repr: s,
                size,
                children: vec![],
            }
        }
        ast::ExpressionType::UnaryExpression {
            operator: op,
            expression: expr,
        } => {
            let operator = operator_to_str(&op);
            let expression = expr_to_str(&expr.node);
            let s = String::from("[ UnaryExpression ] ");
            let size = u32::max(s.len().try_into().unwrap(), operator.size + expression.size);
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
                size: u32::max(size, left.size + operator.size + right.size),
                children: vec![left, operator, right],
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
    }
}

impl PrintAST {
    pub fn str(&self) -> &String {
        &self.repr
    }
    pub fn child_nodes(&self) -> String {
        let mut str = String::new();
        let children = &self.children;
        for child in children {
            let node = &child.repr;
            str.push_str(&node);
        }
        str
    }
}
