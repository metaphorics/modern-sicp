// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.42: captures compose across
//! nesting levels.
//!
//! The book's nested-lambda example threads three bindings through
//! three levels; the host version threads two counters through a
//! factory and its uses. Each counter owns an independent frame, so
//! advancing one leaves the other at its own count: after two calls
//! the first answers `3` on its third call while the second answers
//! `1` on its first. Both engines agree, which is lexical addressing
//! made observable — every closure reads its own frame.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_42 {
    //! Exercise 5.42: nested closures keep independent frames across
    //! levels.

    use super::*;

    const FRAMES: &str = "fn make_counter() -> Box<dyn FnMut() -> i64 + 'static> {\n    let mut value = 0;\n    Box::new(move || {\n        value += 1;\n        value\n    })\n}\n\nfn main() {\n    let mut first = make_counter();\n    let mut second = make_counter();\n    first();\n    first();\n    println!(\"{}\", first());\n    println!(\"{}\", second());\n}\n";

    /// The first counter's third call answers `3` while the second
    /// counter's first call answers `1`: separate frames.
    #[test]
    fn ex_5_42_frames_stay_independent() {
        let program = admitted(FRAMES);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "3\n1\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
