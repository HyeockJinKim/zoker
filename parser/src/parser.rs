use crate::ast;
use crate::error::ParseError;
use crate::lexer;
use crate::zok;

macro_rules! do_lalr_parsing {
    ($input: expr, $parser: ident) => {{
        let lxr = lexer::make_tokenizer($input);
        match zok::$parser::new().parse(lxr) {
            Err(err) => Err(ParseError::from(err)),
            Ok(top) => Ok(top),
        }
    }};
}

pub fn parse_expression(source: &str) -> Result<ast::Expression, ParseError> {
    do_lalr_parsing!(source, ExpressionParser)
}

pub fn parse_statement(source: &str) -> Result<ast::Statement, ParseError> {
    do_lalr_parsing!(source, StatementParser)
}

pub fn parse_program(source: &str) -> Result<ast::Program, ParseError> {
    do_lalr_parsing!(source, ProgramParser)
}
