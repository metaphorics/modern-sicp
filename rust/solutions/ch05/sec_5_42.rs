// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.42: lexical-address code
//! generation and execution.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    Config, DRIVER_WITH_GUARD, compile_block, controller_replacing_driver,
    lexical_global_environment, lexical_operations, make_compiled_evaluator, new_state,
};

mod ex_5_42 {
    //! Exercise 5.42: bound variables use frame/displacement constants;
    //! names not present in the compile-time environment fall back to
    //! global lookup.

    use super::*;

    const EXAMPLE: &str = "(((lambda (x y) (lambda (a b c d e) ((lambda (y z) (* x y z)) (* a b x) (+ c d x)))) 3 4) 1 2 4 6 8)";

    pub fn ex_5_42() -> Result<Vec<String>, Fault> {
        let cfg = Config {
            lexical: true,
            ..ch05::sec_5_5::default_config()
        };
        let (entry, block) = compile_block(&cfg, &new_state(), EXAMPLE)?;
        assert!(block.contains("(const 0) (const 1)"), "{block}");
        assert!(block.contains("(const 2) (const 0)"), "{block}");
        let global = lexical_global_environment();
        let operations = lexical_operations(&global);
        let controller = controller_replacing_driver(DRIVER_WITH_GUARD) + "\n" + &block;
        let mut evaluator = make_compiled_evaluator(Some(&controller), &operations, &[], "")?;
        evaluator.arm_entry(&entry);
        evaluator.run()?;
        let transcript = evaluator.transcript();
        assert!(
            transcript.iter().any(|line| line == "234"),
            "{transcript:?}"
        );
        Ok(vec![
            "lexical code: z=(0 1), y=(0 0), x=(2 0)".to_owned(),
            format!("nested-lambda result: {}", transcript.join(" ")),
        ])
    }

    #[test]
    fn ex_5_42_check() -> Result<(), Fault> {
        let lines = ex_5_42()?;
        assert!(lines[0].contains("x=(2 0)"));
        assert!(lines[1].contains("234"));
        Ok(())
    }
}
