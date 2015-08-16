use std::collections::HashMap;
use std::any::Any;

use {MatchCapture, MatchState};

#[derive(Debug)]
pub struct MatcherState<'a> {
    pos: usize,
    data: &'a [u8],
    captured: Vec<MatchCapture>,
    userdata: HashMap<String, Box<Any>>,
}

impl<'a> MatcherState<'a> {
    pub fn new(data: &'a [u8]) -> MatcherState<'a> {
        MatcherState { pos: 0, data: data, captured: Vec::new(), userdata: HashMap::new() }
    }
}

impl<'a> MatchState for MatcherState<'a> {
    fn pos(&self) -> usize {
        self.pos
    }

    fn set_pos(&mut self, pos: usize) -> bool {
        if pos > self.data.len() {
            return false
        }
        self.pos = pos;
        true
    }

    fn max_pos(&self) -> usize {
        self.data.len()
    }

    fn has_next(&self) -> bool {
        self.pos < self.data.len()
    }

    unsafe fn unsafe_next(&mut self) -> u8 {
        let x = self.pos;
        self.pos += 1;
        self.data[x]
    }

    fn next(&mut self) -> Option<u8> {
        if self.has_next() {
            Some(unsafe { self.unsafe_next() })
        } else {
            None
        }
    }

    fn get(&self) -> u8 {
        self.data[self.pos]
    }

    fn captures(&self) -> usize {
        self.captured.len()
    }

    fn get_capture(&self, index: usize) -> Option<MatchCapture> {
        match self.captured.get(index) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn push_capture(&mut self, captured: MatchCapture) {
        self.captured.push(captured)
    }

    fn get_ud(&mut self) -> &mut HashMap<String, Box<Any>> {
        &mut self.userdata
    }
}
