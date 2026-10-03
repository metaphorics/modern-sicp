// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.52: the typed compiler's C
//! backend, run for real.
//!
//! The admitted guest program travels the full typed pipeline:
//! [`sicp_runtime::host::admit`] checks it, `compile_program` lowers
//! the checked program to the flat instruction stream, and `emit_c`
//! renders that stream as a C translation unit over the tagged-word
//! `mc_*` store. The solution links the unit with
//! `metacircular_backend_5_52.c`, builds it with the system C
//! compiler, runs it, and checks the transcript against the teaching
//! engines running the same source. No output is canned and no host
//! evaluation is substituted: the C binary computes every answer.
//!
//! The guest stays inside the fragment the landed emitter lowers to
//! declared C: main-only straight-line code with no locals — nested
//! integer arithmetic, tuple construction and projection, `if`/`else`
//! control, bare `Some` and `vec!` constructions, and `println!` of
//! integers. Locals lower to `slot{N}` registers the unit never
//! declares, so `let`, parameters, and `while` wait on the emitter
//! fix the handoff report names; calls, closures, pattern binding,
//! methods, and string formatting wait with them.

mod ex_5_52 {
    //! Exercise 5.52: compile one guest program to C, build it, and
    //! run it next to the teaching engines.

    const RUNTIME_C: &str = include_str!("metacircular_backend_5_52.c");

    /// Straight-line integer code, tuple projection, and constructor
    /// statements with no locals: both constructions execute between
    /// printed lines, so the final line proves the middle ran without
    /// trapping.
    const GUEST: &str = "fn main() {\n    println!(\"{}\", (7i64 + 3i64) * (7i64 - 3i64) + 7i64 * 3i64);\n    println!(\"{}\", (3i64, 4i64).0 * 10i64 + (3i64, 4i64).1);\n    Some(4100i64);\n    vec![1i64, 2i64, 3i64];\n    println!(\"{}\", if 2i64 * 21i64 == 42i64 { 100i64 } else { 200i64 });\n}\n";

    fn admitted(source: &str) -> sicp_runtime::host::CheckedProgram {
        match sicp_runtime::host::admit(source) {
            Ok(program) => program,
            Err(diag) => panic!("the 5.52 guest was rejected: {}", diag.message),
        }
    }

    /// The C file: the store, then the emitted unit. Also answers the
    /// teaching compiler's transcript for the cross-check.
    fn compile_to_c() -> (String, String) {
        let checked = admitted(GUEST);
        let program = ch05::sec_5_5::compile_program(&checked);
        let unit = ch05::sec_5_5::emit_c(&program);
        assert!(unit.contains(&program.entry), "the unit defines its entry");
        assert!(unit.contains("mc_main"), "the unit defines mc_main");
        let outcome = ch05::sec_5_5::compiled_run(&checked);
        assert!(outcome.trap.is_none(), "{outcome:?}");
        (format!("{RUNTIME_C}\n{unit}"), outcome.stdout)
    }

    /// Builds the C file with the system compiler, runs it, and
    /// answers its combined output.
    fn build_and_run(c_source: &str) -> String {
        let dir = std::env::temp_dir().join(format!("sicp_rust_5_52_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap_or_else(|error| panic!("scratch dir: {error}"));
        let source = dir.join("compiled.c");
        let binary = dir.join("compiled");
        std::fs::write(&source, c_source).unwrap_or_else(|error| panic!("write c: {error}"));
        let build = std::process::Command::new("cc")
            .arg("-O1")
            .arg("-o")
            .arg(&binary)
            .arg(&source)
            .output()
            .unwrap_or_else(|error| panic!("cc spawn: {error}"));
        if !build.status.success() {
            let text = String::from_utf8_lossy(&build.stderr).to_string();
            let _ = std::fs::remove_dir_all(&dir);
            panic!("the C backend failed to build: {text}");
        }
        let run = std::process::Command::new(&binary)
            .output()
            .unwrap_or_else(|error| panic!("run spawn: {error}"));
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        let _ = std::fs::remove_dir_all(&dir);
        assert!(run.status.success(), "the C program failed: {text}");
        text
    }

    /// Compiles the guest to C, builds it, and runs it: the C binary
    /// prints `61`, `34`, and `100`, the same three tokens the
    /// teaching compiler prints for the same source.
    ///
    /// # Panics
    ///
    /// Panics when the C build fails or the binary disagrees with
    /// the teaching transcript.
    pub fn ex_5_52() -> Vec<String> {
        let (c_source, expected) = compile_to_c();
        let output = build_and_run(&c_source);
        let tokens: Vec<&str> = output.split_whitespace().collect();
        let wanted: Vec<&str> = expected.split_whitespace().collect();
        assert_eq!(
            tokens, wanted,
            "the C binary replays the teaching transcript"
        );
        vec![output]
    }

    #[test]
    fn ex_5_52_check() {
        let lines = ex_5_52();
        assert!(lines[0].contains("61"));
        assert!(lines[0].contains("100"));
    }
}
