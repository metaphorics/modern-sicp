// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.24: `match` as a basic
//! control form.
//!
//! The closed enum with one exhaustive `match` is the host slice's
//! answer to the book's `ev-cond` loop over clauses: each arm selects
//! its actions by the scrutinee's shape, the payload arm returns the
//! carried value the way a bodyless clause returns its predicate's
//! value, and the checker rejects a missing arm before any effect
//! runs, which is the `else`-less cond answering `#f` made static.

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

mod ex_5_24 {
    //! Exercise 5.24: the exhaustive match selects arms, returns
    //! payloads, and refuses missing arms at admission.

    use super::*;

    const SIGNALS: &str = "enum Signal {\n    Stop,\n    Slow(i64),\n    Go,\n}\n\nfn code(signal: Signal) -> i64 {\n    match signal {\n        Signal::Stop => 0,\n        Signal::Slow(limit) => limit,\n        Signal::Go => 100,\n    }\n}\n\nfn main() {\n    println!(\"{}\", code(Signal::Stop));\n    println!(\"{}\", code(Signal::Slow(30)));\n    println!(\"{}\", code(Signal::Go));\n}\n";

    const MISSING_ARM: &str = "enum Signal {\n    Stop,\n    Slow(i64),\n    Go,\n}\n\nfn code(signal: Signal) -> i64 {\n    match signal {\n        Signal::Stop => 0,\n        Signal::Go => 100,\n    }\n}\n\nfn main() {\n    println!(\"{}\", code(Signal::Stop));\n}\n";

    /// Every arm answers on both engines, and the payload arm returns
    /// its carried value: `0`, `30`, `100`.
    #[test]
    fn ex_5_24_match_selects_arms() {
        let transcript = agreed(SIGNALS);
        assert_eq!(transcript, "0\n30\n100\n", "{transcript}");
    }

    /// A match missing an arm is rejected before any effect runs: no
    /// transcript exists to observe.
    #[test]
    fn ex_5_24_missing_arm_rejected() {
        let Err(diag) = sicp_runtime::host::admit(MISSING_ARM) else {
            panic!("the missing arm admitted");
        };
        assert_eq!(diag.kind, sicp_runtime::host::DiagKind::Type, "{diag:?}");
    }
}
