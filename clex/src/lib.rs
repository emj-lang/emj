use std::fmt;

pub mod matchers;

#[derive(Clone, Debug)]
pub struct MatchState<'a> {
    pos: usize,
    data: &'a [u8],
    captured: Vec<MatchCapture>,
}

impl<'a> MatchState<'a> {
    pub fn new(data: &'a [u8]) -> MatchState<'a> {
        MatchState { pos: 0, data: data, captured: Vec::new() }
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

pub trait PatternElement : fmt::Display {
    fn compare(&self, &mut MatchState) -> CompareResult;
}

pub fn parse(pattern: &[u8]) -> Result<Box<PatternElement>, String> {
    unimplemented!()
}
