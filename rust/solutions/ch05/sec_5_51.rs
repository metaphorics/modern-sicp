// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.51: the explicit-control
//! evaluator of 5.4 translated into C and built by the system C
//! compiler.
//!
//! The translation is the controller itself: the registers are
//! fields of one `eceval` function's statics, every `ev-` entry point
//! of 5.4.1 to 5.4.4 is a C label, `(assign continue (label l))`
//! stores a label address, `(goto (reg continue))` is `goto *R_continue`
//! (the GNU computed-goto extension the system compiler accepts), and
//! the stack is an array with the book's push/pop discipline and
//! monitored counters. The object world of pairs, symbols,
//! environments, vectors, tagged option/result pairs, and the
//! primitive table is the run-time support the exercise requires.
//!
//! The object language is the admitted Rust slice: `fn` items with
//! integer arithmetic, comparisons, `if`/`else`, and calls, plus one
//! `main` whose `println!` lines are the interactions the driver loop
//! evaluates and prints. The harness admits every input through the
//! edition checker before the C binary runs it, so the C sessions
//! below execute valid guest programs.

use std::process::Command;

mod ex_5_51 {
    //! Exercise 5.51: the C translation answers the book's factorial
    //! session: `ok`, then `120`. A second session runs tree-recursive
    //! Fibonacci to `55`.

    use super::*;

    const ECEVAL_C: &str = include_str!("eceval_5_51.c");

    const FACTORIAL: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    const FIBONACCI: &str = "fn fib(n: i64) -> i64 {\n    if n < 2 {\n        n\n    } else {\n        fib(n - 1) + fib(n - 2)\n    }\n}\n\nfn main() {\n    println!(\"{}\", fib(10));\n}\n";

    /// Admits `source` through the edition checker, so the C binary
    /// only ever runs valid guest programs.
    fn admitted(source: &str) -> sicp_runtime::host::CheckedProgram {
        match sicp_runtime::host::admit(source) {
            Ok(program) => program,
            Err(diag) => panic!("the 5.51 object program was rejected: {}", diag.message),
        }
    }

    fn compile_and_run(program: &str, expected_value: &str) -> String {
        let checked = admitted(program);
        let expected = ch05::sec_5_4::Eceval::run(&checked);
        assert!(expected.trap.is_none(), "{expected:?}");
        assert!(expected.stdout.contains(expected_value), "{expected:?}");
        let dir = std::env::temp_dir().join(format!("sicp_rust_5_51_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("scratch dir: {error}"));
        let source = dir.join("eceval.c");
        let binary = dir.join("eceval");
        let input = dir.join("program.rs");
        std::fs::write(&source, ECEVAL_C).unwrap_or_else(|error| panic!("write c: {error}"));
        std::fs::write(&input, program).unwrap_or_else(|error| panic!("write program: {error}"));
        let build = Command::new("cc")
            .arg("-O1")
            .arg("-o")
            .arg(&binary)
            .arg(&source)
            .output()
            .unwrap_or_else(|error| panic!("cc spawn: {error}"));
        if !build.status.success() {
            let text = String::from_utf8_lossy(&build.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            panic!("the C translation failed to build: {text}");
        }
        let run = Command::new(&binary)
            .arg(&input)
            .output()
            .unwrap_or_else(|error| panic!("run spawn: {error}"));
        if !run.status.success() {
            let text = String::from_utf8_lossy(&run.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            panic!(
                "the C translation failed with status {}: {text}",
                run.status
            );
        }
        let text = String::from_utf8_lossy(&run.stdout).to_string();
        let _ = std::fs::remove_dir_all(&dir);
        text
    }

    /// Answers both C session transcripts.
    ///
    /// # Panics
    ///
    /// Panics when the C build fails, a session disagrees, or an
    /// answer is missing.
    pub fn ex_5_51() -> Vec<String> {
        let factorial = compile_and_run(FACTORIAL, "120");
        assert!(factorial.contains("ok"), "{factorial}");
        assert!(factorial.contains("120"), "{factorial}");
        let fibonacci = compile_and_run(FIBONACCI, "55");
        assert!(fibonacci.contains("ok"), "{fibonacci}");
        assert!(fibonacci.contains("55"), "{fibonacci}");
        vec![factorial, fibonacci]
    }

    #[test]
    fn ex_5_51_check() {
        let lines = ex_5_51();
        assert!(lines[0].contains("ok"));
        assert!(lines[0].contains("120"));
        assert!(lines[1].contains("55"));
    }
}
