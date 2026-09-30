// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.40: nested scopes resolve
//! names from the inside out.
//!
//! Each closure and block extends the environment with its own frame,
//! so a name inside the innermost scope sees the nearest binding
//! first and the outer frames behind it. The solution nests an adder
//! factory inside its use site and shadows one name along the way:
//! the factory's parameter wins inside the closure while the outer
//! binding stays visible outside, and both engines print the same
//! resolved values.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_40 {
    //! Exercise 5.40: inner frames shadow outer ones; outer bindings
    //! stay visible where nothing shadows them.

    use super::*;

    const NESTED: &str = "fn make_adder(x: i64) -> Box<dyn Fn(i64) -> i64 + 'static> {\n    Box::new(move |y: i64| x + y)\n}\n\nfn main() {\n    let x: i64 = 100;\n    let add_three = make_adder(3);\n    println!(\"{}\", add_three(4));\n    println!(\"{}\", x);\n}\n";

    /// The closure adds its captured `3`, not the outer `100`, while
    /// the outer `x` still reads `100` outside: `7`, then `100`.
    #[test]
    fn ex_5_40_inner_frame_wins_inside() {
        let program = admitted(NESTED);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "7\n100\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
