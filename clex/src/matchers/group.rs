// Groups

use std::fmt;
use {PatternElement, MatchState, CompareResult, MatchCapture, Next};

use super::slice::Slice;

pub struct Group {
    capture: bool,
    handle_next: bool,
    children: Vec<Box<PatternElement>>,
}

impl Group {
    pub fn new(capture: bool) -> Group {
        Group { capture: capture, handle_next: false, children: Vec::new() }
    }

    pub fn push_child(&mut self, child: Box<PatternElement>) {
        self.handle_next = self.handle_next || child.handle_next();
        self.children.push(child)
    }
}

impl PatternElement for Group {
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn compare_next(&self, state: &mut MatchState, next: Option<&Next>) -> CompareResult {
        // TODO check
        // NB: can be simplified by calling into a Slice element, as they have extremely similar code
        let start = state.pos();
        let mut endpos = start;
        let mut result = CompareResult::Match(0);
        let mut iter = self.children.iter().enumerate();
        while let Some((i, c)) = iter.next() {
            if c.handle_next() {
                let slice = Slice::group(&self.children[i+1..]);
                let v = c.compare_next(state, Some(&Next::new(&slice, next)));
                let (count, pos) = Slice::pop_group(state);
                match v {
                    CompareResult::Match(0) => {
                        iter.nth(count);
                    },
                    r => if let CompareResult::Match(0) = result {
                        result = r;
                    },
                }
            } else {
                match c.compare(state) {
                    CompareResult::Match(0) => {},
                    r => if let CompareResult::Match(0) = result {
                        result = r;
                    },
                }
            }
        }
        if self.capture {
            if let CompareResult::Match(0) = result {
                if self.children.is_empty() {
                    state.push_capture(MatchCapture::Position(start))
                } else {
                    let end = state.pos();
                    state.push_capture(MatchCapture::Bytes { start: start, end: end })
                }
            }
        }
        result
    }

    fn handle_next(&self) -> bool {
        self.handle_next
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "("));
        if !self.capture {
            try!(write!(f, "*"));
        }
        for c in self.children.iter() {
            try!(write!(f, "{}", c));
        }
        write!(f, ")")
    }
}


/*#[derive(Copy, Clone)]
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
}*/
