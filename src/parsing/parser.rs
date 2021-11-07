
use std::iter::{Peekable, Enumerate};

use super::lexeme::Lexeme;
use super::ast::Ast;


pub trait Parser {
    fn usable(&self, input : &mut Peekable<Enumerate<std::vec::IntoIter<Lexeme>>>) -> bool;
    fn parse(&self, input : &mut Peekable<Enumerate<std::vec::IntoIter<Lexeme>>>) -> Result<Ast, (usize, usize)>;
}