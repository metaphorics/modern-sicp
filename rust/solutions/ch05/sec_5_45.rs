// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.45: compiled, interpreted,
//! and special-purpose factorial stack use.

use ch05::sec_5_2::Fault;
use ch05::sec_5_5::{
    DRIVER_WITH_GUARD, compile_block, controller_replacing_driver, default_config,
    make_compiled_evaluator, new_state,
};
use sicp_runtime::Value;

mod ex_5_45 {
    //! Exercise 5.45: compare stack pushes and maximum depth for the
    //! same recursive factorial on the 5.4 evaluator, the compiler,
    //! and the Figure 5.11 machine.

    use super::*;

    const FACTORIAL: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";
    const MEASURED_DRIVER: &str = "read-eval-print-loop
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
  (goto (label read-eval-print-loop))";

    fn counters(lines: &[String]) -> Result<(u64, u64), Fault> {
        let line = lines
            .iter()
            .rev()
            .find(|line| line.starts_with("(total-pushes"))
            .ok_or_else(|| Fault::Parse("missing stack statistics".to_owned()))?;
        let pushes = line
            .split("total-pushes = ")
            .nth(1)
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| Fault::Parse("bad push count".to_owned()))?;
        let depth = line
            .split("maximum-depth = ")
            .nth(1)
            .and_then(|rest| rest.strip_suffix(')'))
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| Fault::Parse("bad maximum depth".to_owned()))?;
        Ok((pushes, depth))
    }

    fn interpreted_at(n: u32) -> Result<(u64, u64), Fault> {
        let controller = ch05::sec_5_4::compose_controller(&[("driver", MEASURED_DRIVER)]);
        let source = format!("{FACTORIAL}\n(factorial {n})");
        let mut evaluator = ch05::sec_5_4::make_evaluator(&controller, &[], &source)?;
        evaluator.run()?;
        counters(&evaluator.transcript())
    }

    fn compiled_at(n: u32) -> Result<(u64, u64), Fault> {
        let (entry, block) = compile_block(&default_config(), &new_state(), FACTORIAL)?;
        let controller = controller_replacing_driver(&format!(
            "{}\n{}",
            DRIVER_WITH_GUARD
                .split_once("read-eval-print-loop")
                .map_or("", |(before, _)| before),
            MEASURED_DRIVER
        )) + "\n"
            + &block;
        let mut evaluator =
            make_compiled_evaluator(Some(&controller), &[], &[], &format!("(factorial {n})"))?;
        evaluator.arm_entry(&entry);
        evaluator.run()?;
        counters(&evaluator.transcript())
    }

    fn special_at(n: u32) -> Result<(u64, u64), Fault> {
        let mut machine = ch05::sec_5_2::factorial_machine();
        machine.set_register("n", Value::Int(i128::from(n)))?;
        machine.start()?;
        Ok(machine.stack_statistics())
    }

    pub fn ex_5_45() -> Result<Vec<String>, Fault> {
        let mut rows = Vec::new();
        for n in [5, 10] {
            let interpreted = interpreted_at(n)?;
            let compiled = compiled_at(n)?;
            let special = special_at(n)?;
            rows.push(format!(
                "n={n}: interpreted {}/{}, compiled {}/{}, special-purpose {}/{}",
                interpreted.0, interpreted.1, compiled.0, compiled.1, special.0, special.1
            ));
        }
        Ok(rows)
    }

    #[test]
    fn ex_5_45_check() -> Result<(), Fault> {
        let rows = ex_5_45()?;
        assert!(rows[0].contains("interpreted 144/28"), "{rows:?}");
        assert!(rows[0].contains("compiled 31/14"), "{rows:?}");
        assert!(rows[0].contains("special-purpose 8/8"), "{rows:?}");
        Ok(())
    }
}
