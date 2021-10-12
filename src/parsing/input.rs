use std::str::CharIndices;

pub struct Input<'a> {
    cs : CharIndices<'a>,
    total_length : usize,
}

pub struct RestorePoint<'a> {
    cs : CharIndices<'a>,
}

impl<'a> Input<'a> {

    pub fn new(s : &'a str) -> Input<'a> {
        Input { cs: s.char_indices()
              , total_length: s.len()
              }
    }

    pub fn restore_point(&self) -> RestorePoint<'a> {
        RestorePoint { cs: self.cs.clone()
                     }
    }

    pub fn restore(&mut self, rp : RestorePoint<'a>) {
        self.cs = rp.cs;
    }

    pub fn get_char(&mut self) -> Result<(usize, char), usize> {
        match self.cs.next() {
            Some(c) => Ok(c),
            None => Err(self.total_length),
        }
    }

    pub fn peek(&mut self) -> Result<(usize, char), usize> {
        let rp = self.restore_point();

        match self.get_char() {
            it @ Ok(_) => { self.restore(rp); it },
            it @ Err(_) => it,
        }
    }

    pub fn exact<'b>(&mut self, s : &'b str) -> Result<(usize, usize, &'b str), usize> {
        let (start, _) = self.peek()?;

        let mut n = self.cs.clone();

        let mut end = start;
        for c in s.chars() {
            match n.next() {
                Some((index, target)) if c == target => { end = index }, 
                Some((index, _)) => return Err(index),
                None => return Err(self.total_length),
            }
        }

        self.cs = n;
        Ok((start, end, s))
    }

    pub fn take_while(&mut self, f : impl Fn(char) -> bool) -> Result<(usize, usize, String), usize> {
        let (start, mut c) = self.peek()?;
        let mut end = start;
        let mut cs = vec![]; 

        while f(c) {
            cs.push(c);
            let _ = self.get_char().expect("Input::take_while fails because get_char fails after successful peek");

            match self.peek() {
                Ok((index, cha)) => { end = index; c = cha; },
                Err(_) => break,
            }
        }

        Ok((start, end, cs.into_iter().collect()))
    }

    pub fn when(&mut self, f : impl Fn(char) -> bool) -> Result<(usize, char), usize> {
        let (index, c) = self.peek()?;

        if f(c) {
            let _ = self.get_char().expect("Input::when fails because get_char fails after successful peek");
            Ok((index, c))
        }
        else {
            Err(index)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_char_should_return_char_and_index() {
        let mut input = Input::new("string");

        let c = input.get_char().expect("Should be able to get 's'");
        assert_eq!( (0, 's'), c );

        let c = input.get_char().expect("Should be able to get 't'");
        assert_eq!( (1, 't'), c );

        let c = input.get_char().expect("Should be able to get 'r'");
        assert_eq!( (2, 'r'), c );

        let c = input.get_char().expect("Should be able to get 'i'");
        assert_eq!( (3, 'i'), c );

        let c = input.get_char().expect("Should be able to get 'n'");
        assert_eq!( (4, 'n'), c );

        let c = input.get_char().expect("Should be able to get 'g'");
        assert_eq!( (5, 'g'), c );
    }

    #[test]
    fn get_char_returns_failure_index() {
        let mut input = Input::new("string");

        let _ = input.get_char().expect("Should be able to get 's'");
        let _ = input.get_char().expect("Should be able to get 't'");
        let _ = input.get_char().expect("Should be able to get 'r'");
        let _ = input.get_char().expect("Should be able to get 'i'");
        let _ = input.get_char().expect("Should be able to get 'n'");
        let _ = input.get_char().expect("Should be able to get 'g'");

        let v = input.get_char();

        assert_eq!( Err(6), v );
    }

    #[test]
    fn exact_failure_should_not_change_index() {
        let mut input = Input::new("string");

        let result = input.exact("yy");

        assert!( matches!( result, Err(_) ) );

        let result = input.exact("string");

        assert!( matches!( result, Ok(_) ) );
    }

    #[test]
    fn exact_success_should_change_index() {
        let mut input = Input::new("string");

        let result = input.exact("st");

        assert!( matches!( result, Ok(_) ) );

        let result = input.exact("ring");

        assert!( matches!( result, Ok(_) ) );
    }

    #[test]
    fn exact_returns_index_on_failure() {
        let mut input = Input::new("string");

        let v = input.exact("cat");

        assert_eq!( Err(0), v );
    }

    #[test]
    fn exact_returns_target_string() {
        let mut input = Input::new("string");

        let _ = input.exact("st");

        let result = input.exact("ring");

        match result {
            Ok((_, _, s)) => assert_eq!( "ring", s ),
            _ => assert!(false),
        }
    }

    #[test]
    fn exact_returns_correct_start_index() {
        let mut input = Input::new("string");

        let _ = input.exact("st");

        let result = input.exact("ring");

        match result {
            Ok((s, _, _)) => assert_eq!( 2, s ),
            _ => assert!(false),
        }
    }

    #[test]
    fn exact_returns_correct_end_index() {
        let mut input = Input::new("string");

        let _ = input.exact("st");

        let result = input.exact("ring");

        match result {
            Ok((_, e, _)) => assert_eq!( 5, e ),
            _ => assert!(false),
        }
    }
    
    #[test]
    fn peek_error_returns_index() {
        let mut input = Input::new("string");

        let _ = input.exact("string");

        let v = input.peek();

        assert_eq!( Err(6), v );
    }

    #[test]
    fn peek_success_returns_index_and_value() {
        let mut input = Input::new("string");

        let _ = input.exact("st");

        let v = input.peek();

        assert_eq!( Ok((2, 'r')), v );
    }

    #[test]
    fn peek_success_does_not_increase_index() {
        let mut input = Input::new("string");

        let _ = input.exact("st");

        let v = input.peek();
        assert_eq!( Ok((2, 'r')), v );

        let v = input.peek();
        assert_eq!( Ok((2, 'r')), v );
    }

    #[test]
    fn take_while_failure_returns_index() {
        let mut input = Input::new("string");

        let _ = input.exact("string");

        let result = input.take_while(|_| true);

        assert_eq!( Err(6), result );
    }

    #[test]
    fn take_while_success_returns_string() {
        let mut input = Input::new("string");

        let result = input.take_while(|x| x != 'i');

        match result {
            Ok((_, _, s)) => assert_eq!( "str", s ),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_returns_start_index() {
        let mut input = Input::new("string");

        let _ = input.get_char();

        let result = input.take_while(|x| x != 'i');

        match result {
            Ok((s, _, _)) => assert_eq!( 1, s ),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_returns_end_index() {
        let mut input = Input::new("string");

        let _ = input.get_char();

        let result = input.take_while(|x| x != 'i');

        match result {
            Ok((_, e, _)) => assert_eq!( 3, e ),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_leaves_final_char_alone() {
        let mut input = Input::new("string");

        let _ = input.take_while(|x| x != 'i');

        let result = input.get_char();

        match result {
            Ok((_, c)) => assert_eq!( 'i', c),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_with_zero_chars_returns_empty_string() {
        let mut input = Input::new("string");

        let result = input.take_while(|_| false);

        match result {
            Ok((_, _, s)) => assert_eq!( "", s),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_with_zero_chars_returns_start_index() {
        let mut input = Input::new("string");

        let result = input.take_while(|_| false);

        match result {
            Ok((s, _, _)) => assert_eq!( 0, s),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn take_while_success_with_zero_chars_returns_end_index() {
        let mut input = Input::new("string");

        let result = input.take_while(|_| false);

        match result {
            Ok((_, e, _)) => assert_eq!( 0, e),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn when_failure_returns_end_index() {
        let mut input = Input::new("string");

        let _ = input.exact("string");

        let result = input.when(|_| true);

        assert_eq!( Err(6), result );
    }

    #[test]
    fn when_failure_returns_target_index() {
        let mut input = Input::new("string");

        let result = input.when(|_| false);

        assert_eq!( Err(0), result );
    }

    #[test]
    fn when_failure_leaves_char_alone() {
        let mut input = Input::new("string");

        let _ = input.when(|_| false);

        let result = input.get_char();

        assert_eq!( Ok((0, 's')), result );
    }

    #[test]
    fn when_success_returns_index() {
        let mut input = Input::new("string");

        let _ = input.get_char();

        let result = input.when(|c| c == 't');

        match result {
            Ok((i, _)) => assert_eq!( 1, i ),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn when_success_returns_char() {
        let mut input = Input::new("string");

        let _ = input.get_char();

        let result = input.when(|c| c == 't');

        match result {
            Ok((_, c)) => assert_eq!( 't', c ),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn when_success_moves_index() {
        let mut input = Input::new("string");

        let _ = input.when(|c| c == 's');

        let result = input.get_char();

        assert_eq!( Ok((1, 't')), result );
    }
}