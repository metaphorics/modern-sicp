// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.48: staged compilation at
//! run time.
//!
//! A host program that builds guest source, admits it, and runs it is
//! the edition's `compile-and-run`: the compiler is available during
//! execution, and newly admitted code runs on both engines like any
//! other program. Two staged variants prove the staging is real — the
//! generated constant decides the answer, so `6 * 20` prints `120`
//! while `7 * 20` prints `140`. Nothing is quoted, canned, or
//! precompiled: each run admits fresh source text.

fn staged(scale: i64, multiplier: i64) -> String {
    format!(
        "fn scaled() -> i64 {{\n    {scale} * {multiplier}\n}}\n\nfn main() {{\n    println!(\"{{}}\", scaled());\n}}\n"
    )
}

fn run_staged(source: &str) -> String {
    let program = match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("staged admit: {}", diag.message),
    };
    let interpreted = ch05::sec_5_4::Eceval::run(&program);
    let compiled = ch05::sec_5_5::compiled_run(&program);
    assert!(interpreted.trap.is_none(), "{interpreted:?}");
    assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    interpreted.stdout
}

mod ex_5_48 {
    //! Exercise 5.48: freshly generated source admits and runs, and
    //! its constants decide its answers.

    use super::*;

    /// The staged `6 * 20` answers `120`; restaging with `7` answers
    /// `140`, so the generated text really ran.
    #[test]
    fn ex_5_48_staged_source_runs() {
        assert_eq!(run_staged(&staged(6, 20)), "120\n");
        assert_eq!(run_staged(&staged(7, 20)), "140\n");
    }
}
