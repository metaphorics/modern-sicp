// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.33: operand order changes
//! which value stays live across the recursive call.
//!
//! The book's factorial multiplies the recursive result by `n`; the
//! alternative multiplies `n` by the recursive result. Both recurrences
//! answer `120` at n = 5 on both engines. The operand discipline
//! evaluates each call argument into a saved slot, so both streams
//! carry save traffic around the recursion; which operand the source
//! order leaves live decides its shape. The solution asserts the
//! shared facts and computes both save counts for the comparison
//! instead of pinning them.

use ch05::sec_5_5::{Instr, PerformOp};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn saves_of(program: &CheckedProgram) -> usize {
    ch05::sec_5_5::compile_program(program)
        .instrs
        .iter()
        .filter(|instr| matches!(instr, Instr::Save(_)))
        .count()
}

mod ex_5_33 {
    //! Exercise 5.33: both operand orders answer `120`; the compiled
    //! save traffic reflects which operand stays live.

    use super::*;

    const FACTORIAL: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        factorial(n - 1) * n\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    const FACTORIAL_ALT: &str = "fn factorial_alt(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial_alt(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial_alt(5));\n}\n";

    fn answers_120(source: &str) {
        let program = admitted(source);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "120\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }

    /// Both orders answer `120` through a direct recursive transfer,
    /// and both streams carry save traffic for the call argument.
    #[test]
    fn ex_5_33_operand_order_keeps_value() {
        answers_120(FACTORIAL);
        answers_120(FACTORIAL_ALT);
        let base = admitted(FACTORIAL);
        let alt = admitted(FACTORIAL_ALT);
        assert!(saves_of(&base) > 0, "the call argument is saved");
        assert!(saves_of(&alt) > 0, "the call argument is saved");
        for program in [&base, &alt] {
            let stream = ch05::sec_5_5::compile_program(program).instrs;
            assert!(
                stream
                    .iter()
                    .any(|instr| matches!(instr, Instr::Perform(PerformOp::CallFun, _))),
                "the recursion stays a direct transfer"
            );
        }
    }
}
