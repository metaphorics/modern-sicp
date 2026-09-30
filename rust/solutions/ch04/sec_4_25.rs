// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.25: `unless` works when its
//! arms are delayed; the strict reading forces the failing arm.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

fn unless(test: bool, normal: LazyExpr, exceptional: LazyExpr) -> LazyExpr {
    LazyExpr::If(
        Box::new(LazyExpr::Int(i64::from(test))),
        Box::new(exceptional),
        Box::new(normal),
    )
}

#[test]
fn ex_4_25() {
    let failing = LazyExpr::Force(Box::new(LazyExpr::Thunk(
        1,
        Box::new(LazyExpr::Add(
            Box::new(LazyExpr::Var("missing".to_owned())),
            Box::new(LazyExpr::Int(1)),
        )),
    )));
    let lazy_unless = unless(false, LazyExpr::Int(120), failing.clone());
    let lazy_outcome = LazyEngine::new(Mode::Memo).run(&lazy_unless);
    assert_eq!(lazy_outcome.value, Some(120));

    let strict = LazyExpr::Apply(
        Box::new(LazyExpr::Lambda(
            vec!["a".to_owned(), "b".to_owned()],
            Box::new(LazyExpr::Add(
                Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("a".to_owned())))),
                Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("b".to_owned())))),
            )),
        )),
        vec![LazyExpr::Int(120), failing],
    );
    let strict_outcome = LazyEngine::new(Mode::Memo).run(&strict);
    assert_eq!(strict_outcome.value, None);
}
