//!
//! The Patch library.
//!

mod error;
mod parser;
mod line;
mod context;

pub use crate::{
    error::Error as PatchError,
    parser::{PatchProcessor, Patch},
    line::Line,
    context::{Context, ContextHeader},
};

pub type PatchResult<T> = Result<T, PatchError>;
