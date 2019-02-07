//!
//! The Patch context structures.
//!

mod flip;
mod reduce;
mod merge;

use std::fmt;

//use log::*;

use crate::line::Line;

#[derive(Default, Clone)]
pub struct Context {
    pub header: ContextHeader,
    pub data: Vec<Line>,
}

impl Context {
//    pub fn insert(&mut self, mut line: Line, after: usize) -> isize {
//        let mut i = match line {
//            Line::Delete(_) => self.array_index_for_insert(after),
//            _ => self.array_index_for_delete(after),
//        };
//        trace!("Inserting {} after {} at index {}", line, after, i);
//
//        if let (Line::Delete(delete), Some(Line::Insert(insert))) = (&line, self.data.get(i)) {
//            if delete == insert {
//                trace!("Dedup {}", i);
//                self.remove(i);
//                return self.insert(line, after);
//            }
//        }
//
//        if let (Line::Delete(delete), Some(Line::Context(context))) = (&line, self.data.get(i)) {
//            if context == delete {
//                trace!("Dedup {}", i);
//                self.remove(i);
//                return self.insert(line, after);
//            }
//        }
//
//        if let (Line::Delete(delete), Some(Line::Context(context))) = (&line, self.data.get(i)) {
//            if context == delete {
//                trace!("Dedup {}", i);
//                self.remove(i);
//                return self.insert(line, after);
//            }
//        }
//
//        if let (Line::Context(context), Some(Line::Delete(delete))) = (&line, self.data.get(i)) {
//            if context == delete {
//                trace!("Dedup {}", i);
//                return 0;
//            }
//        }
//
//        if let (Line::Context(context), Some(Line::Insert(insert))) = (&line, self.data.get(i)) {
//            if context == insert {
//                trace!("Dedup {}", i);
//                return 0;
//            }
//        }
//
//        if let (Line::Context(context1), Some(Line::Context(context2))) = (&line, self.data.get(i)) {
//            if context1 == context2 {
//                trace!("Dedup {}", i);
//                return 0;
//            }
//        }
//
//        if let Line::Delete(_) = line {
//            while let Some(Line::Insert(_)) = self.data.get(i) {
//                i -= 1;
//            }
//        }
//
//        if after == 0 {
//            self.header.file1_l -= 1;
//            self.header.file2_l -= 1;
//        }
//        match line {
//            Line::Context(_) => {
//                self.header.file1_s += 1;
//                self.header.file2_s += 1;
//            }
//            Line::Insert(_) => {
//                self.header.file2_s += 1;
//            }
//            Line::Delete(_) => {
//                self.header.file1_s += 1;
//            }
//        }
//        self.data.insert(i, line);
//        0
//    }

//    pub fn array_index_for_delete(&self, line_number: usize) -> usize {
//        let (mut i, mut l) = (0, self.header.file1_l);
//        while let Some(line) = self.data.get(i) {
//            match line {
//                Line::Context(_) | Line::Delete(_) => {
//                    l += 1;
//                },
//                Line::Insert(_) => {}
//            }
//            i += 1;
//            if l > line_number {
//                return i-1;
//            }
//        }
//        self.size()
//    }
//
//    pub fn array_index_for_insert(&self, line_number: usize) -> usize {
//        let (mut i, mut l) = (0, self.header.file1_l);
//        while let Some(line) = self.data.get(i) {
//            match line {
//                Line::Context(_) | Line::Insert(_) => {
//                    l += 1;
//                },
//                Line::Delete(_) => {}
//            }
//            i += 1;
//            if l > line_number {
//                return i-1;
//            }
//        }
//        self.size()
//    }

    pub fn pop_front(&mut self) -> Option<Line> {
        self.remove(0)
    }

    pub fn pop_back(&mut self) -> Option<Line> {
        self.remove(self.size()-1)
    }

    pub fn remove(&mut self, index: usize) -> Option<Line> {
        if index > self.size() {
            return None;
        }

        let line = self.data.remove(index);
        if index == 0 {
            self.header.file1_l += 1;
            self.header.file2_l += 1;
        }

        match line {
            Line::Context(_) => {
                self.header.file1_s -= 1;
                self.header.file2_s -= 1;
            }
            Line::Insert(_) => {
                self.header.file2_s -= 1;
            }
            Line::Delete(_) => {
                self.header.file1_s -= 1;
            }
        }

        Some(line)
    }

    pub fn shift(&mut self, offset: isize) {
        if offset > 0 {
            self.header.file1_l += offset as usize;
        }
        if offset < 0 {
            self.header.file1_l -= -offset as usize;
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn offset(&self) -> isize {
        (self.header.file2_s as isize) - (self.header.file1_s as isize)
    }

    pub fn opening_context_size(&self) -> usize {
        self.data.iter().take_while(|element| if let Line::Context(_) = element {
            true
        } else {
            false
        }).count()
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

    pub fn update(&mut self) {
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
