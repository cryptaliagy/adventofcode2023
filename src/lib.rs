use lalrpop_util::lalrpop_mod;

pub mod day_one;
pub mod day_two;
pub(crate) mod day_two_ast;
lalrpop_mod!(pub(crate) day_two_parser);
pub mod day_five;
pub mod day_four;
pub mod day_six;
