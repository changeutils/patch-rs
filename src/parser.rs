//!
//! The parser implementation.
//!

use pest::{iterators::Pair, Parser};

use super::error::{PatchResult, PatchErrorKind};

#[derive(Parser)]
#[grammar = "../peg/patch.peg"]
pub struct PatchParser<'a> {
    text: &'a [String],
    patch: &'a str,
}

impl<'a> PatchParser<'a> {
    pub fn new(text: &'a [String], patch: &'a str) -> Self {
        Self { text, patch }
    }

    pub fn process(&self, callback: &Fn(String) -> ()) -> PatchResult<()> {
        let patch = Self::parse(Rule::patch, self.patch)?
            .next()
            .ok_or(PatchErrorKind::NotFound("patch"))?
            .into_inner();
        Ok(())
    }
}
