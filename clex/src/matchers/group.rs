// Groups

use std::fmt;
use {PatternElement, MatchState, CompareResult, MatchCapture};

pub struct Group {
    next: Box<PatternElement>,
    children: Vec<Box<PatternElement>>,
}

impl Group {
    pub fn new(next: Box<PatternElement>, children: Vec<Box<PatternElement>>) -> Group {
        Group { next: next, children: children }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.children.push(child)
    }

    pub fn set_next(&mut self, next: Box<PatternElement>) {
        self.next = next
    }
}

impl PatternElement for Group {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let start = state.pos();
        if self.children.is_empty() { // empty group ()
            state.push_capture(MatchCapture::Position(start));
            return CompareResult::Match(0)
        }
        for c in self.children.iter() {
            match c.compare(state) {
                CompareResult::Match(0) => {}, // just keep going
                e @ _ => { return e }
            }
        }
        let end = state.pos();
        state.push_capture(MatchCapture::Bytes { start: start, end: end });
        self.next.compare(state)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        try!(write!(f, ")"));
        write!(f, "{}", self.next)
    }
}
