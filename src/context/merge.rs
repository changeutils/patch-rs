//!
//! The context merging algorithm.
//!

//use std::collections::VecDeque;
//
//use log::*;
//
//use crate::{
//    context::{Context, ContextHeader},
//    line::Line,
//};
//
//impl Context {
//    pub fn merge(self, other: Self) -> Self {
//        let (mut first, mut second) = if self.header.file1_l <= other.header.file1_l {
//            (self, other)
//        } else {
//            (other, self)
//        };
//        if first.header.file1_l + first.header.file1_s < second.header.file1_l {
//            panic!("ERROR 1");
//        }
//
//        trace!("Element 1: \n{}", first);
//        trace!("Element 2: \n{}", second);
//
//        let mut sum = first.clone();
//        for i in 0..second.size() {
//            let index = second.header.file1_l;
//            if let Some(line) = second.pop_front() {
//                let shift = sum.insert(line, index);
//                second.shift(shift);
//                trace!("Intermediate: \n{}", sum);
//            }
//        }
//        sum
//    }
//}
