// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Sections 4.2.1/4.2.2: the lazy application clause. Compound
//! procedures are non-strict in each argument -- their operands bind
//! as delayed thunks -- while primitives stay strict and demand every
//! argument. The operator, the `if` predicate, and each primitive slot
//! carry the demand sites that force.

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

fn branch(condition: LazyExpr, then: LazyExpr, otherwise: LazyExpr) -> LazyExpr {
    LazyExpr::If(Box::new(condition), Box::new(then), Box::new(otherwise))
}

fn call(operator: LazyExpr, args: Vec<LazyExpr>) -> LazyExpr {
    LazyExpr::Apply(Box::new(operator), args)
}

fn closure_expr(params: &[&str], body: LazyExpr) -> LazyExpr {
    LazyExpr::Lambda(
        params.iter().map(|name| (*name).to_owned()).collect(),
        Box::new(body),
    )
}

fn prim(op: PrimOp) -> LazyExpr {
    LazyExpr::Quote(LazyVal::Prim(op))
}

/// The book's `id`: answers its argument without demanding it.
fn id() -> LazyExpr {
    closure_expr(&["x"], var("x"))
}

/// The book's armed operand: demanding it stops the run.
fn armed() -> LazyExpr {
    force(var("no_such_binding"))
}

/// The book's `try`, where `a` equal to 0 answers 1 and never demands
/// the armed `b`; `a` nonzero demands `b` and runs its body.
fn try_call(a: i64, armed: LazyExpr) -> LazyExpr {
    call(
        closure_expr(
            &["a", "b"],
            branch(force(var("a")), force(var("b")), int(1)),
        ),
        vec![int(a), armed],
    )
}

fn agree(program: &LazyExpr) {
    let outcome = LazyEngine::new(Mode::Memo).run(program);
    println!(
        "value={:?} effects={:?} rendered={:?}",
        outcome.value, outcome.effects, outcome.rendered
    );
    assert_eq!(outcome, reference_model(Mode::Memo, program));
}

fn main() {
    // The book's `try`: the armed operand is never demanded, so the
    // effect its body would log never appears and the lazy evaluator
    // answers 1 where a strict evaluator would run the body.
    let probed = try_call(0, emit("b was evaluated"));
    agree(&probed);
    // => value=Some(1) effects=[] rendered=["1"]
    assert_eq!(LazyEngine::new(Mode::Memo).run(&probed).value, Some(1));

    // The same call armed with a body that stops the run: still 1,
    // because non-strictness is the operand rule, not a missing error.
    let stopped = try_call(0, armed());
    agree(&stopped);
    // => value=Some(1) effects=[] rendered=["1"]
    assert_eq!(LazyEngine::new(Mode::Memo).run(&stopped).value, Some(1));

    // With `a` nonzero the armed operand is demanded and the run stops
    // at it.
    let demanded = try_call(1, armed());
    agree(&demanded);
    // => value=None effects=[] rendered=[]
    assert_eq!(LazyEngine::new(Mode::Memo).run(&demanded).value, None);

    // `unless` as a procedure does useful work even when the other arm
    // would stop the run: only the chosen arm is demanded, and its
    // display effect precedes the value.
    let unless = closure_expr(
        &["c", "u", "e"],
        branch(force(var("c")), force(var("e")), force(var("u"))),
    );
    let program = bind(
        "shown",
        call(
            unless,
            vec![
                int(1),
                armed(),
                bind("_", emit("exception: returning 0"), int(0)),
            ],
        ),
        var("shown"),
    );
    agree(&program);
    // => value=Some(0) effects=["exception: returning 0"] rendered=["0"]
    let outcome = LazyEngine::new(Mode::Memo).run(&program);
    assert_eq!(outcome.value, Some(0));
    assert_eq!(outcome.effects, ["exception: returning 0"]);

    // The operator is demanded before apply can dispatch: `id`'s body
    // answers a thunk of `+`, and the demand site forces it first.
    let program = call(
        force(call(id(), vec![prim(PrimOp::Add)])),
        vec![int(2), int(3)],
    );
    agree(&program);
    // => value=Some(5) effects=[] rendered=["5"]
    assert_eq!(LazyEngine::new(Mode::Memo).run(&program).value, Some(5));

    // The `if` predicate is demanded before the branch is chosen: an
    // undemanded thunk decides nothing.
    let picks = |truth: LazyExpr| branch(force(call(id(), vec![truth])), emit("yes"), emit("no"));
    agree(&picks(LazyExpr::Quote(LazyVal::Now(0))));
    // => value=Some(0) effects=["no"] rendered=["0"]
    assert_eq!(
        LazyEngine::new(Mode::Memo)
            .run(&picks(LazyExpr::Quote(LazyVal::Now(0))))
            .effects,
        ["no"]
    );
    agree(&picks(LazyExpr::Quote(LazyVal::Now(1))));
    // => value=Some(0) effects=["yes"] rendered=["0"]
    assert_eq!(
        LazyEngine::new(Mode::Memo)
            .run(&picks(LazyExpr::Quote(LazyVal::Now(1))))
            .effects,
        ["yes"]
    );

    // A compound procedure binds delayed operands: defining `w` runs
    // the outer body once (the probe reads 1), while the inner call is
    // still a thunk awaiting a demand. The body answers its argument's
    // binding unforced, so the value of `w` is the inner thunk itself.
    let ticked_id = closure_expr(&["x"], bind("_", emit("tick"), var("x")));
    let w = call(ticked_id.clone(), vec![call(ticked_id, vec![int(10)])]);
    let program = bind("w", w.clone(), int(0));
    agree(&program);
    // => value=Some(0) effects=["tick"] rendered=["0"]
    assert_eq!(LazyEngine::new(Mode::Memo).run(&program).effects, ["tick"]);

    // Demanding `w` runs the inner body exactly once more: the probe
    // now reads 2 and the value is 10.
    let program = bind("w", w, force(force(var("w"))));
    agree(&program);
    // => value=Some(10) effects=["tick", "tick"] rendered=["10"]
    let outcome = LazyEngine::new(Mode::Memo).run(&program);
    assert_eq!(outcome.value, Some(10));
    assert_eq!(outcome.effects, ["tick", "tick"]);
}
