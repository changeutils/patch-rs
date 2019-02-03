//!
//! The Patch data structures.
//!

use std::fmt;

#[derive(Clone)]
pub enum Line {
    Context(String),
    Insert(String),
    Delete(String),
}

impl Line {
    pub fn flip(&self) -> Self {
        match self {
            Line::Context(line) => Line::Context(line.clone()),
            Line::Insert(line) => Line::Delete(line.clone()),
            Line::Delete(line) => Line::Insert(line.clone()),
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::Context(line) => write!(f, " {}", line),
            Line::Insert(line) => write!(f, "+{}", line),
            Line::Delete(line) => write!(f, "-{}", line),
        }
    }
}
