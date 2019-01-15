//!
//! The parser implementation.
//!

use pest::{iterators::Pair, Parser};

use crate::error::Error;

#[derive(Parser)]
#[grammar = "../peg/patch.peg"]
pub struct PatchParser {
    text: Vec<String>,
    patch: Patch,
}

pub struct Patch(Vec<Context>);

pub type PatchResult<T> = Result<T, Error>;

struct Context {
    pub header: ContextHeader,
    pub data: Vec<PatchLine>,
}

#[derive(Default)]
struct ContextHeader {
    pub file1_l: usize,
    pub file1_s: usize,
    pub file2_l: usize,
    pub file2_s: usize,
}

enum PatchLine {
    Context(String),
    Insert(String),
    Delete(String),
}

impl PatchParser {
    pub fn converted(text: Vec<String>, patch: &str) -> PatchResult<Self> {
        Ok(Self {
            text,
            patch: Self::convert(patch)?,
        })
    }

    pub fn process(&self) -> PatchResult<Vec<String>> {
        let mut file2_text = Vec::new();
        let mut file1_ptr: usize = 0;

        for context in &self.patch.0 {
            for i in file1_ptr..context.header.file1_l {
                file2_text.push(
                    self.text
                        .get(i)
                        .ok_or_else(|| Error::AbruptInput(i))?
                        .to_owned(),
                );
            }
            file1_ptr = context.header.file1_l;
            for line in &context.data {
                match line {
                    PatchLine::Context(ref data) => {
                        if self
                            .text
                            .get(file1_ptr)
                            .ok_or_else(|| Error::AbruptInput(file1_ptr))?
                            != data
                        {
                            return Err(Error::PatchInputMismatch(file1_ptr));
                        }
                        file2_text.push(data.to_owned());
                        file1_ptr += 1;
                    }
                    PatchLine::Delete(ref data) => {
                        if self
                            .text
                            .get(file1_ptr)
                            .ok_or_else(|| Error::AbruptInput(file1_ptr))?
                            != data
                        {
                            return Err(Error::PatchInputMismatch(file1_ptr));
                        }
                        file1_ptr += 1;
                    }
                    PatchLine::Insert(ref data) => {
                        file2_text.push(data.to_owned());
                    }
                }
            }
        }

        for i in file1_ptr..self.text.len() {
            file2_text.push(
                self.text
                    .get(i)
                    .ok_or_else(|| Error::AbruptInput(i))?
                    .to_owned(),
            );
        }

        Ok(file2_text)
    }

    pub fn convert(patch: &str) -> PatchResult<Patch> {
        let peg_patch = Self::parse(Rule::patch, patch)?
            .next()
            .ok_or(Error::NotFound("patch"))?;

        let mut patch = Patch(Vec::new());
        for patch_element in peg_patch.into_inner() {
            if let Rule::context = patch_element.as_rule() {
                let mut peg_context = patch_element.into_inner();
                let context_header = peg_context
                    .next()
                    .ok_or(Error::NotFound("context_header"))?;
                let context_header = if let Rule::context_header = context_header.as_rule() {
                    Self::get_context_header(context_header)?
                } else {
                    return Err(Error::MalformedPatch(
                        "Context header is not at the start of a context",
                    ));
                };

                let mut context = Context {
                    header: context_header,
                    data: Vec::new(),
                };
                for line in peg_context {
                    match line.as_rule() {
                        Rule::line_context => context
                            .data
                            .push(PatchLine::Context(line.as_span().as_str().to_owned())),
                        Rule::line_deleted => context
                            .data
                            .push(PatchLine::Delete(line.as_span().as_str().to_owned())),
                        Rule::line_inserted => context
                            .data
                            .push(PatchLine::Insert(line.as_span().as_str().to_owned())),
                        _ => {}
                    }
                }
                patch.0.push(context);
            }
        }

        Ok(patch)
    }

    fn get_context_header(header: Pair<'_, Rule>) -> PatchResult<ContextHeader> {
        let mut output = ContextHeader::default();
        for header_element in header.into_inner() {
            match header_element.as_rule() {
                Rule::file1_l => output.file1_l = header_element.as_span().as_str().parse()?,
                Rule::file1_s => output.file1_s = header_element.as_span().as_str().parse()?,
                Rule::file2_l => output.file2_l = header_element.as_span().as_str().parse()?,
                Rule::file2_s => output.file2_s = header_element.as_span().as_str().parse()?,
                _ => {}
            }
        }
        output.file1_l -= 1;
        output.file2_l -= 1;
        Ok(output)
    }
}
