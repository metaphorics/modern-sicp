// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.50: the metacircular evaluator compiled.
//!
//! The object-language evaluator source is [`ch05::sec_5_5::METACIRCULAR`],
//! the corpus's 4.1 text adapted to this machine. Compiled and run on the
//! 5.5.7 machine, its session answers `ok`, `120`, and `(tick tick tick)`.
//! The three measurements compare compiled factorial, the 5.4 interpreted
//! evaluator, and the compiled metacircular interpreter.

use ch05::sec_5_2::Fault;
use ch05::sec_5_4::compose_controller;
use ch05::sec_5_5::{compile_and_go, default_config, new_state};

const FACTORIAL: &str = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

fn metacircular_session() -> Result<(Vec<String>, u64), Fault> {
    let driver = "(m-eval '(factorial 5) the-global-environment)";
    let mut evaluator = compile_and_go(
        &default_config(),
        &new_state(),
        ch05::sec_5_5::METACIRCULAR,
        driver,
    )?;
    evaluator.run()?;
    Ok((evaluator.transcript(), evaluator.instruction_count()))
}

fn level_0_steps(n: u32) -> Result<u64, Fault> {
    let mut evaluator = compile_and_go(
        &default_config(),
        &new_state(),
        FACTORIAL,
        &format!("(factorial {n})"),
    )?;
    evaluator.run()?;
    Ok(evaluator.instruction_count())
}

fn level_1_pushes(n: u32) -> Result<u64, Fault> {
    let monitored = "read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input)
           (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output)
           (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))";
    let controller = compose_controller(&[("driver", monitored)]);
    let source = format!("{FACTORIAL}\n(factorial {n})");
    let mut evaluator = ch05::sec_5_4::make_evaluator(&controller, &[], &source)?;
    evaluator.run()?;
    let total = evaluator
        .transcript()
        .iter()
        .filter_map(|line| line.strip_prefix("(total-pushes = "))
        .filter_map(|rest| rest.split(' ').next()?.parse::<u64>().ok())
        .sum();
    Ok(total)
}

fn metacircular_wall_seconds() -> Result<(u64, f64), Fault> {
    let start = std::time::Instant::now();
    let (_, steps) = metacircular_session()?;
    Ok((steps, start.elapsed().as_secs_f64()))
}

mod ex_5_50 {
    //! Exercise 5.50: compile the metacircular evaluator and measure
    //! the cost of each interpretation level.

    use super::*;

    fn answers_5_50() -> Result<Vec<String>, Fault> {
        let (transcript, meta_steps) = metacircular_session()?;
        assert!(
            transcript.iter().any(|line| line == "120"),
            "{transcript:?}"
        );
        assert!(
            transcript.iter().any(|line| line == "(tick tick tick)"),
            "{transcript:?}"
        );
        let l0 = level_0_steps(5)?;
        let l1 = level_1_pushes(5)?;
        let (meta_repeat, wall) = metacircular_wall_seconds()?;
        Ok(vec![
            format!("compiled metacircular session: {}", transcript.join(" ")),
            format!("level 0 (compiled factorial), machine steps = {l0}"),
            format!("level 1 (interpreted factorial), monitored pushes = {l1}"),
            format!(
                "level 2 (compiled metacircular), machine steps = {meta_steps} (repeat {meta_repeat}, wall {wall:.3}s)"
            ),
            format!("interpretation price: level 2 over level 0 = {meta_steps}/{l0} machine steps"),
        ])
    }

    #[test]
    fn ex_5_50_check() -> Result<(), Fault> {
        let lines = answers_5_50()?;
        assert!(lines[0].contains("120"), "{lines:?}");
        assert!(lines[0].contains("(tick tick tick)"), "{lines:?}");
        let l0 = lines[1]
            .split("= ")
            .nth(1)
            .and_then(|rest| rest.parse::<u64>().ok())
            .ok_or_else(|| Fault::Parse("level 0 line was malformed".to_owned()))?;
        let l2 = lines[3]
            .split("= ")
            .nth(1)
            .and_then(|rest| rest.split(' ').next())
            .and_then(|rest| rest.parse::<u64>().ok())
            .ok_or_else(|| Fault::Parse("level 2 line was malformed".to_owned()))?;
        assert!(
            l2 > 100 * l0.max(1),
            "level 2 must pay its constant: {l2} vs {l0}"
        );
        Ok(())
    }
}
