// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.37: the blind-save cost when
//! preserving is disabled.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, Linkage, compile_and_go, compile_program, new_state};

mod ex_5_37 {
    //! Exercise 5.37: unconditional saves preserve every listed
    //! register, including registers the second sequence never needs.

    use super::*;

    const FACTORIAL: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

    fn counts(preserving_on: bool) -> Result<(usize, usize), Fault> {
        let cfg = Config {
            preserving_on,
            ..ch05::sec_5_5::default_config()
        };
        let seq = compile_program(&cfg, &new_state(), FACTORIAL, &Linkage::Next)?;
        let saves = seq
            .stmts
            .iter()
            .filter(|line| line.starts_with("(save "))
            .count();
        Ok((seq.stmts.len(), saves))
    }

    pub fn ex_5_37() -> Result<Vec<String>, Fault> {
        let on = counts(true)?;
        let off = counts(false)?;
        let mut evaluator = compile_and_go(
            &Config {
                preserving_on: false,
                ..ch05::sec_5_5::default_config()
            },
            &new_state(),
            FACTORIAL,
            "(factorial 5)",
        )?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"120".to_owned()), "{transcript:?}");
        assert!(off.0 > on.0 && off.1 > on.1, "on={on:?}, off={off:?}");
        Ok(vec![
            format!("preserving on: {} statements, {} save sites", on.0, on.1),
            format!("preserving off: {} statements, {} save sites", off.0, off.1),
            format!(
                "unconditional-save factorial session: {}",
                transcript.join(" ")
            ),
        ])
    }

    #[test]
    fn ex_5_37_check() -> Result<(), Fault> {
        let lines = ex_5_37()?;
        assert!(lines[0].contains("79"), "{lines:?}");
        assert!(lines[2].contains("120"));
        Ok(())
    }
}
