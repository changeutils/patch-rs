//!
//! The GNU patch Rust library.
//!

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate failure_derive;

mod error;
mod parser;

pub use crate::{
    error::Error as PatchError,
    parser::{PatchProcessor, PatchResult},
};
