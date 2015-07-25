// Alternations and Ranges

use std::fmt;
use {PatternElement, MatchState, CompareResult};

pub struct Set {
    next: Box<PatternElement>,
    elements: Vec<SetElement>,
    negated: bool, // TODO
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
        if self.negated {
            unimplemented!() // TODO
        }
        let start = state.pos();
        if self.elements.is_empty() { return CompareResult::Match(0) }
        let mut last = None;
        for e in self.elements.iter() {
            match e {
                &SetElement::PE(ref pe) => {
                    match pe.compare(state) {
                        CompareResult::Match(0) => { return CompareResult::Match(0); },
                        e @ _ => { last = Some(e) },
                    }
                },
                &SetElement::R(Range { ref lower, ref upper }) => {
                    let lowermatch = lower.compare(state);
                    last = Some(lowermatch);
                    match last {
                        Some(CompareResult::Match(e)) if e < 0 => { continue; },
                        Some(CompareResult::End) => { continue; },
                        Some(_) => {},
                        _ => unreachable!()
                    }
                    let uppermatch = upper.compare(state);
                    last = Some(uppermatch);
                    match last {
                        Some(CompareResult::Match(e)) if e > 0 => { continue; },
                        Some(CompareResult::End) => { continue; },
                        Some(_) => { return CompareResult::Match(0) },
                        _ => unreachable!()
                    }
                },
            }
            state.set_pos(start);
        }
        last.unwrap()
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
