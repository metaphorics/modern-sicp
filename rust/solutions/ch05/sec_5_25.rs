// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.25: deferred evaluation with
//! explicit thunks.
//!
//! The host slice is strict, so laziness is expressed the way the
//! book's controller expresses it: an argument becomes a value whose
//! body runs only when forced. A boxed closure is that thunk. The
//! strict evaluator would die evaluating the never-used argument; the
//! thunk version never forces it and answers. Forcing still traps
//! exactly where the strict program would, so the semantics agree on
//! every demanded value.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn agreed(source: &str) -> String {
    let program = admitted(source);
    let interpreted = ch05::sec_5_4::Eceval::run(&program);
    let compiled = ch05::sec_5_5::compiled_run(&program);
    assert!(interpreted.trap.is_none(), "{interpreted:?}");
    assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    interpreted.stdout
}

mod ex_5_25 {
    //! Exercise 5.25: unused thunked arguments never run; demanded
    //! ones answer exactly.

    use super::*;

    const THUNKS: &str = "fn div(a: i64, b: i64) -> i64 {\n    a / b\n}\n\nfn choose(first: bool, left: Box<dyn Fn() -> i64 + 'static>, right: Box<dyn Fn() -> i64 + 'static>) -> i64 {\n    if first {\n        left()\n    } else {\n        right()\n    }\n}\n\nfn main() {\n    println!(\"{}\", choose(true, Box::new(|| 42), Box::new(|| div(1, 0))));\n    println!(\"{}\", choose(false, Box::new(|| div(1, 0)), Box::new(|| 7)));\n}\n";

    const DEMANDED_TRAPS: &str = "fn div(a: i64, b: i64) -> i64 {\n    a / b\n}\n\nfn main() {\n    println!(\"{}\", div(1, 0));\n}\n";

    /// The never-forced division never runs: both engines print `42`
    /// and `7` with no trap.
    #[test]
    fn ex_5_25_unused_thunk_never_runs() {
        let transcript = agreed(THUNKS);
        assert_eq!(transcript, "42\n7\n", "{transcript}");
    }

    /// Forcing the same division traps on both engines: laziness
    /// defers the failure, it does not remove it.
    #[test]
    fn ex_5_25_demanded_failure_still_traps() {
        let program = admitted(DEMANDED_TRAPS);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_some(), "{interpreted:?}");
        assert!(compiled.trap.is_some(), "{compiled:?}");
    }
}
