//!
//! The error chain.
//!

use std::{io, num};

use parser::Rule;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "Error parsing patch: {}", _0)]
    ParsingPatch(pest::error::Error<Rule>),
    #[fail(display = "Error parsing context: {}", _0)]
    ParsingContext(num::ParseIntError),
    #[fail(display = "Missing an element: {}", _0)]
    NotFound(&'static str),
    #[fail(display = "Elements are found in invalid order: {}", _0)]
    MalformedPatch(&'static str),
    #[fail(display = "Line #{} not found", _0)]
    AbruptInput(usize),
    #[fail(display = "Invalid line #{}", _0)]
    PatchInputMismatch(usize),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Reading(err)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Error {
        Error::ParsingPatch(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParsingContext(err)
    }
}