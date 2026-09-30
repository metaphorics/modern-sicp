// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Section 4.2.2: representing thunks. The book's
//! `thunk`/`evaluated-thunk` pair collapses into the two states of one
//! cell: `Later` holds the thunk identity, the memo table holds the
//! computed value under that identity. Forcing runs the body once and
//! every later demand answers the memoized value; in the recomputing
//! experiment the body runs again at every demand. A thunk that is
//! never demanded stays delayed, which is why the driver's demand
//! sites are what keep raw cells out of the printed answers.

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode, reference_model};

fn int(value: i64) -> LazyExpr {
    LazyExpr::Int(value)
}

fn var(name: &str) -> LazyExpr {
    LazyExpr::Var(name.to_owned())
}

fn force(expr: LazyExpr) -> LazyExpr {
    LazyExpr::Force(Box::new(expr))
}

fn bind(name: &str, value: LazyExpr, body: LazyExpr) -> LazyExpr {
    LazyExpr::Let(name.to_owned(), Box::new(value), Box::new(body))
}

fn emit(tag: &str) -> LazyExpr {
    LazyExpr::Emit(tag.to_owned())
}

fn pair(first: LazyExpr, rest: LazyExpr) -> LazyExpr {
    LazyExpr::Pair(Box::new(first), Box::new(rest))
}

/// The example's printer: this binary's own CLI transcript output.
fn show(label: &str, outcome: &ch04::sec_4_2::LazyOutcome) {
    println!(
        "{label}: value={:?} effects={:?} rendered={:?}",
        outcome.value, outcome.effects, outcome.rendered
    );
}

/// Runs one program in one named mode, printing the outcome record and
/// cross-checking the independent finite reference model.
fn agree(label: &str, mode: Mode, program: &LazyExpr) -> ch04::sec_4_2::LazyOutcome {
    let outcome = LazyEngine::new(mode).run(program);
    show(label, &outcome);
    assert_eq!(outcome, reference_model(mode, program));
    outcome
}

fn main() {
    // One cell, two demands: `(+ 20 22)` under a probe. The memo mode
    // runs the body once and both demands answer 42; the recomputing
    // mode runs the body at every demand.
    let twice = LazyExpr::Thunk(
        0,
        Box::new(bind(
            "_",
            emit("compute"),
            LazyExpr::Add(Box::new(int(20)), Box::new(int(22))),
        )),
    );
    let program = bind("t", twice, pair(force(var("t")), force(var("t"))));

    let memo = agree("memo", Mode::Memo, &program);
    // => memo: value=None effects=["compute"] rendered=["(42 . 42)"]
    assert_eq!(memo.effects, ["compute"]);
    assert_eq!(memo.rendered, ["(42 . 42)"]);

    let recompute = agree("recompute", Mode::Recompute, &program);
    // => recompute: value=None effects=["compute", "compute"] rendered=["(42 . 42)"]
    assert_eq!(recompute.effects, ["compute", "compute"]);
    assert_eq!(recompute.rendered, ["(42 . 42)"]);

    // The strict primitive's two demand sites: `+` forces both of its
    // operand's demands, and the memoized cell answers the one value
    // with one body run. The probe stands for the body executions the
    // book's `id` counts.
    let delayed = LazyExpr::Thunk(0, Box::new(bind("_", emit("tick"), int(5))));
    let program = bind(
        "v",
        delayed,
        LazyExpr::Add(Box::new(force(var("v"))), Box::new(force(var("v")))),
    );
    let memo = agree("memo", Mode::Memo, &program);
    // => memo: value=Some(10) effects=["tick"] rendered=["10"]
    assert_eq!(memo.value, Some(10));
    assert_eq!(memo.effects, ["tick"]);

    let recompute = agree("recompute", Mode::Recompute, &program);
    // => recompute: value=Some(10) effects=["tick", "tick"] rendered=["10"]
    assert_eq!(recompute.value, Some(10));
    assert_eq!(recompute.effects, ["tick", "tick"]);

    // What the driver's demand prevents: the unforced cell renders as
    // the raw thunk identity, while the demanded program answers the
    // integer.
    let raw = bind("w", LazyExpr::Thunk(0, Box::new(int(42))), var("w"));
    let outcome = agree("raw", Mode::Memo, &raw);
    // => raw: value=None effects=[] rendered=["<thunk 0>"]
    assert_eq!(outcome.value, None);
    assert_eq!(outcome.rendered, ["<thunk 0>"]);

    let demanded = bind("w", LazyExpr::Thunk(0, Box::new(int(42))), force(var("w")));
    let outcome = agree("demanded", Mode::Memo, &demanded);
    // => demanded: value=Some(42) effects=[] rendered=["42"]
    assert_eq!(outcome.value, Some(42));
    assert_eq!(outcome.rendered, ["42"]);
}
