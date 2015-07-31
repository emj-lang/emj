use std::fmt;
use std::collections::HashMap;
use std::any::Any;

pub mod matchers;

#[derive(Debug)]
pub struct MatchState<'a> {
    pos: usize,
    data: &'a [u8],
    captured: Vec<MatchCapture>,
    userdata: HashMap<String, Box<Any>>,
}

impl<'a> MatchState<'a> {
    pub fn new(data: &'a [u8]) -> MatchState<'a> {
        MatchState { pos: 0, data: data, captured: Vec::new(), userdata: HashMap::new() }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) -> bool {
        if pos > self.data.len() {
            return false
        }
        self.pos = pos;
        true
    }

    pub fn max_pos(&self) -> usize {
        self.data.len()
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.data.len()
    }

    pub unsafe fn unsafe_next(&mut self) -> u8 {
        let x = self.pos;
        self.pos += 1;
        self.data[x]
    }

    pub fn next(&mut self) -> Option<u8> {
        if self.has_next() {
            Some(unsafe { self.unsafe_next() })
        } else {
            None
        }
    }

    pub fn get(&self) -> u8 {
        self.data[self.pos]
    }

    pub fn get_capture(&self, index: usize) -> Option<MatchCapture> {
        match self.captured.get(index) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    pub fn push_capture(&mut self, captured: MatchCapture) {
        self.captured.push(captured)
    }

    pub fn get_ud(&mut self) -> &mut HashMap<String, Box<Any>> {
        &mut self.userdata
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MatchCapture {
    Position(usize),
    Bytes { start: usize, end: usize },
}

#[derive(Copy, Clone, Debug)]
pub enum CompareResult {
    Match(isize),
    End, // for end of string
}

#[derive(Copy, Clone)]
pub struct Next<'a, 'b> {
    element: &'a PatternElement,
    next: Option<&'b Next<'b, 'b>>,
}

impl<'a, 'b> Next<'a, 'b> {
    pub fn new(element: &'a PatternElement, next: Option<&'b Next>) -> Next<'a, 'b> {
        Next { element: element, next: next }
    }

    pub fn element(&self) -> &PatternElement {
        self.element
    }

    pub fn next(&self) -> Option<&Next> {
        self.next
    }

    pub fn compare(&self, state: &mut MatchState) -> CompareResult {
        self.element.compare_next(state, self.next)
    }
}

pub trait PatternElement : fmt::Display {
    fn compare(&self, state: &mut MatchState) -> CompareResult {
        self.compare_next(state, None)
    }

    fn compare_next(&self, state: &mut MatchState, Option<&Next>) -> CompareResult {
        self.compare(state)
    }

    fn handle_next(&self) -> bool {
        false
    }
}

#[allow(unused_variables)]
pub fn parse(pattern: &[u8]) -> Result<Box<PatternElement>, String> {
    // TODO
    unimplemented!()
}
