// Alternations and Ranges

// TODO

use std::fmt;
use {PatternElement, MatchState, CompareResult};

pub struct Set {
    next: Box<PatternElement>,
    elements: Vec<SetElement>,
    negated: bool,
}

pub enum SetElement {
    PE(Box<PatternElement>),
    R { lower: Box<PatternElement>, upper: Box<PatternElement> },
}

impl Set {
    pub fn new(next: Box<PatternElement>, elements: Vec<SetElement>, negated: bool) -> Set {
        Set { next: next, elements: elements, negated: negated }
    }
}

impl PatternElement for Set {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        if self.elements.is_empty() { return CompareResult::Match(0) }
        let start = state.pos();
        let mut result = None;
        for e in self.elements.iter() {
            match e {
                &SetElement::PE(ref pe) => {
                    result = Some(pe.compare(state));
                    if let Some(CompareResult::Match(0)) = result {
                        break;
                    }
                },
                &SetElement::R{ ref lower, ref upper } => {
                    result = Some(lower.compare(state));
                    match result.unwrap() {
                        CompareResult::Match(e) if e < 0 => { continue; },
                        CompareResult::End => { continue; },
                        _ => {},
                    }
                    result = Some(upper.compare(state));
                    match result.unwrap() {
                        CompareResult::Match(e) if e > 0 => { continue; },
                        CompareResult::End => { continue; },
                        _ => { break; },
                    }
                },
            }
            if self.negated {
                match result.unwrap() {
                    CompareResult::Match(0) => { return CompareResult::End; },
                    CompareResult::End => {},
                    _ => match self.next.compare(state) {
                        r @ CompareResult::Match(0) => { return r; },
                        _ => {},
                    },
                }
            } else {
                match result.unwrap() {
                    CompareResult::Match(0) => match self.next.compare(state) {
                        r @ CompareResult::Match(0) => { return r; },
                        _ => {},
                    },
                    _ => {},
                }
            }
            state.set_pos(start);
        }
        if self.negated {
            match result.unwrap() {
                CompareResult::Match(0) => CompareResult::End,
                _ => CompareResult::Match(0),
            }
        } else {
            result.unwrap()
        }
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        if self.negated {
            try!(write!(f, "*"));
        }
        for e in self.elements.iter() {
            match e {
                &SetElement::PE(ref pe) => { try!(write!(f, "{}", pe)); },
                &SetElement::R{ ref lower, ref upper } => {
                    try!(write!(f, "{}", lower));
                    try!(write!(f, ":"));
                    try!(write!(f, "{}", upper));
                },
            }
        }
        try!(write!(f, "]"));
        write!(f, "{}", self.next)
    }
}
