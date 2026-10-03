// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.47: compiled callers reach
//! computed callees through the indirect transfer.
//!
//! A function pointer parameter is the host slice's interpreted
//! procedure arriving inside compiled code: the caller cannot name
//! the entry at compile time, so it evaluates the callee, parks the
//! value across the argument evaluation, and transfers through
//! `CallValue`. Passing the doubler and 21 answers `42` on both
//! engines, and the stream carries the indirect transfer the direct
//! calls of 5.32 never needed.

use ch05::sec_5_5::{Instr, PerformOp};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_47 {
    //! Exercise 5.47: a computed callee crosses the transfer with its
    //! value parked, and the call answers.

    use super::*;

    const CROSS_CALL: &str = "fn doubler(x: i64) -> i64 {\n    x * 2\n}\n\nfn apply(f: fn(i64) -> i64, v: i64) -> i64 {\n    f(v)\n}\n\nfn main() {\n    println!(\"{}\", apply(doubler, 21));\n}\n";

    /// The pointer call answers `42` on both engines through the
    /// indirect transfer, with a parked callee in its stream.
    #[test]
    fn ex_5_47_computed_callee_answers() {
        let program = admitted(CROSS_CALL);
        let stream = ch05::sec_5_5::compile_program(&program).instrs;
        assert!(
            stream
                .iter()
                .any(|instr| matches!(instr, Instr::Perform(PerformOp::CallValue, _))),
            "the computed callee transfers indirectly"
        );
        assert!(
            stream.iter().any(|instr| matches!(instr, Instr::Save(_))),
            "the callee survives its arguments"
        );
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "42\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
