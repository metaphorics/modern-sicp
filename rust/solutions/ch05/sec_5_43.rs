// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.43: internal definitions are
//! scanned out before procedure-body compilation.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, compile_and_go, compile_block, default_config, new_state};

mod ex_5_43 {
    //! Exercise 5.43: internal definitions become unassigned lambda
    //! bindings followed by assignments.

    use super::*;

    const PROGRAM: &str = "(define (sum) (define a 1) (define b 2) (+ a b))\n(sum)";

    pub fn ex_5_43() -> Result<Vec<String>, Fault> {
        let (_, plain_text) = compile_block(&default_config(), &new_state(), PROGRAM)?;
        let scanned_cfg = Config {
            scan_out: true,
            ..ch05::sec_5_5::default_config()
        };
        let (_, scanned_text) = compile_block(&scanned_cfg, &new_state(), PROGRAM)?;
        let mut scanned = compile_and_go(&scanned_cfg, &new_state(), PROGRAM, "")?;
        scanned.run()?;
        let transcript = scanned.transcript();
        assert!(transcript.contains(&"3".to_owned()), "{transcript:?}");
        assert!(plain_text.contains("define-variable!"));
        assert!(scanned_text.contains("*unassigned*"));
        assert_eq!(scanned_text.matches("(op define-variable!)").count(), 1);
        Ok(vec![
            "plain body: internal define compiles through define-variable!".to_owned(),
            "scanned body: *unassigned* bindings plus set!, no internal define operation"
                .to_owned(),
            format!("scanned session: {}", transcript.join(" ")),
        ])
    }

    #[test]
    fn ex_5_43_check() -> Result<(), Fault> {
        let lines = ex_5_43()?;
        assert!(lines[2].contains(";;; EC-Eval value: 3"));
        Ok(())
    }
}
