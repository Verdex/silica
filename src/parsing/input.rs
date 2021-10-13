
use std::iter::Enumerate;
use std::iter::Peekable;

use super::lexeme::Lexeme;

#[macro_export]
macro_rules! input {
    () => {
        Peekable<Enumerate<impl Iterator<Item=impl Clone> + Clone>>
    };

    ($t:ty) => {
        Peekable<Enumerate<impl Iterator<Item=$t> + Clone>>
    };
}

pub use input;

pub fn to_lex_input(s : &str) -> input!(char) {
    s.chars().collect::<Vec<char>>().into_iter().enumerate().peekable()
}

pub fn to_parse_input(lexemes : Vec<Lexeme>) -> input!(Lexeme) {
    lexemes.into_iter().enumerate().peekable()
}