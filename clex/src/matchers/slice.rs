// Slice matcher. Prime example of the "Userdata" feature. Used internally.

use std::fmt;
use {PatternElement, MatchState, CompareResult, Next};

pub struct Slice<'a> {
    slice: &'a [Box<PatternElement>],
    group: bool,
}

impl<'a> Slice<'a> {
    pub fn new(slice: &'a [Box<PatternElement>], group: bool) -> Slice<'a> {
        Slice { slice: slice, group: group }
    }

    pub fn simple(slice: &'a [Box<PatternElement>]) -> Slice<'a> {
        Slice { slice: slice, group: false }
    }

    pub fn group(slice: &'a [Box<PatternElement>]) -> Slice<'a> {
        Slice { slice: slice, group: true }
    }

    // TODO fix these 2
    // check if they should be FIFO or FILO and change implementation as needed

    pub fn pop(state: &mut MatchState) -> usize {
        let ud = state.get_ud();
        let boxed = ud.get_mut("clex::slice").unwrap();
        boxed.downcast_mut::<Vec<(usize, usize)>>().unwrap().pop().unwrap().0
    }

    pub fn pop_group(state: &mut MatchState) -> (usize, usize) {
        let ud = state.get_ud();
        let boxed = ud.get_mut("clex::slice").unwrap();
        boxed.downcast_mut::<Vec<_>>().unwrap().pop().unwrap()
    }
}

/*fn push(state: &mut MatchState, count: usize) {
    let ud = state.get_ud();
    if !ud.contains_key("clex::slice") {
        ud.insert("clex::slice".to_string(), Box::new(Vec::<(usize, usize)>::new()));
    }
    let boxed = ud.get_mut("clex::slice").unwrap();
    boxed.downcast_mut::<Vec<(usize, usize)>>().unwrap().push((count, 0))
}*/

fn push_group(state: &mut MatchState, count: usize, pos: usize) {
    let ud = state.get_ud();
    if !ud.contains_key("clex::slice") {
        ud.insert("clex::slice".to_string(), Box::new(Vec::<(usize, usize)>::new()));
    }
    let boxed = ud.get_mut("clex::slice").unwrap();
    boxed.downcast_mut::<Vec<(usize, usize)>>().unwrap().push((count, pos))
}

impl<'a> PatternElement for Slice<'a> {
    // set this to always inline so LLVM derives the `root` and `group` variants for us
    #[inline(always)]
    #[allow(unused_variables)]
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        let mut result = CompareResult::Match(0);
        if self.slice.len() != 0 {
            let mut iter = self.slice.iter().enumerate();
            let mut lastpos = None;
            while let Some((i, c)) = iter.next() {
                if c.handle_next() {
                    let slice = Slice::new(&self.slice[i+1..], self.group);
                    let v = c.compare_next(state, Some(&Next::new(&slice, next))); // TODO check this
                    let (count, pos) = Slice::pop_group(state);
                    match v {
                        CompareResult::Match(0) => {
                            iter.nth(count);
                        },
                        r => if let CompareResult::Match(0) = result {
                            result = r;
                        },
                    }
                    if i == self.slice.len() - 1 { // TODO fix this
                        lastpos = Some(pos);
                    }
                } else {
                    match c.compare(state) {
                        CompareResult::Match(0) => {},
                        r => if let CompareResult::Match(0) = result {
                            result = r;
                        },
                    }
                    if i == self.slice.len() - 1 {
                        lastpos = Some(state.pos());
                    }
                }
            }
            push_group(state, self.slice.len(), lastpos.unwrap()); // TODO check for off-by-one
            //result
        } else {
            // TODO
            let pos = state.pos();
            push_group(state, self.slice.len(), pos); // TODO check for off-by-one
            //unimplemented!()
        }
        if let Some(n) = next { // TODO
            match result {
                CompareResult::Match(0) => {
                    n.element.compare_next(state, n.next)
                },
                r => {
                    n.element.compare_next(state, n.next);
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
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[Slice matcher cannot be printed, as it is not part of a pattern.]")
    }
}
