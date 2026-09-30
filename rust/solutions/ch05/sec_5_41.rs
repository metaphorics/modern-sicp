// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.41: lexical resolution finds
//! the nearest binding.
//!
//! Searching from the innermost frame outward, a name resolves to the
//! first frame that binds it; names bound nowhere resolve to the
//! global function. Three shadowing depths make the order observable:
//! the innermost `x` wins in its block, the middle `x` wins where the
//! inner block ends, and the outer `x` survives untouched. The
//! arithmetic combines the three answers into `241` on both engines.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_41 {
    //! Exercise 5.41: each use of `x` finds its nearest enclosing
    //! binding.

    use super::*;

    const SHADOWED: &str = "fn main() {\n    let x: i64 = 1;\n    let a = {\n        let x: i64 = 2;\n        x\n    };\n    let b = {\n        let y = 3;\n        x + y\n    };\n    println!(\"{}\", a * 100 + b * 10 + x);\n}\n";

    /// The inner block sees `2`, the sibling block sees the outer `1`
    /// plus its `3`, and the outer `x` is still `1`: `241`.
    #[test]
    fn ex_5_41_nearest_binding_wins() {
        let program = admitted(SHADOWED);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "241\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
