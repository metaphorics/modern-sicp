// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.48: `compile-and-run` as a
//! runtime primitive.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    RuntimeFn, State, bump_entry, compile_forms, default_config, eceval_controller,
    make_compiled_evaluator, new_state, statements_text,
};
use sicp_runtime::Value;

mod ex_5_48 {
    //! Exercise 5.48: a primitive compiles its quoted form and records
    //! a block for the next assembly; that machine runs the block and
    //! then resumes the read-eval-print driver.

    use super::*;

    const DEFINITION: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

    fn compile_and_run_runtime(
        state: State,
        blocks: Rc<RefCell<Vec<(String, String)>>>,
    ) -> (String, RuntimeFn) {
        let name = "compile-and-run".to_owned();
        let function: RuntimeFn = Rc::new(move |args| {
            let Some(expression) = args.first() else {
                return Err(Fault::Parse(
                    "compile-and-run needs one expression".to_owned(),
                ));
            };
            let seq = compile_forms(
                &default_config(),
                &state,
                std::slice::from_ref(expression),
                &ch05::sec_5_5::Linkage::Return,
            )?;
            let entry = format!("compiled-entry-run-{}", bump_entry(&state));
            let block = format!("{entry}\n{}", statements_text(&seq));
            blocks.borrow_mut().push((entry, block));
            Ok(Value::sym("ok"))
        });
        (name, function)
    }

    pub fn ex_5_48() -> Result<Vec<String>, Fault> {
        let state = new_state();
        let blocks = Rc::new(RefCell::new(Vec::new()));
        let extra = vec![compile_and_run_runtime(state, Rc::clone(&blocks))];
        let mut first = make_compiled_evaluator(
            None,
            &[],
            &extra,
            &format!("(compile-and-run '{DEFINITION})"),
        )?;
        first.run()?;
        let first_transcript = first.transcript();
        let block = blocks
            .borrow()
            .first()
            .cloned()
            .ok_or_else(|| Fault::Parse("compile-and-run recorded no code".to_owned()))?;
        let controller = format!("{}\n{}", eceval_controller(), block.1);
        let mut second = make_compiled_evaluator(Some(&controller), &[], &[], "(factorial 5)")?;
        second.arm_entry(&block.0);
        second.run()?;
        let second_transcript = second.transcript();
        assert!(
            first_transcript.contains(&"ok".to_owned()),
            "{first_transcript:?}"
        );
        assert!(
            second_transcript.contains(&"ok".to_owned()),
            "{second_transcript:?}"
        );
        assert!(
            second_transcript.contains(&"120".to_owned()),
            "{second_transcript:?}"
        );
        Ok(vec![
            format!("compile-and-run primitive: {}", first_transcript.join(" ")),
            format!(
                "compiled definition and call: {}",
                second_transcript.join(" ")
            ),
        ])
    }

    #[test]
    fn ex_5_48_check() -> Result<(), Fault> {
        let lines = ex_5_48()?;
        assert!(lines[0].contains("ok"));
        assert!(lines[1].contains("120"));
        Ok(())
    }
}
