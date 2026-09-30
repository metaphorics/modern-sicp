// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.27: the recursive factorial
//! grows its stack linearly.
//!
//! Each non-base level of the Figure 5.11 machine saves `continue`
//! and `n`, so n = 1..=7 measures pushes and depth both at `2n - 2`,
//! the slope 2 counting the pair each level parks and the base level
//! discounting itself. The same recurrence as a guest program answers
//! `120` at n = 5 on both engines, and the compiled run's counted
//! stack grows the same linear way.

use ch05::sec_5_1::factorial_recursive;
use ch05::sec_5_2::{Machine, assemble};

mod ex_5_27 {
    //! Exercise 5.27: recursion parks two words per level, so stack
    //! use is `2n - 2`.

    use super::*;

    const RECURSIVE: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    fn machine_at(n: i64) -> (i64, u64, u64) {
        let mut machine = Machine::new(assemble(&factorial_recursive()).expect("assembles"));
        machine.set_register("n", n).expect("register n");
        machine.run().expect("runs");
        let stats = machine.stack_statistics();
        let depth = u64::try_from(stats.max_depth).expect("small depth");
        (
            machine.get_register("val").expect("val"),
            stats.pushes,
            depth,
        )
    }

    fn admitted(source: &str) -> sicp_runtime::host::CheckedProgram {
        match sicp_runtime::host::admit(source) {
            Ok(program) => program,
            Err(diag) => panic!("admitted: {}", diag.message),
        }
    }

    /// Seven runs of the recursive machine: the answers are the
    /// factorials and both stack measures fit `2n - 2`.
    #[test]
    fn ex_5_27_recursive_stack_is_linear() {
        let mut expected = 1i64;
        for n in 1i64..=7 {
            expected *= n;
            let (answer, pushes, depth) = machine_at(n);
            let formula = u64::try_from(2 * (n - 1)).expect("small n");
            assert_eq!(answer, expected, "factorial({n})");
            assert_eq!(pushes, formula, "factorial({n}) pushes");
            assert_eq!(depth, formula, "factorial({n}) depth");
        }
    }

    /// The guest recurrence answers `120` on both engines, and the
    /// compiled run's counted depth is positive: recursion really
    /// parks stack, unlike the 5.26 loop.
    #[test]
    fn ex_5_27_guest_recursion_matches() {
        let program = admitted(RECURSIVE);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        let (counted, stats) = ch05::sec_5_5::compiled_run_counted(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "120\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
        assert_eq!(counted.stdout, "120\n");
        assert!(stats.max_depth > 0, "recursion parks stack");
    }
}
