// "Root" Element

use std::fmt;
use {PatternElement, MatchState, CompareResult};

use super::slice::Slice;

pub struct Root {
    children: Vec<Box<PatternElement>>,
}

impl Root {
    pub fn new(children: Vec<Box<PatternElement>>) -> Root {
        Root { children: children }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.children.push(child)
    }
}

impl PatternElement for Root {
    // Root elements don't handle continuations, mostly because they're, well, root elements.
    // NB: simplified by calling into a Slice element, as they basically have the same code.
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let slice = Slice::simple(&self.children[..]);
        let result = slice.compare_next(state, None);
        Slice::pop(state);
        result
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        Ok(())
    }
}
