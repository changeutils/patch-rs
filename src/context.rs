//!
//! The Patch context structures.
//!

use std::fmt;

use crate::line::Line;

#[derive(Default)]
pub struct Context {
    pub header: ContextHeader,
    pub data: Vec<Line>,
}

impl Context {
    pub fn offset(&self) -> isize {
        (self.header.file2_s as isize) - (self.header.file1_s as isize)
    }

    pub fn shift(&mut self, offset: isize) {
        if offset > 0 {
            self.header.file1_l -= offset as usize;
        }
        if offset < 0 {
            self.header.file1_l += -offset as usize;
        }
    }

    pub fn closing_context_size(&self) -> usize {
        self.data.iter().rev().take_while(|element| if let Line::Context(_) = element {
            true
        } else {
            false
        }).count()
    }

    pub fn has_changes(&self) -> bool {
        for line in self.data.iter() {
            match line {
                Line::Context(_) => continue,
                _ => return true,
            }
        }
        false
    }

    pub fn set_s_values(&mut self) {
        let mut s1 = 0;
        let mut s2 = 0;
        for line in self.data.iter() {
            match line {
                Line::Context(_) => {
                    s1 += 1;
                    s2 += 1;
                },
                Line::Insert(_) => s2 += 1,
                Line::Delete(_) => s1 += 1,
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
