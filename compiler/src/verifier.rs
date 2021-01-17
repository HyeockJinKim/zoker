/*use crate::symbol::{symbol_to_string, Contract, Function, Operation, OperationType};

// pub fn rewrite_verifier(contracts: Vec<Contract>) -> RewriterResult<()> {
//     let mut verifier = Verifier::new();
//     for contract in contracts {
//         verifier.rewrite_contract(&contract);
//     }
//     Ok(())
// }

struct Verifier {
    code: String,
    tab: u32,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier {
            code: String::new(),
            tab: 0,
        }
    }

    pub fn rewrite_contract(&mut self, contract: &Contract) {
        self.write_line("contract ");
        self.write_line(contract.name.as_str());
        self.write_line(" {\n");
        self.tab += 1;
        // TODO: Don't consider member variable.
        for function in &contract.functions {
            self.rewrite_function(function);
        }
        self.write_line("}\n");
    }

    pub fn rewrite_function(&mut self, function: &Function) {
        self.write_line("function _");
        self.write(function.name.as_str());
        self.write("(");
        // for param in &function.params {
        //     TODO: MUST CONSUME ZKP PROOF.
        // }
        self.write(") returns (");
        for ret in &function.returns {
            self.write(symbol_to_string(&ret.symbol_type));
        }
        self.write(") {\n");
        for operation in &function.operations {
            self.write_line("");
            self.rewrite_operation(&operation);
            self.write(";");
        }
        self.write_line("}");
    }

    fn rewrite_operation(&mut self, operation: &Operation) {
        match &operation.operation {
            OperationType::Add { left, right } => {
                self.write("_add(");
                self.rewrite_operation(left);
                self.write(",");
                self.rewrite_operation(right);
                self.write(")");
            }
            OperationType::Sub { left, right } => {
                self.write("_sub(");
                self.rewrite_operation(left);
                self.write(",");
                self.rewrite_operation(right);
                self.write(")");
            }
            OperationType::Mul { left, right } => {
                self.write("_mul(");
                self.rewrite_operation(left);
                self.write(",");
                self.rewrite_operation(right);
                self.write(")");
            }
            OperationType::Assign { left, right } => {
                // TODO:
                self.rewrite_operation(left);
                self.write(" = ");
                self.rewrite_operation(right);
            }
            OperationType::For { stmts, .. } => {
                // self.write();
                for stmt in stmts {
                    self.rewrite_operation(stmt);
                }
            }
            OperationType::If { cond, .. } => {
                // TODO:
                self.write("_if(");
                self.rewrite_operation(cond);
                self.write(",");
                // self.rewrite_operation(right);
                self.write(")");
            }
            OperationType::Else { cond, .. } => {
                self.write("_if(not(");
                self.rewrite_operation(cond);
                self.write("),");
                // self.rewrite_operation(right);
                self.write(")");
            }
            OperationType::Return { ret } => {
                self.write("return ");
                self.rewrite_operation(ret);
            }
            OperationType::Call { func, .. } => {
                self.write("_");
                self.write(func.as_str());
                self.write("(");
            }
            OperationType::Symbol { symbol } => {
                // TODO: Use IKOS Variable
                self.write(symbol.name.as_str());
            }
            OperationType::Constant { value } => {
                // TODO: Use IKOS Variable
                self.write(value.to_string().as_str());
            }
            OperationType::Nop => {
                // Do nothing
            }
        }
    }

    fn write(&mut self, line: &str) {
        self.code.push_str(line);
    }

    fn write_line(&mut self, line: &str) {
        for _ in 0..self.tab {
            self.code.push_str("    ");
        }
        self.code.push_str(line);
    }
}
*/
