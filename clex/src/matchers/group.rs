// Groups

use std::fmt;

use {PatternElement, MatchState, CompareResult, MatchCapture};

// Prime example of the "Userdata" feature.
#[derive(Copy, Clone)]
struct GroupInfo {
    capture: bool,
    start: usize,
}

fn push(state: &mut MatchState, info: GroupInfo) {
    let ud = state.get_ud();
    if !ud.contains_key("clex::group") {
        ud.insert("clex::group".to_string(), Box::new(Vec::<GroupInfo>::new()));
    }
    let boxed = ud.get_mut("clex::group").unwrap();
    boxed.downcast_mut::<Vec<_>>().unwrap().push(info)
}

fn pop(state: &mut MatchState) -> GroupInfo {
    let ud = state.get_ud();
    let boxed = ud.get_mut("clex::group").unwrap();
    boxed.downcast_mut::<Vec<_>>().unwrap().pop().unwrap()
}


pub struct GroupOpen {
    children: Vec<Box<PatternElement>>,
    capture: bool,
}

impl GroupOpen {
    pub fn new(children: Vec<Box<PatternElement>>, capture: bool) -> GroupOpen {
        GroupOpen { children: children, capture: capture }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.children.push(child)
    }
}

impl PatternElement for GroupOpen {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let start = state.pos();
        push(state, GroupInfo { capture: self.capture, start: start });
        let mut result = CompareResult::Match(0);
        for c in self.children.iter() {
            match c.compare(state) {
                CompareResult::Match(0) => {},
                r => if let CompareResult::Match(0) = result {
                    result = r;
                },
            }
        }
        match result {
            CompareResult::Match(0) => {},
            _ => {
                pop(state);
            },
        }
        result
    }
}

impl fmt::Display for GroupOpen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        if !self.capture {
            try!(write!(f, "*"));
        }
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        Ok(())
    }
}


pub struct GroupClose {
    next: Box<PatternElement>,
}

impl GroupClose {
    pub fn new(next: Box<PatternElement>) -> GroupClose {
        GroupClose { next: next }
    }
}

impl PatternElement for GroupClose {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        let end = state.pos();
        let gi = pop(state);
        let r = self.next.compare(state);
        match r {
            CompareResult::Match(0) => if gi.capture {
                state.push_capture(MatchCapture::Bytes { start: gi.start, end: end })
            },
            _ => {
                push(state, gi)
            },
        }
        r
    }
}

impl fmt::Display for GroupClose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, ")"));
        write!(f, "{}", self.next)
    }
}
