// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.39: lexical-address-lookup
//! and lexical-address-set! over the book's frame structure.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    CompiledEvaluator, Config, DRIVER_WITH_GUARD, compile_block, controller_replacing_driver,
    lexical_global_environment, lexical_operations, make_compiled_evaluator, new_state,
};

mod ex_5_39 {
    //! Exercise 5.39: lexical addresses walk frame number and
    //! displacement, and set! mutates the binding cell in place.

    use super::*;

    fn run(source: &str) -> Result<CompiledEvaluator, Fault> {
        let cfg = Config {
            lexical: true,
            ..ch05::sec_5_5::default_config()
        };
        let state = new_state();
        let (entry, block) = compile_block(&cfg, &state, source)?;
        let global = lexical_global_environment();
        let operations = lexical_operations(&global);
        let controller = controller_replacing_driver(DRIVER_WITH_GUARD) + "\n" + &block;
        let mut evaluator = make_compiled_evaluator(Some(&controller), &operations, &[], "")?;
        evaluator.arm_entry(&entry);
        evaluator.run()?;
        Ok(evaluator)
    }

    pub fn ex_5_39() -> Result<Vec<String>, Fault> {
        let evaluator =
            run("(define n 10)\n((lambda (cell) (set! cell (* cell 10)) (+ n cell)) 11)")?;
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"120".to_owned()), "{transcript:?}");
        let instructions = evaluator.machine().instruction_texts();
        assert!(
            instructions
                .iter()
                .any(|line| line.contains("lexical-address-lookup"))
        );
        assert!(
            instructions
                .iter()
                .any(|line| line.contains("lexical-address-set!"))
        );
        Ok(vec![
            format!("lexical machine session: {}", transcript.join(" ")),
            "lexical-address-lookup detects *unassigned*; lexical-address-set! mutates the addressed value cell".to_owned(),
        ])
    }

    #[test]
    fn ex_5_39_check() -> Result<(), Fault> {
        let lines = ex_5_39()?;
        assert!(lines[0].contains("120"));
        Ok(())
    }
}
