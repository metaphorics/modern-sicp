// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.47: compiled procedures can
//! call interpreted procedures through `compound-apply`.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, compile_and_go, new_state};

mod ex_5_47 {
    //! Exercise 5.47: the compound-call branch sets the continuation,
    //! saves it on the evaluator's stack, and jumps through `unev` to
    //! `compound-apply`.

    use super::*;

    pub fn ex_5_47() -> Result<Vec<String>, Fault> {
        let cfg = Config {
            compound_calls: true,
            ..ch05::sec_5_5::default_config()
        };
        let compiled = "(define (f n) (g (+ n 1)))";
        let source = "(define (g x) (* x 2))\n(f 5)";
        let mut evaluator = compile_and_go(&cfg, &new_state(), compiled, source)?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"12".to_owned()), "{transcript:?}");
        let sample: Vec<String> = evaluator
            .machine()
            .instruction_texts()
            .into_iter()
            .filter(|line| {
                line.contains("compound-procedure?")
                    || line.contains("compound-branch")
                    || line.contains("compound-apply")
                    || line.contains("(save continue)")
            })
            .collect();
        Ok(vec![
            format!("compound-call instructions: {}", sample.join("; ")),
            format!("session: {}", transcript.join(" ")),
        ])
    }

    #[test]
    fn ex_5_47_check() -> Result<(), Fault> {
        let lines = ex_5_47()?;
        assert!(lines[0].contains("compound-apply"));
        assert!(lines[1].contains("12"));
        Ok(())
    }
}
