
use std::str::CharIndices;
use std::iter::Peekable; 

use super::lexeme::Lexeme;


pub fn lex(s : &str) -> Vec<Lexeme> {


    let lexers : [&dyn Lexer; 1] = [ &IntegerLexer{} ];

    vec![]
}

struct Input<'a> {
    cs : Peekable<CharIndices<'a>>
}

impl<'a> Input<'a> {
    fn restore_point(&self) -> Peekable<CharIndices<'a>> {
        self.cs.clone()
    }

    fn restore(&mut self, rp : Peekable<CharIndices<'a>>) {
        self.cs = rp;
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.cs.next()
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.cs.peek()
    }
}

trait Lexer {
    fn usable(&self, input : &mut Peekable<CharIndices>) -> bool;
    fn lex(&self, input : &mut Peekable<CharIndices>) -> Lexeme;
}

struct IntegerLexer {}

impl Lexer for IntegerLexer {
    fn usable(&self, input : &mut Peekable<CharIndices>) -> bool {
        match input.peek() {
            Some((index, c)) => c.is_digit(10),
            None => false,
        }
    }

    fn lex(&self, input : &mut Peekable<CharIndices>) -> Lexeme {
        let mut digits = vec![];

        let v = input.next();

        loop {
            match v {
                Some((_, v)) if v.is_digit(10) => digits.push(v),
                _ => break
            }
        }

        Lexeme::Integer(digits.into_iter().collect::<String>().parse::<i64>().expect("parse::<i64>() failure"))
    }
}
