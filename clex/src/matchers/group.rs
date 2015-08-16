// Groups

use std::fmt;
use {PatternElement, MatchState, CompareResult, MatchCapture, Next};

use super::slice::Slice;

pub struct Group {
    capture: bool,
    backmatch: bool,
    handle_next: bool, // lame optimization, is this even useful?
    children: Vec<Box<PatternElement>>,
}

impl Group {
    pub fn new(capture: bool, backmatch: bool) -> Group {
        Group {
            capture: capture,
            backmatch: backmatch,
            handle_next: false,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.handle_next = self.handle_next || child.handle_next();
        self.children.push(child)
    }
}

impl PatternElement for Group {
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        // TODO check
        // NB: can be simplified by calling into a Slice element, as they have extremely similar code
        let pos = state.pos();
        if self.children.is_empty() {
            state.push_capture(MatchCapture::Position(pos));
            return CompareResult::Match(0)
        } else {
            let slice = Slice::new(&self.children[..], self.capture, pos, self.backmatch);
            let result = slice.compare_next(state, next);
            result
        }
    }

    fn handle_next(&self) -> bool {
        self.handle_next
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        if !self.capture {
            try!(write!(f, "*"));
        }
        if self.backmatch {
            try!(write!(f, "<"));
        }
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        write!(f, ")")
    }
}
