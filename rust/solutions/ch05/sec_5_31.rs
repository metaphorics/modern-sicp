// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.31: which evaluator saves
//! preserving eliminates in four combinations.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Linkage, compile_program, default_config, new_state};

mod ex_5_31 {
    //! Exercise 5.31: compile the four applications and list their saves.

    use super::*;

    const CASES: [&str; 4] = ["(f 'x 'y)", "((f) 'x 'y)", "(f (g 'x) y)", "(f (g 'x) 'y)"];

    fn saves(source: &str) -> Result<Vec<String>, Fault> {
        let seq = compile_program(&default_config(), &new_state(), source, &Linkage::Next)?;
        Ok(seq
            .stmts
            .into_iter()
            .filter(|line| line.starts_with("(save ") || line.starts_with("(restore "))
            .collect())
    }

    /// The four code-generated answers: quoted operands need nothing,
    /// so the first two combinations keep no saves at all; the last
    /// two keep the `proc` and `argl` pairs around `(g 'x)`, because
    /// the call it compiles modifies both while they stay live.
    pub fn ex_5_31() -> Result<Vec<String>, Fault> {
        let expected = [0, 0, 4, 4];
        let mut answers = Vec::new();
        for (index, source) in CASES.iter().enumerate() {
            let instructions = saves(source)?;
            assert_eq!(instructions.len(), expected[index], "{source}");
            answers.push(format!("{source}: {}", instructions.join(" ")));
        }
        Ok(answers)
    }

    #[test]
    fn ex_5_31_check() -> Result<(), Fault> {
        let answers = ex_5_31()?;
        assert!(answers[0].ends_with(": "));
        assert!(answers[1].ends_with(": "));
        assert!(answers[2].contains("(save proc)"));
        assert!(answers[2].contains("(save argl)"));
        assert!(answers[3].contains("(save proc)"));
        assert!(answers[3].contains("(save argl)"));
        Ok(())
    }
}
