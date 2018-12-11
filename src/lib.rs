//!
//! The GNU patch Rust library.
//!

mod error;
mod parser;

#[macro_use]
extern crate error_chain;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use {
    parser::PatchParser,
    error::{PatchResult, PatchError},
};
