//!
//! The Patch structure.
//!

use std::{
    fmt,
    collections::VecDeque,
};

use crate::context::Context;

pub struct Patch {
    pub input: String,
    pub output: String,
    pub contexts: VecDeque<Context>,
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- {}", self.input)?;
        writeln!(f, "+++ {}", self.output)?;
        for context in self.contexts.iter() {
            write!(f, "{}", context)?;
        }
        Ok(())
    }
}
