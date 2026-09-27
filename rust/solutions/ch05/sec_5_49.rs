// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.49: a host-driven
//! read-compile-execute-print loop.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    DRIVER_WITH_GUARD, compile_block, controller_replacing_driver, default_config,
    make_compiled_evaluator, new_state,
};
use std::fmt::Write as _;

mod ex_5_49 {
    //! Exercise 5.49: each form is compiled once and executed by a
    //! controller chain; no evaluator dispatch runs the user forms.

    use super::*;

    fn chain_driver(entries: &[String]) -> String {
        let guard = DRIVER_WITH_GUARD
            .split_once("read-eval-print-loop")
            .map_or("", |(prefix, _)| prefix);
        let mut driver = format!(
            "{guard}\nread-eval-print-loop\n  (assign env (op get-global-environment))\n  (goto (label chain-start))\nprint-result\n  (goto (label read-eval-print-loop))\nchain-start\n"
        );
        for (index, entry) in entries.iter().enumerate() {
            let _ = write!(
                driver,
                "  (perform (op prompt-for-input) (const \";;; Compiled input:\"))\n  (assign continue (label print-{index}))\n  (goto (label {entry}))\nprint-{index}\n  (perform (op announce-output) (const \";;; Compiled value:\"))\n  (perform (op user-print) (reg val))\n"
            );
            if index + 1 == entries.len() {
                driver.push_str("  (goto (label machine-end))\n");
            }
        }
        driver
    }

    fn loop_forms(forms: &[&str]) -> Result<Vec<String>, Fault> {
        let state = new_state();
        let mut blocks = Vec::new();
        for form in forms {
            blocks.push(compile_block(&default_config(), &state, form)?);
        }
        let entries: Vec<String> = blocks.iter().map(|(entry, _)| entry.clone()).collect();
        let body = blocks
            .iter()
            .map(|(_, block)| block.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let controller = format!(
            "{}\n{}\nmachine-end",
            controller_replacing_driver(&chain_driver(&entries)),
            body
        );
        let mut evaluator = make_compiled_evaluator(Some(&controller), &[], &[], "")?;
        evaluator.run()?;
        Ok(evaluator.transcript())
    }

    pub fn ex_5_49() -> Result<Vec<String>, Fault> {
        let transcript = loop_forms(&[
            "(define (square n) (* n n))",
            "(square 12)",
            "(define (twice n) (+ n n))",
            "(twice 441)",
        ])?;
        let values: Vec<&str> = transcript
            .iter()
            .filter(|line| !line.starts_with(";;;"))
            .map(String::as_str)
            .collect();
        assert_eq!(values, ["ok", "144", "ok", "882"]);
        Ok(vec![format!("compiled loop: {}", transcript.join(" "))])
    }

    #[test]
    fn ex_5_49_check() -> Result<(), Fault> {
        let lines = ex_5_49()?;
        assert!(lines[0].contains("144"));
        assert!(lines[0].contains("882"));
        Ok(())
    }
}
