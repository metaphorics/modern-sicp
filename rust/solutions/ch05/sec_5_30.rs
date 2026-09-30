// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.30: failures trap with their
//! printed prefix intact, and invalid programs never run.
//!
//! Both parts of the exercise survive as engine behavior. A trap
//! during evaluation — division by zero through an ordinary
//! parameter, or an index past the vector's end — stops the run with
//! the transcript kept up to the failure on both engines, which is
//! the book's condition-signalling discipline without a condition
//! datatype. An unbound variable never reaches evaluation at all:
//! admission rejects the whole program before any effect, so there is
//! no transcript to keep.

use sicp_runtime::host::{CheckedProgram, DiagKind};

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_30 {
    //! Exercise 5.30: traps keep their prefix on both engines, and
    //! rejection precedes every effect.

    use super::*;

    const DIV_ZERO: &str = "fn div(a: i64, b: i64) -> i64 {\n    a / b\n}\n\nfn main() {\n    println!(\"before\");\n    println!(\"{}\", div(1, 0));\n}\n";

    const INDEX_PAST_END: &str = "fn main() {\n    let pair: (i64, i64) = (10, 20);\n    println!(\"before\");\n    println!(\"{}\", pair.1);\n    let items: Vec<i64> = Vec::new();\n    println!(\"{}\", items[0]);\n}\n";

    const UNBOUND: &str = "fn main() {\n    println!(\"{}\", no_such_variable);\n}\n";

    const CLEAN: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    fn trapped(source: &str) -> String {
        let program = admitted(source);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_some(), "{interpreted:?}");
        assert!(compiled.trap.is_some(), "{compiled:?}");
        assert_eq!(interpreted.stdout, compiled.stdout, "prefixes agree");
        interpreted.stdout
    }

    /// Division by zero traps on both engines with `before` already
    /// printed: the failure keeps its prefix.
    #[test]
    fn ex_5_30_division_traps_with_prefix() {
        let stdout = trapped(DIV_ZERO);
        assert!(stdout.contains("before"), "{stdout}");
    }

    /// An index past the vector's end traps the same way on both
    /// engines, after the earlier lines print.
    #[test]
    fn ex_5_30_index_traps_with_prefix() {
        let stdout = trapped(INDEX_PAST_END);
        assert!(stdout.contains("before"), "{stdout}");
        assert!(stdout.contains("20"), "{stdout}");
    }

    /// An unbound variable is rejected before any effect: admission
    /// fails, so no run exists.
    #[test]
    fn ex_5_30_unbound_rejected_before_effects() {
        let Err(diag) = sicp_runtime::host::admit(UNBOUND) else {
            panic!("the unbound name admitted");
        };
        assert_eq!(diag.kind, DiagKind::Type, "{diag:?}");
    }

    /// The checks change nothing on the correct path: the clean
    /// factorial still answers `120` on both engines.
    #[test]
    fn ex_5_30_clean_path_unchanged() {
        let program = admitted(CLEAN);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "120\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
