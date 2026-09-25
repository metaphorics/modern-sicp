// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Sections 4.2.2: representing thunks. The book's
//! `thunk`/`evaluated-thunk` pair collapses into the two states of one
//! `Rc`-shared cell: `Delayed` holds the expression and its
//! environment, `Forced` holds the memoized value. Forcing runs the
//! expression once, and the driver forces before printing, so a thunk
//! never reaches the printer.

use ch04::eval_support::{
    Lazy, OutputSink, Value, delay_it, force_memo, lazy_driver_transcript, printed, read, run_with,
    setup_lazy_environment_in,
};
use sicp_runtime::print_value;

fn main() {
    // `delay-it` builds the cell; the driver's `actual-value` forces
    // before printing, so `w` displays as 10, never as `#[thunk]`.
    let transcript = lazy_driver_transcript(
        &Lazy,
        &[
            "(define count 0)",
            "(define (id x) (set! count (+ count 1)) x)",
            "(define w (id (id 10)))",
            "count",
            "w",
            "count",
        ],
    );
    println!("{transcript}");
    // => ;;; L-Eval value: 1
    // => ;;; L-Eval value: 10
    // => ;;; L-Eval value: 2
    assert!(transcript.contains(";;; L-Eval value: 1\n"));
    assert!(transcript.contains(";;; L-Eval value: 10\n"));
    assert!(transcript.contains(";;; L-Eval value: 2\n"));

    // Forcing the same thunk twice runs the body once: the cell
    // memoizes. `share` binds one delayed `(id 5)` and feeds it to both
    // slots of `+`, the two demand sites of the strict primitive.
    let (values, _) = run_with(
        &Lazy,
        "(define count 0)\n\
         (define (id x) (set! count (+ count 1)) x)\n\
         (define (share v) (+ v v))\n\
         (share (id 5))\n\
         count",
    )
    .expect("runs");
    for value in &values {
        println!("{}", print_value(value));
    }
    // => ok
    // => ok
    // => ok
    // => 10
    // => 1
    assert_eq!(printed(&values).last(), Some(&"1".to_owned()));

    // The two states of one cell, at the host level: `delay-it` builds
    // `Delayed`, one forcing fills it as `Forced`, and the memo answers
    // the same value ever after.
    let (sink, _cell) = OutputSink::buffer();
    let env = setup_lazy_environment_in(&sink);
    let exp = read("(+ 20 22)").expect("reads");
    let thunk = delay_it(exp, &env);
    assert!(matches!(thunk, Value::Thunk(_)));
    let once = force_memo(&Lazy, thunk.clone()).expect("forces");
    let twice = force_memo(&Lazy, thunk).expect("forces");
    println!("{} {}", print_value(&once), print_value(&twice));
    // => 42 42
    assert_eq!(once, twice, "the memoized cell answers the one value");

    // What the driver prevents: an unforced thunk would print as the
    // raw cell, which is why the driver runs `actual-value` first.
    let thunk = delay_it(read("(+ 20 22)").expect("reads"), &env);
    println!("{}", print_value(&thunk));
    // => #[thunk]
    assert_eq!(print_value(&thunk), "#[thunk]");
}
