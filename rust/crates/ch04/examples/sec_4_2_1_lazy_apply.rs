// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Sections 4.2.1/4.2.2: the lazy application clause. Compound
//! procedures are non-strict in each argument -- their operands bind as
//! thunks -- while primitives stay strict and force every argument.
//! The operator, the `if` predicate, and the driver's printed value are
//! the demand sites that force.

use ch04::eval_support::{Lazy, printed, run_lazy};
use sicp_runtime::print_value;

fn main() {
    // The book's `try`: the armed operand `(/ 1 0)` is never demanded,
    // so the lazy evaluator answers 1 where Scheme raises.
    let (values, _) = run_lazy(
        &Lazy,
        "(define (try a b) (if (= a 0) 1 b))\n(try 0 (/ 1 0))",
    )
    .expect("runs");
    for value in &values {
        println!("{}", print_value(value));
    }
    // => ok
    // => 1
    assert_eq!(printed(&values), vec!["ok", "1"]);

    // `unless` as a procedure does useful work even when an arm would
    // raise: only the chosen arm is forced, and the strict primitive
    // rule is what forces it.
    let (values, displayed) = run_lazy(
        &Lazy,
        "(define (unless condition usual-value exceptional-value) \
         (if condition exceptional-value usual-value))\n\
         (unless (= 0 0) (/ 1 0) (begin (display \"exception: returning 0\") 0))",
    )
    .expect("runs");
    println!("{displayed}");
    // => exception: returning 0
    // => 0
    assert_eq!(displayed, "exception: returning 0");
    assert_eq!(printed(&values).last(), Some(&"0".to_owned()));

    // The operator is forced: `id`'s body answers a thunk of `+`, and
    // the application clause forces it before apply can dispatch.
    let (values, _) = run_lazy(&Lazy, "(define (id x) x)\n((id +) 2 3)").expect("runs");
    println!("{}", print_value(values.last().expect("a value")));
    // => 5
    assert_eq!(printed(&values).last(), Some(&"5".to_owned()));

    // `eval-if` forces the predicate: an unforced thunk would be
    // truthy, and the wrong branch would run.
    let (values, _) = run_lazy(&Lazy, "(define (id x) x)\n(if (id #f) 'yes 'no)").expect("runs");
    println!("{}", print_value(values.last().expect("a value")));
    // => no
    assert_eq!(printed(&values).last(), Some(&"no".to_owned()));

    // A compound procedure binds delayed operands: defining `w` runs
    // `id`'s set! once (the outer body), while the inner `(id 10)` is
    // still a thunk awaiting a demand.
    let (values, _) = run_lazy(
        &Lazy,
        "(define count 0)\n\
         (define (id x) (set! count (+ count 1)) x)\n\
         (define w (id (id 10)))\n\
         count",
    )
    .expect("runs");
    println!("{}", print_value(values.last().expect("a value")));
    // => 1
    assert_eq!(printed(&values).last(), Some(&"1".to_owned()));
}
