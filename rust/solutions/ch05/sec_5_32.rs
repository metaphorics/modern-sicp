// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.32: the compiler's fast path
//! for statically known operators.
//!
//! A call whose callee the checker resolves to a function id compiles
//! to a direct `CallFun` transfer: no operator value is ever built,
//! tested, or dispatched. A call through a variable — a function
//! pointer, a boxed closure — compiles to the indirect sequence that
//! evaluates the callee, parks it across the arguments, and transfers
//! through `CallValue`. Both answer identically on both engines; the
//! emitted streams show which path each took.

use ch05::sec_5_5::{Instr, PerformOp};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn has_direct(program: &CheckedProgram) -> bool {
    ch05::sec_5_5::compile_program(program)
        .instrs
        .iter()
        .any(|instr| matches!(instr, Instr::Perform(PerformOp::CallFun, _)))
}

fn has_indirect(program: &CheckedProgram) -> bool {
    ch05::sec_5_5::compile_program(program)
        .instrs
        .iter()
        .any(|instr| matches!(instr, Instr::Perform(PerformOp::CallValue, _)))
}

mod ex_5_32 {
    //! Exercise 5.32: known operators compile to direct transfers;
    //! computed operators go through the indirect path.

    use super::*;

    const DIRECT: &str = "fn square(n: i64) -> i64 {\n    n * n\n}\n\nfn main() {\n    println!(\"{}\", square(6));\n}\n";

    const INDIRECT: &str = "fn double(x: i64) -> i64 {\n    x * 2\n}\n\nfn apply(f: fn(i64) -> i64, v: i64) -> i64 {\n    f(v)\n}\n\nfn main() {\n    println!(\"{}\", apply(double, 21));\n}\n";

    fn agreed(source: &str) -> String {
        let program = admitted(source);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
        interpreted.stdout
    }

    /// The named call answers `36` through the direct transfer, with
    /// no indirect dispatch in its stream.
    #[test]
    fn ex_5_32_direct_call_fast_path() {
        let program = admitted(DIRECT);
        assert!(has_direct(&program), "a direct transfer is emitted");
        assert!(!has_indirect(&program), "no indirect dispatch is needed");
        assert_eq!(agreed(DIRECT), "36\n");
    }

    /// The pointer call answers `42` through the indirect sequence,
    /// which parks the callee across its argument.
    #[test]
    fn ex_5_32_indirect_call_general_path() {
        let program = admitted(INDIRECT);
        assert!(has_indirect(&program), "an indirect transfer is emitted");
        assert_eq!(agreed(INDIRECT), "42\n");
    }
}
