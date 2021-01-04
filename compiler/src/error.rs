use zoker_parser::location::Location;

#[derive(Debug, PartialEq)]
pub struct RewriteError {
    pub error: RewriteErrorType,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum RewriteErrorType {
    SyntaxError(String),
    TypeError(String),
    UnsupportedError,
    Unreachable,
}
