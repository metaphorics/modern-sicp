// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.36: operand order and argument
//! list construction.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    Config, DRIVER_WITH_GUARD, RuntimeFn, compile_block, controller_replacing_driver,
    make_compiled_evaluator, new_state,
};
use sicp_runtime::Value;

mod ex_5_36 {
    //! Exercise 5.36: default code evaluates operands right to left.
    //! The alternative evaluates left to right and uses end-adjoin to
    //! keep the argument list in source order.

    use super::*;

    fn run(left_to_right: bool) -> Result<(Vec<String>, Vec<i128>, usize), Fault> {
        let recorded = Rc::new(RefCell::new(Vec::<i128>::new()));
        let record: RuntimeFn = {
            let recorded = Rc::clone(&recorded);
            Rc::new(move |args| {
                let Some(Value::Int(n)) = args.first() else {
                    return Err(Fault::Parse("record needs an integer".to_owned()));
                };
                recorded.borrow_mut().push(*n);
                Ok(Value::Int(*n))
            })
        };
        let cfg = Config {
            left_to_right,
            ..ch05::sec_5_5::default_config()
        };
        let source = "(list (record 1) (record 2))";
        let (entry, block) = compile_block(&cfg, &new_state(), source)?;
        let operations = [("record".to_owned(), record)];
        let runtime: Vec<(String, RuntimeFn)> = operations.into_iter().collect();
        let controller = controller_replacing_driver(DRIVER_WITH_GUARD) + "\n" + &block;
        let mut evaluator = make_compiled_evaluator(Some(&controller), &[], &runtime, "")?;
        evaluator.arm_entry(&entry);
        evaluator.run()?;
        let order = recorded.borrow().clone();
        let transcript = evaluator.transcript();
        Ok((transcript, order, block.lines().count()))
    }

    pub fn ex_5_36() -> Result<Vec<String>, Fault> {
        let (right_transcript, right_order, right_size) = run(false)?;
        let (left_transcript, left_order, left_size) = run(true)?;
        assert_eq!(right_order, vec![2, 1]);
        assert_eq!(left_order, vec![1, 2]);
        assert!(right_transcript.iter().any(|line| line == "(1 2)"));
        assert!(left_transcript.iter().any(|line| line == "(1 2)"));
        assert_eq!(right_size, left_size);
        Ok(vec![
            format!(
                "right-to-left recording order: {right_order:?}; list: {}",
                right_transcript.join(" ")
            ),
            format!(
                "left-to-right recording order: {left_order:?}; list: {}",
                left_transcript.join(" ")
            ),
            format!("instruction counts: {right_size} / {left_size}"),
        ])
    }

    #[test]
    fn ex_5_36_check() -> Result<(), Fault> {
        let lines = ex_5_36()?;
        assert!(lines[0].contains("[2, 1]"));
        assert!(lines[1].contains("[1, 2]"));
        Ok(())
    }
}
