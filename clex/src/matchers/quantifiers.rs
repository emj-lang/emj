// Quantifiers

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
        let start = state.pos();
        let first = self.quantified.compare(state);
        match self.quantification {
            Quantification::Greedy => {
                match first {
                    CompareResult::Match(0) => {},
                    e @ _ => { return e }
                }
                let mut matched: Vec<usize> = Vec::new();
                matched.push(state.pos());
                loop {
                    match self.quantified.compare(state) {
                        CompareResult::Match(0) => { matched.push(state.pos()) },
                        _ => { break },
                    }
                }
                for backtrack in matched.iter().rev() {
                    state.set_pos(*backtrack);
                    match self.next.compare(state) {
                        CompareResult::Match(0) => { return CompareResult::Match(0) },
                        _ => {},
                    }
                }
                state.set_pos(matched[0]);
                self.next.compare(state)
            },
            Quantification::NonGreedy => {
                match first {
                    CompareResult::Match(0) => {},
                    e @ _ => { return e }
                }
                loop {
                    match self.quantified.compare(state) {
                        CompareResult::Match(0) => {
                            match self.next.compare(state) {
                                CompareResult::Match(0) => { return CompareResult::Match(0) },
                                _ => {},
                            }
                        },
                        e @ _ => { return e },
                    }
                }
            },
            Quantification::Optional => {
                match first {
                    CompareResult::Match(0) => {},
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
