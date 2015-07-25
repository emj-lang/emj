// Alternations and Ranges

use std::fmt;
use {PatternElement, MatchState, CompareResult};

pub struct Set {
    next: Box<PatternElement>,
    elements: Vec<SetElement>,
    negated: bool,
}

pub enum SetElement {
    PE(Box<PatternElement>),
    R(Range),
}

pub struct Range {
    lower: Box<PatternElement>,
    upper: Box<PatternElement>,
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
                &SetElement::R(Range { ref lower, ref upper }) => {
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
                &SetElement::R(Range { ref lower, ref upper }) => {
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
