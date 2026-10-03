// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.36: call operands evaluate
//! left to right.
//!
//! The contract fixes what the book leaves to the implementation:
//! calls and nested operands evaluate left to right, as in Rust. Two
//! operand functions announce themselves as they run, so the
//! transcript records the order while the combined value proves the
//! argument list still arrives in source order. Both engines must
//! print the same three lines in the same order.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_36 {
    //! Exercise 5.36: the recording order is left to right and the
    //! built combination keeps source order.

    use super::*;

    const ORDERED: &str = "fn first() -> i64 {\n    println!(\"first\");\n    1\n}\n\nfn second() -> i64 {\n    println!(\"second\");\n    2\n}\n\nfn combine(a: i64, b: i64) -> i64 {\n    a * 10 + b\n}\n\nfn main() {\n    println!(\"{}\", combine(first(), second()));\n}\n";

    fn positions(transcript: &str) -> (usize, usize, usize) {
        let first = transcript
            .find("first")
            .unwrap_or_else(|| panic!("order lost: {transcript}"));
        let second = transcript
            .find("second")
            .unwrap_or_else(|| panic!("order lost: {transcript}"));
        let value = transcript
            .find("12")
            .unwrap_or_else(|| panic!("order lost: {transcript}"));
        (first, second, value)
    }

    /// Both engines print `first`, then `second`, then `12`: the
    /// operands ran left to right and combined in source order.
    #[test]
    fn ex_5_36_operands_run_left_to_right() {
        let program = admitted(ORDERED);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
        for transcript in [interpreted.stdout, compiled.stdout] {
            let (first, second, value) = positions(&transcript);
            assert!(first < second, "{transcript}");
            assert!(second < value, "{transcript}");
        }
    }
}
