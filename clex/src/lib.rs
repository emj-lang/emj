use std::fmt;
use std::collections::HashMap;
use std::any::Any;

pub mod matchers;
pub mod matchstate;
pub use matchstate::MatcherState;

pub trait MatchState {
    fn pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize) -> bool;
    fn max_pos(&self) -> usize;
    fn has_next(&self) -> bool;
    unsafe fn unsafe_next(&mut self) -> u8;
    fn next(&mut self) -> Option<u8>;
    fn get(&self) -> u8;
    fn captures(&self) -> usize;
    fn get_capture(&self, index: usize) -> Option<MatchCapture>;
    fn push_capture(&mut self, captured: MatchCapture);
    fn get_ud(&mut self) -> &mut HashMap<String, Box<Any>>;
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

pub fn parse(pattern: &[u8]) -> Result<Box<PatternElement>, String> {
    let mut root = matchers::Root::new(vec![]);
    let mut iter = pattern.iter();
    while let Some(&c) = iter.next() {
        match c {
            b'%' => {
                match iter.next() {
                    Some(&c) => {
                        match c {
                            c @ b'A' ... b'Z' | c @ b'a' ... b'z' => {
                                let mut s = "Unknown escape ".to_string();
                                s.push(c as char);
                                return Err(s);
                            },
                            c => root.push_child(Box::new(matchers::Byte::new(c, true))),
                        }
                    },
                    None => {
                        return Err("".to_string());
                    }
                }
            }
            c => {
                root.push_child(Box::new(matchers::Byte::new(c, false)))
            },
        }
    }
    Ok(Box::new(root))
}
