// Groups

use std::fmt;
use {PatternElement, MatchState, CompareResult, MatchCapture};

pub struct Group {
    children: Vec<Box<PatternElement>>,
}

impl Group {
    pub fn new(children: Vec<Box<PatternElement>>) -> Group {
        Group { children: children }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.children.push(child)
    }
}

impl PatternElement for Group {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let start = state.pos();
        if self.children.is_empty() { // empty group ()
            state.push_capture(MatchCapture::Position(start));
            return CompareResult::Match(0)
        }
        let mut result = CompareResult::Match(0);
        for c in self.children.iter() {
            match c.compare(state) {
                CompareResult::End => { return CompareResult::End; },
                r => {
                    if let CompareResult::Match(0) = result {
                        result = r;
                    }
                },
            }
        }
        match result {
            CompareResult::Match(0) => {
                let end = state.pos();
                state.push_capture(MatchCapture::Bytes { start: start, end: end });
                //self.next.compare(state)
                CompareResult::Match(0)
            },
            r => r,
        }
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        write!(f, ")")
    }
}
