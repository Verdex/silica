
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
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool;
    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize>;
}

struct SymbolLexer {}

impl Lexer for SymbolLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((index, c)) => c.is_alphabetic() || *c == '_',
            None => false,
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        let mut letters = vec![];

        let mut rp = input.restore_point();
        let mut v = input.next();

        match v {
            Some((_, v)) if v.is_alphabetic() || v == '_' => letters.push(v),
            Some((index, _)) => { input.restore(rp); return Err(index) },
            _ => return Err(0), // TODO get index? (end of input)
        }

        rp = input.restore_point();
        v = input.next();

        loop {
            match v {
                Some((_, v)) if v.is_alphanumeric() || v == '_' => letters.push(v),
                Some((_, _)) => { input.restore(rp); break },
                _ => break, 
            }
            rp = input.restore_point();
            v = input.next();
        }

        if letters[0].is_uppercase() {
            Ok(Lexeme::UpperCaseSymbol(letters.into_iter().collect::<String>()))
        }
        else {
            Ok(Lexeme::LowerCaseSymbol(letters.into_iter().collect::<String>()))
        }
    }
}

struct BoolLexer {}

impl Lexer for BoolLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((index, c)) => *c == 't' || *c == 'f',
            None => false,
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        // TODO
        Err(0)
    }
}

struct IntegerLexer {}

impl Lexer for IntegerLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((index, c)) => c.is_digit(10),
            None => false,
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        let mut digits = vec![];

        let mut rp = input.restore_point();
        let mut v = input.next();

        loop {
            match v {
                Some((_, v)) if v.is_digit(10) => digits.push(v),
                Some((_, _)) => { input.restore(rp); break},
                _ => break,
            }
            rp = input.restore_point();
            v = input.next();
        }

        Ok(Lexeme::Integer(digits.into_iter().collect::<String>().parse::<i64>().expect("parse::<i64>() failure")))
    }
}

// TODO Decimal Lexer
// TODO sci notation lexer (?)
// TODO Number lexer
// TODO add indices to lexemes


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symbol_lexer_should_lex_upper_case_symbol() {
        let lex = SymbolLexer {};
        let mut input = Input { cs : "Blah__123".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("SymbolLexer should lex symbol");

        match r {
            Lexeme::UpperCaseSymbol(s) => assert_eq!( s, "Blah__123" ),
            _ => panic!("expected upper case symbol"),
        }
    }

    #[test]
    fn symbol_lexer_should_lex_lower_case_symbol() {
        let lex = SymbolLexer {};
        let mut input = Input { cs : "blah__123".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("SymbolLexer should lex symbol");

        match r {
            Lexeme::LowerCaseSymbol(s) => assert_eq!( s, "blah__123" ),
            _ => panic!("expected upper case symbol"),
        }
    }

    #[test]
    fn integer_lexer_should_lex_standard_integer() {
        let lex = IntegerLexer {};
        let mut input = Input { cs : "1234".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("IntegerLexer should lex standard integer");

        assert_eq!( r, Lexeme::Integer(1234) );
    }

    #[test]
    fn integer_lexer_should_not_conume_ending_input() {
        let lex = IntegerLexer {};
        let mut input = Input { cs : "1234s".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("IntegerLexer should lex standard integer");

        assert_eq!( r, Lexeme::Integer(1234) );
        assert!( matches!( input.next(), Some((_, 's'))) );
    }

}