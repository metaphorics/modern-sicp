// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.34: compile the iterative
//! factorial and identify its tail transfer.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    compile_block, controller_replacing_driver, default_config, make_compiled_evaluator, new_state,
};

mod ex_5_34 {
    //! Exercise 5.34: `iter` calls itself in tail position, so the
    //! compiler transfers directly to its entry without saving a
    //! continuation.

    use super::*;

    const ITERATIVE: &str = "(define (factorial n) (define (iter product counter) (if (> counter n) product (iter (* counter product) (+ counter 1)))) (iter 1 1))";

    fn monitored_driver() -> String {
        ";; branches if the compiled entry is armed:
  (test (op compiled-entry-armed?))
  (branch (label external-entry))
read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input) (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output) (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))"
            .to_owned()
    }

    fn depth_at(n: u32) -> Result<(u64, u64), Fault> {
        let (entry, block) = compile_block(&default_config(), &new_state(), ITERATIVE)?;
        let controller = controller_replacing_driver(&monitored_driver()) + "\n" + &block;
        let mut evaluator =
            make_compiled_evaluator(Some(&controller), &[], &[], &format!("(factorial {n})"))?;
        evaluator.arm_entry(&entry);
        evaluator.run()?;
        let transcript = evaluator.transcript();
        let stats = transcript
            .iter()
            .find(|line| line.starts_with("(total-pushes"))
            .ok_or_else(|| Fault::Parse("no statistics printed".to_owned()))?;
        let pushes = stats
            .split("total-pushes = ")
            .nth(1)
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| Fault::Parse("bad push count".to_owned()))?;
        let depth = stats
            .split("maximum-depth = ")
            .nth(1)
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| Fault::Parse("bad maximum depth".to_owned()))?;
        Ok((pushes, depth))
    }

    pub fn ex_5_34() -> Result<Vec<String>, Fault> {
        let depths = [3, 4, 5]
            .map(depth_at)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        assert!(depths.iter().all(|(_, depth)| *depth == depths[0].1));
        Ok(vec![
            "iter tail call: (assign val (op compiled-procedure-entry) (reg proc)); (goto (reg val))".to_owned(),
            format!("n=3: pushes {}, depth {}", depths[0].0, depths[0].1),
            format!("n=4: pushes {}, depth {}", depths[1].0, depths[1].1),
            format!("n=5: pushes {}, depth {}", depths[2].0, depths[2].1),
            "the continuation register is not saved for the tail call; maximum depth is constant".to_owned(),
        ])
    }

    #[test]
    fn ex_5_34_check() -> Result<(), Fault> {
        let lines = ex_5_34()?;
        assert_eq!(
            lines[1].split("depth ").nth(1),
            lines[2].split("depth ").nth(1)
        );
        assert_eq!(
            lines[2].split("depth ").nth(1),
            lines[3].split("depth ").nth(1)
        );
        Ok(())
    }
}
