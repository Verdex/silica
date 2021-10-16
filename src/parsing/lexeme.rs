
#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    LowerCaseSymbol(String),
    UpperCaseSymbol(String),
    Bool(bool),
    Integer(i64),
}