// Basic matchers

use std::fmt;
use {PatternElement, MatchState, CompareResult};


pub struct Null; // "Null Matcher" is used when a matcher requires a "next"

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl PatternElement for Null {
    #[allow(unused_variables)]
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        CompareResult::Match(0)
    }
}


pub struct Start;

impl fmt::Display for Start {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "^")
    }
}

impl PatternElement for Start {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let n = state.pos();
        if n < isize::max_value() as usize {
            CompareResult::Match(n as isize)
        } else {
            CompareResult::Match(isize::max_value())
        }
    }
}


pub struct End;

impl fmt::Display for End {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$")
    }
}

impl PatternElement for End {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        assert!(state.pos() <= state.max_pos());
        let n = state.max_pos() - state.pos();
        if n < isize::max_value() as usize {
            CompareResult::Match(n as isize)
        } else {
            CompareResult::Match(isize::max_value())
        }
    }
}


pub struct Any;

impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".")
    }
}

impl PatternElement for Any {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        match state.next() {
            Some(..) => CompareResult::Match(0),
            None => CompareResult::End
        }
    }
}


pub struct Byte { // "basic" matcher... but not "simple"
    b: u8,
    escaped: bool, // only used for putting the "%" back where it should be, when printing
}

impl Byte {
    pub fn new(b: u8, escaped: bool) -> Byte {
        Byte {b: b, escaped: escaped}
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.escaped {
            try!(write!(f, "%"));
        }
        match self.b {
            b @ 0x20 ... 0x7E => write!(f, "{}", b as char),
            b @ _ => write!(f, "\\x{:X}", b),
        }
    }
}

impl PatternElement for Byte {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        match state.next() {
            Some(b) => CompareResult::Match(b as isize - self.b as isize),
            None => CompareResult::End
        }
    }
}
