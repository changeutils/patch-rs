//!
//! The Patch library.
//!

mod error;
mod parser;
mod line;
mod context;
mod patch;

pub use crate::{
    error::Error as PatchError,
    parser::PatchProcessor,
    line::Line,
    patch::Patch,
    context::{Context, ContextHeader},
};

pub type PatchResult<T> = Result<T, PatchError>;
