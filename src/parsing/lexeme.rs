
#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    Junk,
    RParen,
    LParen,
    LowerCaseSymbol(String),
    UpperCaseSymbol(String),
    Bool(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Lexeme {
    pub fn start_index(&self) -> usize {
        0
    }

    pub fn end_index(&self) -> usize {
        0
    }
}