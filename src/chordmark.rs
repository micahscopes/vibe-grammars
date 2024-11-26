use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "elements.pest"]
#[grammar = "chordmark.pest"]
pub struct Parser;