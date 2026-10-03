// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.35: reading the compiler's
//! listing.
//!
//! Compiling `combine` answers what the book's Figure 5.18 asks the
//! reader to reconstruct: the entry label the stream starts from, the
//! register file it runs on, the primitives it lowers to `mc_*`
//! calls, and the save stack the argument discipline stages through.
//! The solution checks the listing's load-bearing features — entry,
//! unit boundary, store calls, stack arrays — instead of pinning
//! generated label spellings, and runs the source on both engines to
//! tie the listing to its answers.

use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_35 {
    //! Exercise 5.35: the emitted unit carries its entry, its store
    //! calls, and its stack discipline in the open.

    use super::*;

    const COMBINE: &str = "fn combine(a: i64, b: i64) -> i64 {\n    (a + b) * (a - b) + a * b\n}\n\nfn main() {\n    println!(\"{}\", combine(7, 3));\n}\n";

    /// The source answers `61` on both engines: `(7 + 3) * (7 - 3) +
    /// 7 * 3` is `40 + 21`.
    #[test]
    fn ex_5_35_listing_runs() {
        let program = admitted(COMBINE);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "61\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }

    /// The emitted unit starts at the program's entry, defines the
    /// unit boundary, allocates integers through the store, and stages
    /// arguments through the save stack.
    #[test]
    fn ex_5_35_listing_carries_its_machinery() {
        let program = admitted(COMBINE);
        let compiled = ch05::sec_5_5::compile_program(&program);
        let unit = ch05::sec_5_5::emit_c(&compiled);
        assert!(unit.contains(&compiled.entry), "the entry is defined");
        assert!(unit.contains("mc_main"), "the unit boundary exists");
        assert!(
            unit.contains("mc_alloc_int"),
            "integers come from the store"
        );
        assert!(unit.contains("save_stack"), "arguments stage through saves");
    }
}
