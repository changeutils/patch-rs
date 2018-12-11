//!
//! The GNU patch Rust binary entry point.
//!

extern crate patch_rs;
extern crate clap;

use std::{
    fs,
    io::{self, BufRead},
};

use patch_rs::{PatchParser, PatchResult};

fn read_to_vec(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let file = io::BufReader::new(file);
    file.lines().collect()
}

fn main() -> PatchResult<()> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("file")
                .help("The file to patch")
                .index(1)
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("patch")
                .help("The patch")
                .index(2)
                .value_name("PATCH")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let file = args.value_of("file").unwrap();
    let patch = args.value_of("patch").unwrap();

    println!("patching {}", file);

    let text = read_to_vec(file)?;
    let patch = fs::read_to_string(patch)?;

    let parser = PatchParser::new(&text, &patch);
    let _ = parser.process(&|s| println!("{}", s))?;

    Ok(())
}
