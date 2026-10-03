// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.6: internal definitions. Bindings execute in sequence,
//! each extending the newest scope one name at a time; the section's
//! point is that this agrees with simultaneous definition whenever the
//! bindings come first and their initializers do not read the names
//! being defined, as in the mutually recursive `even`/`odd` pair.

use ch04::sec_4_1::run_program;
use sicp_runtime::host::diag::DiagKind;
use sicp_runtime::host::{admit, check_program, parse_program};

const MUTUAL: &str = "\
fn even(n: i64) -> bool {
    if n == 0 {
        true
    } else {
        odd(n - 1)
    }
}

fn odd(n: i64) -> bool {
    if n == 0 {
        false
    } else {
        even(n - 1)
    }
}

fn main() {
    println!(\"{}\", even(10));
    println!(\"{}\", even(7));
}
";

const MIXED: &str = "\
fn main() {
    let base = 100;
    let scaled = 5 * 2;
    println!(\"{}\", base + scaled);
}
";

fn main() {
    // Mutual recursion works, for the reason the section names: no
    // call runs before both names exist, and top-level functions may
    // call each other in either order.
    let output = run_program(MUTUAL);
    println!("{output}");
    // => true
    // => false
    assert_eq!(output, "true\nfalse\n");

    // A name read before its binding runs is not merely unbound at
    // runtime: the checker rejects the program before any effect,
    // which is the sequential mechanism's one visible difference from
    // simultaneous definition made static.
    let premature = "\
fn main() {
    let a = b * 2;
    let b = 3;
    println!(\"{}\", a);
}
";
    let parsed = parse_program(premature).expect("valid syntax");
    let diag = check_program(&parsed).expect_err("b is not yet bound");
    println!("{:?}", diag.kind);
    // => Type
    assert!(matches!(diag.kind, DiagKind::Type | DiagKind::Ownership));
    assert!(admit(premature).is_err());

    // Internal bindings mix freely with the body's other statements,
    // each one extending the scope in order.
    let output = run_program(MIXED);
    println!("{output}");
    // => 110
    assert_eq!(output, "110\n");
}
