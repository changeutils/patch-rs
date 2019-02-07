//!
//! The context flipping algorithm.
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
    Buffering,
}

impl Context {
    pub fn flip(&self) -> VecDeque<Self> {
        let mut results = VecDeque::new();

        let mut output = Self::default();
        output.header.file1_l = self.header.file2_l;
        output.header.file2_l = self.header.file1_l;

        let mut state = FlipState::StartContext;
        let mut deletes = Vec::new();
        let mut inserts = Vec::new();

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
                                output.update();

                                trace!("PUSH OUTPUT");
                                let output_next = Context {
                                    header: ContextHeader {
                                        file1_l: output.header.file2_l + output.header.file2_s + lines - 2,
                                        file1_s: Default::default(),
                                        file2_l: output.header.file1_l + output.header.file1_s + lines - 2,
                                        file2_s: Default::default(),
                                    },
                                    data,
                                };

                                if output.has_changes() {
                                    results.push_back(output);
                                }
                                output = output_next;

                                state = FlipState::Context;
                            }
                        },
                        FlipState::Buffering => {
                            trace!("Context Buffering");
                            output.data.append(&mut deletes);
                            output.data.append(&mut inserts);
                            state = FlipState::Context;
                            output.data.push(line.clone());
                        },
                    }
                },
                Line::Delete(_) => {
                    if let FlipState::StartContext = state {
                        trace!("Delete StartContext");
                        state = FlipState::Buffering;
                    }
                    if let FlipState::Context = state {
                        trace!("Delete Context");
                        state = FlipState::Buffering;
                    }
                    if let FlipState::Buffering = state {
                        trace!("Delete Buffering");
                        inserts.push(line.flip());
                    }
                },
                Line::Insert(_) => {
                    trace!("INSERT");
                    if let FlipState::StartContext = state {
                        trace!("Insert StartContext");
                        state = FlipState::Buffering;
                    }
                    if let FlipState::Context = state {
                        trace!("Insert Context");
                        state = FlipState::Buffering;
                    }
                    if let FlipState::Buffering = state {
                        trace!("Insert Buffering");
                        deletes.push(line.flip());
                    }
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

        output.update();
        if output.has_changes() {
            results.push_back(output);
        }

        trace!("END");

        results
    }
}
