use pest_derive::*;
#[derive(Parser)]
#[grammar = "./syntax/grammar.pest"]
pub struct TkLang {}