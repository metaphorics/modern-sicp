// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Section 4.2.2/4.2.3: the lazy driver loop and lazy lists. The
//! run's outcome record is the driver: it answers the demanded
//! integer, the ordered effect log, and the rendered form of the
//! produced value, so a delayed value propagated back to the driver
//! shows its raw cell until a demand site forces it. Lazy pairs make
//! streams and lists identical: both slots of the pair are delayed,
//! so a list's elements compute only when a walk demands them.

use ch04::sec_4_2::{LazyEngine, LazyExpr, LazyVal, Mode, PrimOp, reference_model};

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

fn call(operator: LazyExpr, args: Vec<LazyExpr>) -> LazyExpr {
    LazyExpr::Apply(Box::new(operator), args)
}

fn prim(op: PrimOp) -> LazyExpr {
    LazyExpr::Quote(LazyVal::Prim(op))
}

fn pair(first: LazyExpr, rest: LazyExpr) -> LazyExpr {
    LazyExpr::Pair(Box::new(first), Box::new(rest))
}

fn first_of(items: LazyExpr) -> LazyExpr {
    force(call(prim(PrimOp::First), vec![items]))
}

fn rest_of(items: LazyExpr) -> LazyExpr {
    force(call(prim(PrimOp::Rest), vec![items]))
}

/// Runs one program on the memo engine, printing the driver's outcome
/// record and cross-checking the finite reference model.
fn agree(program: &LazyExpr) -> ch04::sec_4_2::LazyOutcome {
    let outcome = LazyEngine::new(Mode::Memo).run(program);
    println!(
        "value={:?} effects={:?} rendered={:?}",
        outcome.value, outcome.effects, outcome.rendered
    );
    assert_eq!(outcome, reference_model(Mode::Memo, program));
    outcome
}

/// The demand chain the lazy pair walk folds over: `n` rest steps.
fn walk(mut items: LazyExpr, steps: usize) -> LazyExpr {
    for _ in 0..steps {
        items = rest_of(items);
    }
    first_of(items)
}

/// One delayed list element that logs when, and only when, a demand
/// reaches it.
fn element(id: usize, value: i64) -> LazyExpr {
    LazyExpr::Thunk(id, Box::new(bind("_", emit("compute"), int(value))))
}

/// The driver's demand discipline: a program that answers a delayed
/// value shows the raw cell in the rendered form, and the same
/// program answered through a demand site reports the integer.
fn demand_discipline() {
    let raw = bind(
        "w",
        LazyExpr::Thunk(0, Box::new(bind("_", emit("compute"), int(10)))),
        var("w"),
    );
    let outcome = agree(&raw);
    // => value=None effects=[] rendered=["<thunk 0>"]
    assert_eq!(outcome.value, None);
    assert_eq!(outcome.rendered, ["<thunk 0>"]);

    let demanded = bind(
        "w",
        LazyExpr::Thunk(0, Box::new(bind("_", emit("compute"), int(10)))),
        force(var("w")),
    );
    let outcome = agree(&demanded);
    // => value=Some(10) effects=["compute"] rendered=["10"]
    assert_eq!(outcome.value, Some(10));
    assert_eq!(outcome.effects, ["compute"]);
}

/// `list_ref` at 17 over a lazy list whose elements AND tails are
/// delayed: building the spine runs nothing, and the walk demands
/// exactly the eighteenth element plus the tails it steps over.
fn delayed_list_ref() {
    let mut spine = LazyExpr::Empty;
    for k in (1..=18_usize).rev() {
        spine = pair(
            element(2 * k, i64::try_from(k).expect("k is at most 18")),
            LazyExpr::Thunk(2 * k + 1, Box::new(spine)),
        );
    }
    let program = bind("xs", spine, walk(var("xs"), 17));
    let outcome = agree(&program);
    // => value=Some(18) effects=["compute"] rendered=["18"]
    assert_eq!(outcome.value, Some(18));
    assert_eq!(outcome.effects, ["compute"]);
}

/// The integral of 4.2.3 over a delayed spine: each step's value is
/// a thunk over its predecessor, so demanding the last one computes
/// the whole prefix, in order, once.
fn integration_chain() {
    let step = |k: usize, prev: &str| {
        bind(
            "prev",
            force(var(prev)),
            bind(
                "_",
                emit(&format!("step {k}")),
                bind(
                    "dy",
                    LazyExpr::Mul(Box::new(int(1)), Box::new(var("prev"))),
                    LazyExpr::Add(Box::new(var("prev")), Box::new(var("dy"))),
                ),
            ),
        )
    };
    let program = bind(
        "y0",
        int(1),
        bind(
            "y1",
            LazyExpr::Thunk(0, Box::new(step(1, "y0"))),
            bind(
                "y2",
                LazyExpr::Thunk(1, Box::new(step(2, "y1"))),
                bind(
                    "y3",
                    LazyExpr::Thunk(2, Box::new(step(3, "y2"))),
                    bind(
                        "y4",
                        LazyExpr::Thunk(3, Box::new(step(4, "y3"))),
                        force(var("y4")),
                    ),
                ),
            ),
        ),
    );
    let outcome = agree(&program);
    // => value=Some(16) effects=["step 1", "step 2", "step 3", "step 4"] rendered=["16"]
    assert_eq!(outcome.value, Some(16));
    assert_eq!(outcome.effects, ["step 1", "step 2", "step 3", "step 4"]);
}

/// `solve` as originally intended in 3.5.4: no explicit delay. The
/// experiment language computes with `i64` only (grammar §3 has no
/// floating point), so the session's printed value is maintained
/// through the host's own native arithmetic: dy/dt = y, y(0) = 1,
/// dt = 0.001, demanded at 1000.
fn solve_experiment() {
    let mut y = 1.0_f64;
    for _ in 0..1000 {
        y += y * 0.001;
    }
    println!("solve={y}");
    // => solve=2.716923932235896
    // The loop is one fixed chain of IEEE-754 multiply/add steps, so
    // the final bits are exactly reproducible, and Rust's `{}` float
    // formatting is the shortest decimal that parses back to those
    // exact bits; pinning the printed transcript therefore pins the
    // bits, with no float equality needed.
    assert_eq!(y.to_string(), "2.716923932235896");
}

fn main() {
    demand_discipline();
    delayed_list_ref();
    integration_chain();
    solve_experiment();
}
