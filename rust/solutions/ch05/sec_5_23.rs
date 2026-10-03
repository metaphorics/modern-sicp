// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.23: derived branching forms
//! evaluated through explicit control.
//!
//! The host slice has no `cond` or `let` surface of its own; its
//! derived forms are the `if`/`else` chain, `match` over a closed
//! enum, and closure application for local bindings. Each desugars to
//! the core control transfers the 5.4 machine already runs, so the
//! rest of the evaluator never knows the forms existed. The solution
//! runs the same source through the explicit-control evaluator and
//! the compiler and checks both the answers and their agreement.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

/// Runs `source` on both engines, asserts neither traps, and answers
/// their shared transcript.
fn agreed(source: &str) -> String {
    let program = admitted(source);
    let interpreted = ch05::sec_5_4::Eceval::run(&program);
    let compiled = ch05::sec_5_5::compiled_run(&program);
    assert!(interpreted.trap.is_none(), "{interpreted:?}");
    assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    interpreted.stdout
}

mod ex_5_23 {
    //! Exercise 5.23: derived expressions evaluate through the core
    //! dispatch on both engines.

    use super::*;

    const BRANCHING: &str = "enum Shape {\n    Circle(i64),\n    Square(i64),\n}\n\nfn classify(shape: Shape) -> i64 {\n    match shape {\n        Shape::Circle(radius) => 3 * radius * radius,\n        Shape::Square(side) => side * side,\n    }\n}\n\nfn pick(n: i64) -> i64 {\n    if n == 0 {\n        0\n    } else {\n        if n == 1 {\n            10\n        } else {\n            70\n        }\n    }\n}\n\nfn main() {\n    println!(\"{}\", classify(Shape::Circle(2)));\n    println!(\"{}\", classify(Shape::Square(3)));\n    println!(\"{}\", pick(0));\n    println!(\"{}\", pick(1));\n    println!(\"{}\", pick(7));\n}\n";

    const LOCAL_BINDING: &str = "fn main() {\n    let apply: Box<dyn Fn(i64) -> i64 + 'static> = Box::new(|x: i64| x * 6);\n    println!(\"{}\", apply(7));\n}\n";

    /// The `if`-chain and the `match` answer every branch on both
    /// engines: `12`, `9`, then `0`, `10`, `70`.
    #[test]
    fn ex_5_23_derived_branches() {
        let transcript = agreed(BRANCHING);
        assert_eq!(transcript, "12\n9\n0\n10\n70\n", "{transcript}");
    }

    /// A closure application binds its local like the derived `let`:
    /// `7 * 6` answers `42` on both engines.
    #[test]
    fn ex_5_23_closure_binds_local() {
        let transcript = agreed(LOCAL_BINDING);
        assert_eq!(transcript, "42\n", "{transcript}");
    }
}
