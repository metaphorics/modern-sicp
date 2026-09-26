// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.44: open coding consults the
//! compile-time environment before treating a name as primitive.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Config, Linkage, compile_program, new_state};

mod ex_5_44 {
    //! Exercise 5.44: lambda parameters named `+` and `*` shadow the
    //! open-coded primitive names; free names remain eligible.

    use super::*;

    const SHADOWED: &str = "(lambda (+ * a b x y) (+ (* a x) (* b y)))";
    const FREE: &str = "(lambda (a b x y) (+ (* a x) (* b y)))";

    fn open_code_count(source: &str) -> Result<usize, Fault> {
        let cfg = Config {
            open_code: true,
            ..ch05::sec_5_5::default_config()
        };
        let seq = compile_program(&cfg, &new_state(), source, &Linkage::Next)?;
        Ok(seq
            .stmts
            .iter()
            .filter(|line| line.contains("(op +)") || line.contains("(op *)"))
            .count())
    }

    pub fn ex_5_44() -> Result<Vec<String>, Fault> {
        let shadowed = open_code_count(SHADOWED)?;
        let free = open_code_count(FREE)?;
        assert_eq!(shadowed, 0);
        assert!(free > 0);
        Ok(vec![
            format!("shadowed + and *: {shadowed} open-coded operations"),
            format!("free + and *: {free} open-coded operations"),
            "compile-time lambda frames prevent open coding of rebound names; top-level rebinding is outside this analysis".to_owned(),
        ])
    }

    #[test]
    fn ex_5_44_check() -> Result<(), Fault> {
        let lines = ex_5_44()?;
        assert!(lines[0].contains(": 0 "));
        assert!(lines[1].contains("open-coded operations"));
        Ok(())
    }
}
