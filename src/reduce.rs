//!
//! The context reducing algorithm.
//!

use std::collections::VecDeque;

use log::*;

use crate::{
    context::{Context, ContextHeader},
    line::Line,
};

enum FlipState {
    StartContext,
    Context,
}

impl Context {
    pub fn reduce(&self) -> VecDeque<Self> {
        let mut results = VecDeque::new();

        let mut output = Self::default();
        output.header.file1_l = self.header.file1_l;
        output.header.file2_l = self.header.file2_l;

        let mut state = FlipState::StartContext;

        trace!("START");
        for line in self.data.iter() {
            match line {
                Line::Context(_) => {
                    match state {
                        FlipState::StartContext => {
                            trace!("Context StartContext");
                            if output.closing_context_size() >= 1 {
                                output.data.pop();
                                output.header.file1_l += 1;
                                output.header.file2_l += 1;
                                trace!("POP START");
                            }
                            output.data.push(line.clone());
                        },
                        FlipState::Context => {
                            trace!("Context Context");
                            output.data.push(line.clone());
                            let lines = output.closing_context_size();
                            if lines > 2 {
                                let mut data = Vec::new();
                                data.push(output.data.pop().unwrap());
                                for _ in 2..lines {
                                    output.data.pop();
                                    trace!("POP END");
                                }
                                output.set_s_values();

                                trace!("PUSH OUTPUT");
                                let output_next = Context {
                                    header: ContextHeader {
                                        file1_l: output.header.file1_l + output.header.file1_s + lines - 2,
                                        file1_s: Default::default(),
                                        file2_l: output.header.file2_l + output.header.file2_s + lines - 2,
                                        file2_s: Default::default(),
                                    },
                                    data,
                                };

                                if output.has_changes() {
                                    results.push_back(output);
                                }
                                output = output_next;
                            }
                        },
                    }
                },
                Line::Delete(_) | Line::Insert(_) => {
                    if let FlipState::StartContext = state {
                        state = FlipState::Context;
                    }
                    output.data.push(line.clone());
                },
            }
        }

        let lines = output.closing_context_size();
        if lines > 1 {
            for _ in 1..lines {
                output.data.pop();
                trace!("POP END");
            }
        }

        output.set_s_values();
        if output.has_changes() {
            results.push_back(output);
        }

        trace!("END");

        results
    }
}
