//!
//! The GNU patch Rust library.
//!


#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure_derive;
extern crate failure;
extern crate pest;


mod error;
mod parser;



pub use {
    std::result,
    error::Error as PatchError,
    parser::PatchParser,
};

pub type PatchResult<T> = result::Result<T, PatchError>;