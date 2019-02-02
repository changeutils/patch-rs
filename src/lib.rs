//!
//! The GNU patch Rust library.
//!

mod error;
mod parser;

pub use crate::{
    error::Error as PatchError,
    parser::{PatchProcessor, PatchResult, PatchLine, Patch},
};
