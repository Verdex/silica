
use std::str::CharIndices;
use std::iter::Peekable; 

use super::lexeme::Lexeme;


pub fn lex(s : &str) -> Vec<Lexeme> {


    let lexers : [&dyn Lexer; 2] = [ &BoolLexer{}, &IntegerLexer{} ];

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

struct JunkLexer {}

impl Lexer for JunkLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((_, c)) if c.is_whitespace() => return true,
            Some((_, '/'))=> { },
            _ => return false,
        }

        let rp = input.restore_point();

        input.next();

        match input.next() {
            Some((_, '*')) => { input.restore(rp); true },
            _ => { input.restore(rp); false },
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        let mut comment = 0;

        loop {
            if comment > 0 {
                match input.next() {
                    Some((_, '*')) => {
                        match input.next() {
                            Some((_, '/')) => { comment-=1; },
                            Some((_, _)) => { },
                            None => return Err(0), // TODO end of file
                        }
                    },
                    Some((_, '/')) => {
                        match input.next() {
                            Some((_, '*')) => { comment+=1; },
                            Some((_, _)) => { },
                            None => return Err(0), // TODO end of file
                        }
                    }
                    Some((_, _)) => { },
                    None => return Err(0),  // TODO end of file
                }
            }
            else {
                match input.peek() {
                    Some((_, c)) if c.is_whitespace() => { input.next(); },
                    Some((_, '/')) => {
                        let rp = input.restore_point();
                        input.next();
                        match input.peek() {
                            Some((_, '*')) => { input.next(); comment+=1 },
                            _ => { input.restore(rp); return Ok(Lexeme::Junk); },
                        }
                    },
                    _ => return Ok(Lexeme::Junk),
                }
            }
        }
    }
}

struct SymbolLexer {}

impl Lexer for SymbolLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((_, c)) => c.is_alphabetic() || *c == '_',
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
            Some((_, c)) => *c == 't' || *c == 'f',
            None => false,
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        let symbol_lexer = SymbolLexer {};

        let rp = input.restore_point();

        match symbol_lexer.lex(input) {
            Ok(Lexeme::LowerCaseSymbol(lexeme)) if lexeme == "true" => Ok(Lexeme::Bool(true)),
            Ok(Lexeme::LowerCaseSymbol(lexeme)) if lexeme == "false" => Ok(Lexeme::Bool(false)),
            _ => { input.restore(rp); Err(0) }, // TODO need a way to grab index
        }
    }
}

struct IntegerLexer {}

impl Lexer for IntegerLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((_, c)) => c.is_digit(10),
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


struct StringLexer {}

impl Lexer for StringLexer {
    fn usable<'a>(&self, input : &mut Input<'a>) -> bool {
        match input.peek() {
            Some((_, c)) => *c == '"',
            None => false,
        }
    }

    fn lex<'a>(&self, input : &mut Input<'a>) -> Result<Lexeme, usize> {
        let mut cs = vec![];

        let rp = input.restore_point();
        let mut v = input.next();

        match v {
            Some((_, '"')) => { },
            _ => { input.restore(rp); return Err(0); }, // TODO index   
        }

        v = input.next();

        loop {
            match v {
                Some((_, '"')) => break,
                Some((_, '\\')) => {
                    let escape = input.next();
                    match escape {
                        Some((_, 't')) => cs.push('\t'),
                        Some((_, 'n')) => cs.push('\n'),
                        Some((_, 'r')) => cs.push('\r'),
                        Some((_, '\\')) => cs.push('\\'),
                        Some((_, '"')) => cs.push('"'),
                        Some((index, _)) => return Err(index),
                        None => return Err(0), // TODO end of file
                    }
                },
                Some((_, v)) => cs.push(v),
                None => return Err(0), // TODO end of file
            }
            v = input.next();
        }

        Ok(Lexeme::String(cs.into_iter().collect::<String>()))
    }
}

