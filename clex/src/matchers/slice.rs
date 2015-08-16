// Slice matcher. Used internally.

use std::fmt;
use {PatternElement, MatchState, CompareResult, Next, MatchCapture};

pub struct Slice<'a> {
    slice: &'a [Box<PatternElement>],
    capture: bool,
    startpos: usize,
    backmatch: bool,
}

impl<'a> Slice<'a> {
    pub fn new(slice: &'a [Box<PatternElement>], capture: bool, startpos: usize, backmatch: bool) -> Slice<'a> {
        Slice { slice: slice, capture: capture, startpos: startpos, backmatch: backmatch }
    }

    pub fn simple(slice: &'a [Box<PatternElement>]) -> Slice<'a> {
        Slice::new(slice, false, 0, false)
    }

    pub fn slice(&self, from: usize) -> Slice<'a> {
        Slice { slice: &self.slice[from..], .. *self }
    }
}

impl<'a> PatternElement for Slice<'a> {
    // set this to always inline so LLVM derives the `root` and `group` variants for us
    #[inline(always)]
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        let mut result = CompareResult::Match(0);
        if self.slice.len() == 0 {
            // TODO
            let pos = state.pos();
            if let Some(n) = next {
                result = n.compare(state);
                if let CompareResult::Match(0) = result {
                    if self.capture {
                        state.push_capture(MatchCapture::Bytes { start: self.startpos, end: pos});
                    }
                }
            } else {
                if self.capture {
                    state.push_capture(MatchCapture::Bytes { start: self.startpos, end: pos});
                }
            }
        } else {
            let mut iter = self.slice.iter().enumerate();
            while let Some((i, c)) = iter.next() {
                if c.handle_next() {
                    let slice = self.slice(i+1);
                    let v = c.compare_next(state, Some(&Next::new(&slice, next)));
                    match v {
                        CompareResult::Match(0) => {},
                        r => if self.backmatch { // last is most significant
                            result = r;
                        } else if let CompareResult::Match(0) = result { // first is most significant
                            result = r;
                        },
                    }
                    break;
                } else {
                    match c.compare(state) {
                        CompareResult::Match(0) => {},
                        r => if self.backmatch { // last is most significant
                            result = r;
                        } else if let CompareResult::Match(0) = result { // first is most significant
                            result = r;
                        },
                    }
                }
            }
        }
        if let Some(n) = next { // TODO
            match result {
                CompareResult::Match(0) => {
                    n.compare(state)
                },
                r => {
                    //n.compare(state);
                    r
                },
            }
        } else {
            result
        }
    }

    fn handle_next(&self) -> bool { // is this needed for something that's only ever passed as a next?
        true
    }
}

impl<'a> fmt::Display for Slice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[Slice matcher cannot be printed, as it is not part of a pattern.]")
    }
}
