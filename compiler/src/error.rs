use zoker_parser::location::Location;

#[derive(Debug, PartialEq)]
pub struct CompileError {
    pub error: CompileErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum CompileErrorType {
    SyntaxError(String),
    TypeError(String),
}
