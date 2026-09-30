// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.50: the guest evaluator
//! runs on the teaching compiled machine.
//!
//! The corpus guest evaluator is an ordinary admitted program: it
//! builds expression data, evaluates it with its own environments,
//! and prints `120`, `42`, `42`, `1`, `2`. Running it through the
//! explicit-control evaluator and the compiler checks the same
//! transcript on both, and counted VM steps compare the second
//! interpretation level with a direct compiled factorial.
//! This run takes 7405 steps versus 146 for level 0 (about 50.7x).
//! The exercise requires only that interpretation costs more.
//! Wall-clock time is reported in the prose, never pinned.

use sicp_runtime::host::CheckedProgram;

const SELF_INTERPRETER: &str =
    include_str!("../../../spec/host-subsets/rust/programs/selfinterp.rs");

const LEVEL_ZERO: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

fn counted(source: &str) -> (String, u64) {
    let program = admitted(source);
    let (outcome, stats) = ch05::sec_5_5::compiled_run_counted(&program);
    assert!(outcome.trap.is_none(), "trapped: {outcome:?}");
    (outcome.stdout, stats.steps)
}

mod ex_5_50 {
    //! Exercise 5.50: the guest evaluator's session answers on both
    //! engines, and its stack price dwarfs the direct run's.

    use super::*;

    /// The guest evaluator prints `120`, `42`, `42`, `1`, `2` on both
    /// engines: the factorial, the captured value, the adder, and the
    /// two counter calls.
    #[test]
    fn ex_5_50_guest_evaluator_runs_compiled() {
        let program = admitted(SELF_INTERPRETER);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "120\n42\n42\n1\n2\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }

    /// The guest evaluator uses more VM steps than the direct compiled
    /// factorial, as the exercise predicts.
    #[test]
    // These fixed programs keep step counts within f64's exact integer range.
    #[allow(clippy::cast_precision_loss)]
    fn ex_5_50_interpretation_has_a_price() {
        let (level_zero_out, level_zero) = counted(LEVEL_ZERO);
        let (level_two_out, level_two) = counted(SELF_INTERPRETER);
        assert_eq!(level_zero_out, "120\n");
        assert_eq!(level_two_out, "120\n42\n42\n1\n2\n");
        let ratio = level_two as f64 / level_zero as f64;
        println!("level 2: {level_two} steps vs level 0: {level_zero} steps ({ratio:.1}x)");
        assert!(
            level_two > level_zero,
            "level 2 must cost more than direct execution: {level_two} vs {level_zero}"
        );
    }
}
