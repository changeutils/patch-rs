//!
//! The error chain.
//!

use std::io;

use parser::Rule;

error_chain! {
    types {
        PatchError, PatchErrorKind, PatchResultExt, PatchResult;
    }

    foreign_links {
        Reading(io::Error);
        Parsing(pest::error::Error<Rule>);
    }

    errors {
        NotFound(t: &'static str) {
            description("Element is not found")
            display("Missing an element: {}", t)
        }
    }
}
