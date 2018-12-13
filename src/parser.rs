//!
//! The parser implementation.
//!

use std::fmt;

use pest::{iterators::Pair, Parser};

use {
    error::Error,
    PatchResult
};

#[derive(Parser)]
#[grammar = "../peg/patch.peg"]
pub struct PatchParser {
    text: Vec<String>,
    patch: String,
}

#[derive(Default)]
struct ContextHeader {
    pub file1_l: usize,
    pub file1_s: usize,
    pub file2_l: usize,
    pub file2_s: usize,
}

impl fmt::Display for ContextHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "-{},{} +{},{}",
            self.file1_l, self.file1_s, self.file2_l, self.file2_s,
        )?;
        Ok(())
    }
}

impl PatchParser {
    pub fn new(text: Vec<String>, patch: String) -> Self {
        Self { text, patch }
    }

    pub fn process(&self) -> PatchResult<Vec<String>> {
        let patch = Self::parse(Rule::patch, &self.patch)?
            .next()
            .ok_or(Error::NotFound("patch"))?;

        let mut file2_text = Vec::new();
        let mut file1_ptr: usize = 0;

        for patch_element in patch.into_inner() {
            match patch_element.as_rule() {
                Rule::context => {
                    let mut context = patch_element.into_inner();
                    let context_header = context
                        .next()
                        .ok_or(Error::NotFound("context_header"))?;
                    let context_header = if let Rule::context_header = context_header.as_rule() {
                        Self::get_context_header(context_header)?
                    } else {
                        return Err(Error::MalformedPatch(
                            "Context header is not at the start of a context",
                        )
                        .into());
                    };
                    for i in file1_ptr..context_header.file1_l {
                        file2_text.push(
                            self.text
                                .get(i)
                                .ok_or(Error::AbruptInput(i))?
                                .to_owned(),
                        );
                    }
                    file1_ptr = context_header.file1_l;
                    for line in context {
                        match line.as_rule() {
                            Rule::line_context => {
                                if self
                                    .text
                                    .get(file1_ptr)
                                    .ok_or(Error::AbruptInput(file1_ptr))?
                                    != line.as_span().as_str()
                                {
                                    return Err(Error::PatchInputMismatch(file1_ptr).into());
                                }
                                file2_text.push(line.as_span().as_str().to_owned());
                                file1_ptr += 1;
                            }
                            Rule::line_deleted => {
                                if self
                                    .text
                                    .get(file1_ptr)
                                    .ok_or(Error::AbruptInput(file1_ptr))?
                                    != line.as_span().as_str()
                                {
                                    return Err(Error::PatchInputMismatch(file1_ptr).into());
                                }
                                file1_ptr += 1;
                            }
                            Rule::line_inserted => {
                                file2_text.push(line.as_span().as_str().to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        for i in file1_ptr..self.text.len() {
            file2_text.push(
                self.text
                    .get(i)
                    .ok_or(Error::AbruptInput(i))?
                    .to_owned(),
            );
        }

        Ok(file2_text)
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
