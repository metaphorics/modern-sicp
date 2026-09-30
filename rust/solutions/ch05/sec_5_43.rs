// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.43: bindings initialize
//! before use.
//!
//! Internal definitions become ordered local bindings: each `let`
//! initializes its cell before the next one reads it, so `a = 1` and
//! `b = 2` combine to `3`. Reading a name before its binding exists
//! never reaches evaluation — admission rejects the program the way
//! the scan rejects an unassigned variable, before any effect runs.

use sicp_runtime::host::{CheckedProgram, DiagKind};

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_43 {
    //! Exercise 5.43: ordered bindings provide their values; uses
    //! before any binding are rejected.

    use super::*;

    const ORDERED: &str = "fn sum() -> i64 {\n    let a = 1;\n    let b = 2;\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", sum());\n}\n";

    const TOO_EARLY: &str = "fn main() {\n    println!(\"{}\", x);\n    let x = 1;\n}\n";

    /// The ordered bindings combine to `3` on both engines.
    #[test]
    fn ex_5_43_ordered_bindings_provide_values() {
        let program = admitted(ORDERED);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "3\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }

    /// The use before any binding is rejected before effects run.
    #[test]
    fn ex_5_43_early_use_rejected() {
        let Err(diag) = sicp_runtime::host::admit(TOO_EARLY) else {
            panic!("the early use admitted");
        };
        assert_eq!(diag.kind, DiagKind::Type, "{diag:?}");
    }
}
