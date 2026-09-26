// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.35: the expression behind
//! Figure 5.18.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{Linkage, compile_program, default_config, new_state_seeded};

mod ex_5_35 {
    //! Exercise 5.35: seed the compiler's label counter at fourteen and
    //! reproduce Figure 5.18, under the edition's required const-entry
    //! spelling.

    use super::*;

    const SOURCE: &str = "(define (f x) (+ x (g (+ x 2))))";

    pub fn ex_5_35() -> Result<Vec<String>, Fault> {
        let seq = compile_program(
            &default_config(),
            &new_state_seeded(14),
            SOURCE,
            &Linkage::Next,
        )?;
        let listing = seq.stmts.join("\n");
        assert!(listing.contains("entry16"));
        assert!(listing.contains("after-lambda15"));
        assert!(listing.contains("after-call23"));
        Ok(vec![
            "expression: (define (f x) (+ x (g (+ x 2))))".to_owned(),
            format!("Figure 5.18 reproduced: {} controller statements", seq.stmts.len()),
            listing,
            "the entry operand is (const entry16), because the 5.2 assembler rejects label operands".to_owned(),
        ])
    }

    #[test]
    fn ex_5_35_check() -> Result<(), Fault> {
        let answers = ex_5_35()?;
        assert!(answers[2].starts_with("(assign val (op make-compiled-procedure) (const entry16)"));
        assert!(answers[2].contains("after-call23"));
        Ok(())
    }
}