// TODO Decimal Lexer
// TODO sci notation lexer (?)
// TODO Number lexer
// TODO add indices to lexemes
// TODO punctuation lexers
// TODO junk lexers


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn junk_lexer_usable_should_leave_input_alone_for_lonely_slash() {
        let lex = JunkLexer {};
        let mut input = Input { cs : "/ *".char_indices().peekable() };

        let result = lex.usable(&mut input);

        assert_eq!( result, false );

        assert!( matches!( input.next(), Some((_, '/') ) ) );
    }

    #[test]
    fn junk_lexer_should_lex_whitespace() {
        let lex = JunkLexer {};
        let mut input = Input { cs : "    \t \r \n a".char_indices().peekable() };

        let result = lex.lex(&mut input);

        assert_eq!( result, Ok(Lexeme::Junk) );

        assert!( matches!( input.next(), Some((_, 'a') ) ) );
    }

    #[test]
    fn junk_lexer_should_lex_whitespace_and_comment() {
        let lex = JunkLexer {};
        let mut input = Input { cs : " /* blah blah blah */ a".char_indices().peekable() };

        let result = lex.lex(&mut input);

        assert_eq!( result, Ok(Lexeme::Junk) );

        assert!( matches!( input.next(), Some((_, 'a') ) ) );
    }
    
    #[test]
    fn junk_lexer_should_lex_nested_comment() {
        let lex = JunkLexer {};
        let mut input = Input { cs : " /* /* /* blah blah blah */ */ */ a".char_indices().peekable() };

        let result = lex.lex(&mut input);

        assert_eq!( result, Ok(Lexeme::Junk) );

        assert!( matches!( input.next(), Some((_, 'a') ) ) );
    }

    #[test]
    fn junk_lexer_should_lex_almost_but_not_quite_comment_end() {
        let lex = JunkLexer {};
        let mut input = Input { cs : " /* * / */ a".char_indices().peekable() };

        let result = lex.lex(&mut input);

        assert_eq!( result, Ok(Lexeme::Junk) );

        assert!( matches!( input.next(), Some((_, 'a') ) ) );
    }

    #[test]
    fn junk_lexer_should_leave_lonely_slash() {
        let lex = JunkLexer {};
        let mut input = Input { cs : " / ".char_indices().peekable() };

        let result = lex.lex(&mut input);

        assert_eq!( result, Ok(Lexeme::Junk) );

        assert!( matches!( input.next(), Some((_, '/') ) ) );
    }

    #[test]
    fn string_lexer_should_lex_string() {
        let lex = StringLexer {};
        let mut input = Input { cs : r#""this is a \t \n \r \" \\ string""#.char_indices().peekable() };

        let r = lex.lex(&mut input).expect("StringLexer should lex string");

        match r {
            Lexeme::String(s) => assert_eq!(s, "this is a \t \n \r \" \\ string"),
            _ => panic!("Expected string"),
        }

        assert_eq!( input.next(), None );
    }

    #[test]
    fn bool_lexer_should_lex_true() {
        let lex = BoolLexer {};
        let mut input = Input { cs : "true".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("BoolLexer should lex bool");

        assert_eq!( r, Lexeme::Bool(true) );
    }

    #[test]
    fn bool_lexer_should_lex_false() {
        let lex = BoolLexer {};
        let mut input = Input { cs : "false".char_indices().peekable() };

        let r = lex.lex(&mut input).expect("BoolLexer should lex bool");

        assert_eq!( r, Lexeme::Bool(false) );
    }

    #[test]
    fn bool_lexer_should_not_consume_boolish_symbol() {
        let lex = BoolLexer {};
        let mut input = Input { cs : "trueish".char_indices().peekable() };

        let r = lex.lex(&mut input);

        assert!( matches!( r, Err(_) ) );

        assert!( matches!( input.next(), Some((_, 't')) ) );
    }

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