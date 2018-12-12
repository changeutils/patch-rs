//!
//! The error chain.
//!

use std::{io, num};

use parser::Rule;

error_chain! {
    types {
        PatchError, PatchErrorKind, PatchResultExt, PatchResult;
    }

    foreign_links {
        Reading(io::Error);
        ParsingPatch(pest::error::Error<Rule>);
        ParsingContext(num::ParseIntError);
    }

    errors {
        NotFound(desc: &'static str) {
            description("Element is not found")
            display("Missing an element: {}", desc)
        }
        MalformedPatch(desc: &'static str) {
            description("Malformed patch")
            display("Elements are found in invalid order: {}", desc)
        }
        AbruptInput(line: usize) {
            description("Abrupt input file")
            display("Line #{} not found", line)
        }
        PatchInputMismatch(line: usize) {
            description("Patch does not match the input file")
            display("Invalid line #{}", line)
        }
    }
}
