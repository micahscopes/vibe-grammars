use pest_derive::Parser;
// #[cfg(debug_assertions)]
// const _GRAMMAR: &'static str = include_str!("chords.pest");

#[derive(Parser)]
#[grammar = "elements.pest"]
pub struct Parser;