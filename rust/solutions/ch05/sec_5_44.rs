// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.44: shadowing is lexical, so
//! a rebound name stays an ordinary binding.
//!
//! A block that rebinds `x` evaluates its own binding where the outer
//! one stood, while free names still resolve to the functions they
//! named: the inner `double(x) + x` uses the inner `1` twice for `3`,
//! and the outer `double(x)` still doubles the outer `100` to `200`.
//! No special case distinguishes the two; the environment's
//! innermost-first order decides both.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_44 {
    //! Exercise 5.44: rebound names resolve inward; free names still
    //! reach their functions.

    use super::*;

    const SHADOWED: &str = "fn double(x: i64) -> i64 {\n    x * 2\n}\n\nfn main() {\n    let x = 100;\n    let inner = {\n        let x = 1;\n        double(x) + x\n    };\n    println!(\"{}\", inner);\n    println!(\"{}\", double(x));\n}\n";

    /// The inner block answers `3` from its own `x` and the outer
    /// call answers `200` from the untouched `100`.
    #[test]
    fn ex_5_44_shadowed_name_stays_ordinary() {
        let program = admitted(SHADOWED);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "3\n200\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
