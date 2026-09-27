// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.5 and 4.4.4.6: the indexed, chronological data base
//! and the stream operations, including the undelayed comparison pairs
//! the exercises analyze.

use std::rc::Rc;

use ch04::sec_4_4::{
    Frame, StepFn, interleave_delayed, microshaft, query_syntax_process, singleton_stream,
    stream_append_delayed, stream_flatmap,
};
use sicp_runtime::{Stream, Value};

fn main() {
    let engine = microshaft();

    // The index answers the supervisor bucket directly; a variable-led
    // pattern falls back to the whole data base.
    assert_eq!(
        engine
            .fetch_assertions(&read_query("(supervisor ?x ?y)"))
            .iter()
            .count(),
        8
    );
    assert_eq!(
        engine
            .fetch_assertions(&read_query("(?relation (Bitdiddle Ben) ?y)"))
            .iter()
            .count(),
        39
    );

    // The stream operations of 4.4.4.6 over frames.
    let a = || singleton_stream(Frame::new().extend(Value::sym("k"), Value::int(1)));
    let b = || singleton_stream(Frame::new().extend(Value::sym("k"), Value::int(2)));
    let appended = stream_append_delayed(a(), Box::new(b));
    assert_eq!(appended.iter().count(), 2);
    let interleaved = interleave_delayed(a(), Box::new(b));
    assert_eq!(interleaved.iter().count(), 2);

    // stream-flatmap interleave: the combined stream carries every
    // element of every inner stream.
    let doubled = {
        let step: StepFn<Frame, Frame> = Rc::new(|frame: &Frame| {
            let first = frame.clone();
            let second = frame.clone();
            Stream::cons_stream(first, move || singleton_stream(second))
        });
        stream_flatmap(step, singleton_stream(Frame::new()))
    };
    assert_eq!(doubled.iter().count(), 2);
}

fn read_query(text: &str) -> Value {
    query_syntax_process(&sicp_runtime::read(text).expect("parses"))
}
