// Quantifiers
// ... ARE A PAIN IN THE ASS TO DO ;_;

// TODO

use std::fmt;
use {PatternElement, MatchState, CompareResult};

pub enum Quantification {
    Greedy,
    NonGreedy,
    Optional
}

pub struct Quantifier {
    quantification: Quantification,
    quantified: Box<PatternElement>,
    next: Box<PatternElement>,
}

impl Quantifier {
    pub fn new(quantification: Quantification, quantified: Box<PatternElement>, next: Box<PatternElement>) -> Quantifier {
        Quantifier{quantification: quantification, quantified: quantified, next: next}
    }
}

impl PatternElement for Quantifier {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        match self.quantification {
            Quantification::Greedy => { // TODO check
                match self.quantified.compare(state) {
                    CompareResult::Match(0) => {},
                    r => { return r }
                }
                let mut matched: Vec<usize> = Vec::new();
                matched.push(state.pos());
                loop {
                    match self.quantified.compare(state) {
                        CompareResult::Match(0) => { matched.push(state.pos()) },
                        _ => { break },
                    }
                }
                let mut last = None;
                for backtrack in matched.iter().rev() {
                    state.set_pos(*backtrack);
                    match self.next.compare(state) {
                        CompareResult::Match(0) => { return CompareResult::Match(0) },
                        r => { last = Some(r) },
                    }
                }
                last.unwrap()
            },
            Quantification::NonGreedy => {
                loop {
                    match self.quantified.compare(state) {
                        CompareResult::Match(0) => {
                            let pos = state.pos();
                            match self.next.compare(state) {
                                CompareResult::Match(0) => { return CompareResult::Match(0) },
                                _ => {},
                            }
                            state.set_pos(pos);
                        },
                        r => { return r },
                    }
                }
            },
            Quantification::Optional => {
                let start = state.pos();
                match self.quantified.compare(state) {
                    CompareResult::Match(0) => match self.next.compare(state) {
                        r @ CompareResult::Match(0) => { return r; },
                        _ => { state.set_pos(start); },
                    },
                    _ => { state.set_pos(start); }
                }
                self.next.compare(state)
            },
        }
    }
}

impl fmt::Display for Quantifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.quantified));
        match self.quantification {
            Quantification::Greedy => try!(write!(f, "+")),
            Quantification::NonGreedy => try!(write!(f, "-")),
            Quantification::Optional => try!(write!(f, "?")),
        }
        write!(f, "{}", self.next)
    }
}
