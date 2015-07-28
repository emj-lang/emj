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
        let mut result = CompareResult::Match(0);
        for c in self.children.iter() {
            match c.compare(state) {
                CompareResult::Match(0) => {},
                r => if let CompareResult::Match(0) = result {
                    result = r;
                },
            }
        }
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
