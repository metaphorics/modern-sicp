// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.31: the `preserving`
//! discipline saves only live registers.
//!
//! The compiler threads every sequence with its liveness: `needs`
//! names what the sequence reads, `modifies` what it overwrites, and
//! `preserving` parks exactly their intersection around a nested
//! sequence. Registers the inner sequence never touches need no
//! stack traffic however deep the nesting goes. The solution proves
//! the rule on synthetic sequences and shows it at work in a compiled
//! indirect call, whose callee value the argument evaluation must not
//! clobber.

use ch05::sec_5_5::{Instr, PerformOp, Seq};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_31 {
    //! Exercise 5.31: `preserving` saves the intersection of needed
    //! and modified registers, and nothing else.

    use super::*;

    const INDIRECT: &str = "fn g(x: i64) -> i64 {\n    x * 2\n}\n\nfn apply(f: fn(i64) -> i64, v: i64) -> i64 {\n    f(v)\n}\n\nfn main() {\n    println!(\"{}\", apply(g, 21));\n}\n";

    fn saves_of(sequence: &Seq) -> Vec<String> {
        sequence
            .stmts
            .iter()
            .filter_map(|instr| match instr {
                Instr::Save(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    /// A register the inner sequence never modifies is never saved,
    /// however live it stays across the boundary.
    #[test]
    fn ex_5_31_untouched_register_needs_no_save() {
        let outer = Seq::new(&["env"], &[], Vec::new());
        let inner = Seq::new(&[], &["val"], Vec::new());
        let kept = outer.preserving(&["env", "val"], inner);
        assert!(saves_of(&kept).is_empty(), "{kept:?}");
    }

    /// A register the outer sequence needs and the inner sequence
    /// modifies is saved and restored around the inner sequence.
    #[test]
    fn ex_5_31_live_register_is_preserved() {
        let outer = Seq::new(&["val"], &[], Vec::new());
        let inner = Seq::new(&[], &["val"], Vec::new());
        let kept = outer.preserving(&["val"], inner);
        assert_eq!(saves_of(&kept), vec!["val".to_owned()], "{kept:?}");
        assert!(
            kept.stmts
                .iter()
                .any(|instr| matches!(instr, Instr::Restore(name) if name == "val")),
            "{kept:?}"
        );
    }

    /// The compiled indirect call parks its callee across the
    /// argument evaluation and answers `42` on both engines.
    #[test]
    fn ex_5_31_indirect_call_preserves_callee() {
        let program = admitted(INDIRECT);
        let compiled = ch05::sec_5_5::compile_program(&program);
        let saves = compiled
            .instrs
            .iter()
            .filter(|instr| matches!(instr, Instr::Save(_)))
            .count();
        let restores = compiled
            .instrs
            .iter()
            .filter(|instr| matches!(instr, Instr::Restore(_)))
            .count();
        assert!(saves > 0, "the arguments stage through saves");
        assert!(restores > 0, "the callee is restored");
        assert!(
            compiled
                .instrs
                .iter()
                .any(|instr| matches!(instr, Instr::Perform(PerformOp::MakeVec, _))),
            "the staged arguments collect into argl"
        );
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let outcome = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "42\n");
        assert_eq!(interpreted.stdout, outcome.stdout, "engines agree");
    }
}
