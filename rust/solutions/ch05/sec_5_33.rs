// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.33: compare the compiled
//! recursive factorial with the alternative operand order.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Linkage, compile_and_go, compile_program, default_config, new_state};

mod ex_5_33 {
    //! Exercise 5.33: both procedures answer 120; the alternative
    //! compilation changes which value stays live across the recursive
    //! call, changing its saves and restores.

    use super::*;

    const FACTORIAL: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";
    const ALT: &str = "(define (factorial-alt n) (if (= n 1) 1 (* n (factorial-alt (- n 1)))))";

    fn summary(source: &str) -> Result<(usize, usize, String), Fault> {
        let seq = compile_program(&default_config(), &new_state(), source, &Linkage::Next)?;
        let pairs = seq
            .stmts
            .iter()
            .filter(|line| line.starts_with("(save "))
            .count();
        Ok((seq.stmts.len(), pairs, seq.stmts.join("\n")))
    }

    fn run(compiled: &str, source: &str) -> Result<Vec<String>, Fault> {
        let mut evaluator = compile_and_go(&default_config(), &new_state(), compiled, source)?;
        evaluator.run()?;
        Ok(evaluator.transcript())
    }

    /// The value of the last interaction: the transcript ends with the
    /// next prompt after the run stops at the dry input queue.
    fn last_value(transcript: &[String]) -> &str {
        transcript
            .get(transcript.len().saturating_sub(2))
            .map_or("", String::as_str)
    }

    pub fn ex_5_33() -> Result<Vec<String>, Fault> {
        let base = summary(FACTORIAL)?;
        let alt = summary(ALT)?;
        let base_run = run(FACTORIAL, "(factorial 5)")?;
        let alt_run = run(ALT, "(factorial-alt 5)")?;
        assert_eq!(last_value(&base_run), "120");
        assert_eq!(last_value(&alt_run), "120");
        Ok(vec![
            format!("factorial: {} statements, {} save sites", base.0, base.1),
            format!("factorial-alt: {} statements, {} save sites", alt.0, alt.1),
            format!("factorial session: {}", base_run.join(" ")),
            format!("factorial-alt session: {}", alt_run.join(" ")),
            "the recursive operand's position changes which register is live across its call"
                .to_owned(),
        ])
    }

    #[test]
    fn ex_5_33_check() -> Result<(), Fault> {
        let lines = ex_5_33()?;
        assert!(lines[0].contains("79"), "{lines:?}");
        assert!(lines[2].contains("120"));
        assert!(lines[3].contains("120"));
        Ok(())
    }
}
