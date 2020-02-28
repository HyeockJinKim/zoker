use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod location;
lalrpop_mod!(
    #[allow(clippy::all)]
    pub zoker
);
