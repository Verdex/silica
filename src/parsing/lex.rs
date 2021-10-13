
use std::iter::Enumerate;
use std::iter::Peekable; 

use super::lexeme::Lexeme;
use super::input::{self, input};


pub fn lex(s : &str) -> Vec<Lexeme> {
    let input = input::to_lex_input(s);

    //let lexers = [];

    vec![]
}

trait Lexer {
    fn usable(&self, input : &mut input!(char)) -> bool;
    fn lex(&self, input : &mut input!(char)) -> Lexeme;
}

struct NumberLexer {}

impl Lexer for NumberLexer {
    fn usable(&self, input : &mut input!(char)) -> bool {
        false
    }

    fn lex(&self, input : &mut input!(char)) -> Lexeme {
        Lexeme::Integer(0)
    }
}
