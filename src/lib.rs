// I hate this lint too :)
#![allow(unused_parens)]

use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod error;
pub mod location;
pub mod print;
lalrpop_mod!(
    #[allow(clippy::all)]
    pub zoker_parser
);
