// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.49: a host-driven
//! admit-and-run loop.
//!
//! Each interaction of the session is a complete guest program: the
//! loop admits it, runs it on both engines, and prints the shared
//! transcript before moving to the next. Definitions report through
//! their uses — `square(12)` answers `144`, `twice(441)` answers
//! `882` — and the factorial interaction answers `120`. No evaluator
//! dispatch runs the user forms; every line of output comes from an
//! admitted program's own run.

fn interact(source: &str) -> String {
    let program = match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("session admit: {}", diag.message),
    };
    let interpreted = ch05::sec_5_4::Eceval::run(&program);
    let compiled = ch05::sec_5_5::compiled_run(&program);
    assert!(interpreted.trap.is_none(), "{interpreted:?}");
    assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    interpreted.stdout
}

mod ex_5_49 {
    //! Exercise 5.49: the loop's session prints `144`, `882`, and
    //! `120` in order.

    use super::*;

    const SQUARE: &str = "fn square(n: i64) -> i64 {\n    n * n\n}\n\nfn main() {\n    println!(\"{}\", square(12));\n}\n";

    const TWICE: &str = "fn twice(n: i64) -> i64 {\n    n + n\n}\n\nfn main() {\n    println!(\"{}\", twice(441));\n}\n";

    const FACTORIAL: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    /// Three admitted interactions run in order and answer `144`,
    /// `882`, and `120`.
    #[test]
    fn ex_5_49_session_runs_in_order() {
        let session = [interact(SQUARE), interact(TWICE), interact(FACTORIAL)];
        assert_eq!(session, ["144\n", "882\n", "120\n"]);
    }
}
