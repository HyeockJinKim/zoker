use zoker_parser::location::Location;

#[derive(Debug)]
pub struct CompileError {
    pub error: CompileErrorType,
    pub location: Location,
}

#[derive(Debug)]
pub enum CompileErrorType {
    SyntaxError(String),
}
