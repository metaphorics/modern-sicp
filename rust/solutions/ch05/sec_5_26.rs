// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.26: the iterative factorial
//! holds constant stack space.
//!
//! The loop machine of section 5.1 never touches the stack: every
//! iteration reuses the same registers, so pushes and maximum depth
//! stay zero from n = 1 to 6 while the answers run through the
//! factorials. The same loop written as a guest program answers
//! identically on the explicit-control evaluator and the compiler,
//! which is the interpreted/compiled agreement the later comparisons
//! build on.

use ch05::sec_5_1::factorial_iterative;
use ch05::sec_5_2::{Machine, assemble};

mod ex_5_26 {
    //! Exercise 5.26: iteration reuses its registers, so stack use is
    //! independent of n.

    use super::*;

    const LOOP_FACTORIAL: &str = "fn factorial(n: i64) -> i64 {\n    let mut product = 1;\n    let mut counter = 1;\n    while counter <= n {\n        product = product * counter;\n        counter += 1;\n    }\n    product\n}\n\nfn main() {\n    println!(\"{}\", factorial(1));\n    println!(\"{}\", factorial(2));\n    println!(\"{}\", factorial(3));\n    println!(\"{}\", factorial(4));\n    println!(\"{}\", factorial(5));\n    println!(\"{}\", factorial(6));\n}\n";

    fn machine_at(n: i64) -> (i64, u64, usize) {
        let mut machine = Machine::new(assemble(&factorial_iterative()).expect("assembles"));
        machine.set_register("n", n).expect("register n");
        machine.run().expect("runs");
        let stats = machine.stack_statistics();
        (
            machine.get_register("product").expect("product"),
            stats.pushes,
            stats.max_depth,
        )
    }

    /// Six runs of the loop machine: the answers are the factorials
    /// and no run pushes anything at any depth.
    #[test]
    fn ex_5_26_iterative_stack_is_constant() {
        let mut expected = 1i64;
        for n in 1i64..=6 {
            expected *= n;
            let (answer, pushes, depth) = machine_at(n);
            assert_eq!(answer, expected, "factorial({n})");
            assert_eq!(pushes, 0, "factorial({n}) pushes");
            assert_eq!(depth, 0, "factorial({n}) depth");
        }
    }

    /// The guest loop answers the same factorials on both engines.
    #[test]
    fn ex_5_26_guest_loop_matches() {
        let program = match sicp_runtime::host::admit(LOOP_FACTORIAL) {
            Ok(program) => program,
            Err(diag) => panic!("admitted: {}", diag.message),
        };
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "1\n2\n6\n24\n120\n720\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
