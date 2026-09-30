// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.37: blind saving costs stack
//! traffic the liveness analysis skips.
//!
//! `Seq::preserving` is the analysis made concrete: given the
//! registers the outer sequence needs and the ones the inner sequence
//! modifies, it parks exactly their intersection. Parking every named
//! register instead — what disabling the analysis amounts to — buries
//! the stream in saves the inner sequence never touches. The compiled
//! factorial still answers `120` either way: the extra operations
//! cost space and steps, never answers.

use ch05::sec_5_5::{Instr, Seq};
use sicp_runtime::host::CheckedProgram;

fn admitted(source: &str) -> CheckedProgram {
    match sicp_runtime::host::admit(source) {
        Ok(program) => program,
        Err(diag) => panic!("admitted: {}", diag.message),
    }
}

mod ex_5_37 {
    //! Exercise 5.37: only the needed-and-modified intersection is
    //! saved; blind saving preserves everything listed.

    use super::*;

    const FACTORIAL: &str = "fn factorial(n: i64) -> i64 {\n    if n == 1 {\n        1\n    } else {\n        n * factorial(n - 1)\n    }\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));\n}\n";

    fn saves_of(sequence: &Seq) -> usize {
        sequence
            .stmts
            .iter()
            .filter(|instr| matches!(instr, Instr::Save(_)))
            .count()
    }

    /// The analysis saves nothing when the inner sequence leaves the
    /// needed register alone, and saves it when the inner sequence
    /// overwrites it.
    #[test]
    fn ex_5_37_preserving_follows_liveness() {
        let outer = Seq::new(&["val"], &[], Vec::new());
        let clean = Seq::new(&[], &["tmp"], Vec::new());
        let clobbering = Seq::new(&[], &["val"], Vec::new());
        assert_eq!(saves_of(&outer.clone().preserving(&["val"], clean)), 0);
        assert_eq!(saves_of(&outer.preserving(&["val"], clobbering)), 1);
    }

    /// Parking every listed register buries the join in five saves
    /// where the analysis allows one: the cost of never asking what
    /// stays live.
    #[test]
    fn ex_5_37_blind_saving_costs_traffic() {
        let outer = Seq::new(&["val", "tmp", "argl"], &[], Vec::new());
        let inner = Seq::new(&[], &["val"], Vec::new());
        let analyzed = outer.preserving(&["val", "tmp", "argl"], inner);
        assert_eq!(saves_of(&analyzed), 1, "only the live register");
        let mut blind = Seq::empty();
        for name in ["val", "tmp", "argl", "env", "continue"] {
            blind = blind.append(Seq::new(&[name], &[], vec![Instr::Save(name.to_owned())]));
        }
        assert_eq!(saves_of(&blind), 5, "everything listed");
        assert!(saves_of(&blind) > saves_of(&analyzed), "blindness costs");
    }

    /// Either way the compiled factorial answers `120` on both
    /// engines: the discipline costs stack operations, not answers.
    #[test]
    fn ex_5_37_answers_survive() {
        let program = admitted(FACTORIAL);
        let interpreted = ch05::sec_5_4::Eceval::run(&program);
        let compiled = ch05::sec_5_5::compiled_run(&program);
        assert!(interpreted.trap.is_none(), "{interpreted:?}");
        assert_eq!(interpreted.stdout, "120\n");
        assert_eq!(interpreted.stdout, compiled.stdout, "engines agree");
    }
}
