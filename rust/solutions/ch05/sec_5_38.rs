// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.38: arithmetic lowers to
//! inline operations, calls to transfers.
//!
//! The typed compiler has no open-coding switch because it needs
//! none: integer arithmetic is already an inline `Arith` operation
//! over the operand registers, while a helper call builds an argument
//! list and transfers through `CallFun`. The direct factorial keeps
//! its arithmetic inline; routing the same operations through helpers
//! adds the call machinery without changing any answer. Both run to
//! the same values on both engines.

use ch05::sec_5_5::{Instr, Operand, PerformOp};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn uses_inline_arith(program: &CheckedProgram) -> bool {
    ch05::sec_5_5::compile_program(program)
        .instrs
        .iter()
        .any(|instr| {
            matches!(instr, Instr::Perform(PerformOp::Arith(_), _))
                | matches!(instr, Instr::Assign(_, Operand::Op(PerformOp::Arith(_), _)))
        })
}

fn uses_call(program: &CheckedProgram) -> bool {
    ch05::sec_5_5::compile_program(program)
        .instrs
        .iter()
        .any(|instr| matches!(instr, Instr::Perform(PerformOp::CallFun, _)))
}

mod ex_5_38 {
    //! Exercise 5.38: arithmetic stays inline while helper calls pay
    //! the transfer; the answers never differ.

    use super::*;

    const DIRECT: &str = "fn main() {\n    println!(\"{}\", 6i64 * 20i64);\n    println!(\"{}\", 4i64 + 3i64 + 2i64 + 1i64);\n    println!(\"{}\", 10i64 - 3i64);\n}\n";

    const THROUGH_HELPERS: &str = "fn mul(a: i64, b: i64) -> i64 {\n    a * b\n}\n\nfn add(a: i64, b: i64) -> i64 {\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", mul(6, 20));\n    println!(\"{}\", add(add(4, 3), add(2, 1)));\n}\n";

    fn agreed(source: &str) -> String {
        let program = admitted(source);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
        interpreted.stdout
    }

    /// Inline arithmetic answers `120`, `10`, and `7` with no call
    /// transfer in the stream.
    #[test]
    fn ex_5_38_arithmetic_stays_inline() {
        let program = admitted(DIRECT);
        assert!(uses_inline_arith(&program), "arithmetic is inline");
        assert!(!uses_call(&program), "no transfer is needed");
        assert_eq!(agreed(DIRECT), "120\n10\n7\n");
    }

    /// The helper-routed versions answer the same values through real
    /// call transfers.
    #[test]
    fn ex_5_38_helpers_pay_the_transfer() {
        let program = admitted(THROUGH_HELPERS);
        assert!(uses_call(&program), "helpers transfer");
        assert_eq!(agreed(THROUGH_HELPERS), "120\n10\n");
    }
}
