// "Root" Element

use std::fmt;
use {PatternElement, MatchState, CompareResult};

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
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        for c in self.children.iter() {
            match c.compare(state) {
                CompareResult::Match(0) => {},
                r => { return r; },
            }
        }
        CompareResult::Match(0)
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
