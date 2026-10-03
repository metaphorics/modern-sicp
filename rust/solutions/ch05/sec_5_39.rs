// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.39: captured bindings mutate
//! in place.
//!
//! A closure owns the bindings it captures: calling it with `FnMut`
//! runs the body against the same cells, so an accumulator's update
//! persists across calls the way the book's addressed assignment
//! persists across lookups. Starting the accumulator at 10 and adding
//! 11 answers 21, then adding 99 answers 120. Both engines agree,
//! which ties the capture discipline to observable state.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_39 {
    //! Exercise 5.39: a captured cell updated by one call is read back
    //! by the next.

    use super::*;

    const ACCUMULATOR: &str = "fn make_accumulator(mut total: i64) -> Box<dyn FnMut(i64) -> i64 + 'static> {\n    Box::new(move |n: i64| {\n        total += n;\n        total\n    })\n}\n\nfn main() {\n    let mut cell = make_accumulator(10);\n    println!(\"{}\", cell(11));\n    println!(\"{}\", cell(99));\n}\n";

    /// The first call moves the cell from 10 to 21 and the second
    /// from 21 to 120, identically on both engines.
    #[test]
    fn ex_5_39_captured_cell_mutates() {
        let program = admitted(ACCUMULATOR);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "21\n120\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
