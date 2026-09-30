// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.1: the core of the evaluator. The `eval`/`apply`
//! interplay runs one checked program: a definition binds a procedure,
//! an application evaluates its operands and applies the operator, an
//! assignment rebinds a mutable binding, and the conditional branches
//! on the language's own truth, a typed `bool`.

use ch04::eval_support::trap_line;
use ch04::sec_4_1::{admit, run, run_source};
use sicp_runtime::host::diag::DiagKind;
use sicp_runtime::host::value::Trap;

const CORE: &str = "\
fn square(x: i64) -> i64 {
    x * x
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn twice(f: fn(i64, i64) -> i64, x: i64) -> i64 {
    f(x, x)
}

fn main() {
    println!(\"{}\", square(21));
    let mut n = 21;
    n = 2;
    println!(\"{}\", square(n));
    println!(\"{}\", twice(add, 4));
}
";

const CAPTURED: &str = "\
fn main() {
    let base = 4;
    let add_base = |x: i64| x + base;
    let shift = 38;
    println!(\"{}\", add_base(shift));
}
";

const BRANCH: &str = "\
fn classify(x: i64) -> i64 {
    if x == 0 {
        100
    } else {
        -x
    }
}

fn main() {
    println!(\"{}\", classify(0));
    println!(\"{}\", classify(7));
}
";

const STOPS: &str = "\
fn main() {
    println!(\"{}\", 36);
    let d = 0;
    println!(\"{}\", 441 / d);
}
";

fn main() {
    // The core interplay: a definition binds a procedure, each call
    // evaluates its operands and applies the operator, and an
    // assignment rebinds the mutable binding without creating one.
    let outcome = run_source(CORE).expect("admitted");
    println!("{}", outcome.stdout);
    // => 441
    // => 4
    // => 8
    assert_eq!(outcome.stdout, "441\n4\n8\n");
    assert!(outcome.trap.is_none());

    // Apply classifies its procedure: a function pointer calls its
    // handler directly, while a closure value carries the environment
    // of its definition into every call.
    let outcome = run_source(CAPTURED).expect("admitted");
    println!("{}", outcome.stdout);
    // => 42
    assert_eq!(outcome.stdout, "42\n");

    // The conditional tests the language's truth, which is a typed
    // `bool`: a non-bool condition is a typing rejection raised before
    // any effect, never a truthiness rule.
    let outcome = run_source(BRANCH).expect("admitted");
    println!("{}", outcome.stdout);
    // => 100
    // => -7
    assert_eq!(outcome.stdout, "100\n-7\n");
    let diag = admit("fn main() { if 0 { println!(\"no\"); } }").expect_err("not a bool");
    assert_eq!(diag.kind, DiagKind::Type);

    // Source is data until it runs: the front end checks a program
    // whose evaluation will stop at a trap and produces no effect
    // while doing so. Running it writes its prefix and answers the
    // trap on the one error channel.
    let checked = admit(STOPS).expect("checked as data");
    let outcome = run(&checked);
    println!("{}", outcome.stdout);
    // => 36
    assert_eq!(outcome.stdout, "36\n");
    assert!(matches!(
        outcome.trap.as_ref().map(|report| &report.trap),
        Some(Trap::DivByZero)
    ));
    println!("{}", trap_line(&outcome).expect("one trap line"));
    // => trap: DivByZero
    assert_eq!(trap_line(&outcome).as_deref(), Some("trap: DivByZero"));
}
