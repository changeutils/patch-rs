//!
//! The Patch library.
//!

mod error;
mod parser;
mod data;

pub use crate::{
    error::Error as PatchError,
    parser::PatchProcessor,
    data::{PatchLine, Patch, Context, ContextHeader},
};

pub type PatchResult<T> = Result<T, PatchError>;
