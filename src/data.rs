//!
//! The Patch data structures.
//!

use std::fmt;

pub struct Patch {
    pub input: String,
    pub output: String,
    pub contexts: Vec<Context>,
}

#[derive(Default)]
pub struct Context {
    pub header: ContextHeader,
    pub data: Vec<PatchLine>,
}

impl Context {
    pub fn ends_with_context_lines(&self) -> usize {
        self.data.iter().rev().take_while(|element| if let PatchLine::Context(_) = element {
            true
        } else {
            false
        }).collect::<Vec<&PatchLine>>().len()
    }

    pub fn set_s_values(&mut self) {
        let mut s1 = 0;
        let mut s2 = 0;
        for line in self.data.iter() {
            match line {
                PatchLine::Context(_) => {
                    s1 += 1;
                    s2 += 1;
                },
                PatchLine::Insert(_) => s1 += 1,
                PatchLine::Delete(_) => s2 += 1,
            }
        }
        self.header.file1_s = s1;
        self.header.file2_s = s2;
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for line in self.data.iter() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Default, Clone, Copy)]
pub struct ContextHeader {
    pub file1_l: usize,
    pub file1_s: usize,
    pub file2_l: usize,
    pub file2_s: usize,
}

impl fmt::Display for ContextHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@@ -{},{} +{},{} @@", self.file1_l, self.file1_s, self.file2_l, self.file2_s)
    }
}

#[derive(Clone)]
pub enum PatchLine {
    Context(String),
    Insert(String),
    Delete(String),
}

impl PatchLine {
    pub fn flip(&self) -> Self {
        match self {
            PatchLine::Context(line) => PatchLine::Context(line.clone()),
            PatchLine::Insert(line) => PatchLine::Delete(line.clone()),
            PatchLine::Delete(line) => PatchLine::Insert(line.clone()),
        }
    }
}

impl fmt::Display for PatchLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PatchLine::Context(line) => write!(f, " {}", line),
            PatchLine::Insert(line) => write!(f, "+{}", line),
            PatchLine::Delete(line) => write!(f, "-{}", line),
        }
    }
}
