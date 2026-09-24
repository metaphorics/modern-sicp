// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.15: the halting-problem diagonal, run. `halts?` gets a
// fixed verdict each run; both verdicts violate its contract..

use ch04::eval_support::*;

mod ex_4_15 {
    use super::*;

    /// The bounded evaluator: every step consumes one unit of the
    /// budget, so a program that never halts ends as a budget error
    /// instead of hanging the host.
    pub struct Bounded {
        remaining: Cell<u64>,
    }

    impl Bounded {
        /// A bounded evaluator with `budget` steps.
        #[must_use]
        pub fn new(budget: u64) -> Self {
            Self {
                remaining: Cell::new(budget),
            }
        }
    }

    impl Evaluator for Bounded {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            let remaining = self.remaining.get();
            if remaining == 0 {
                return Err(SchemeError::TypeMismatch(
                    "the step budget ran out: the program was still running".to_owned(),
                ));
            }
            self.remaining.set(remaining - 1);
            self.base_step(exp, env)
        }
    }

    /// Runs `(try try)` under one fixed verdict of `halts?`.
    ///
    /// # Errors
    /// The budget error of the diverging run.
    pub fn try_try(verdict: bool) -> Result<String, SchemeError> {
        let program = format!(
            "(define (run-forever) (run-forever))\n(define (halts? p a) {verdict})\n(define (try p) (if (halts? p p) (run-forever) 'halted))\n(try try)"
        );
        let (values, _) = run_with(&Bounded::new(100_000), &program)?;
        Ok(printed(&values).last().cloned().unwrap_or_default())
    }

    /// Answers both outcomes: the optimistic verdict burns the budget
    /// and the pessimistic one contradicts itself.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let optimistic = try_try(true).expect_err("run-forever diverges");
        Ok((optimistic.to_string(), pessimistic_value()?))
    }

    fn pessimistic_value() -> Result<String, SchemeError> {
        try_try(false)
    }
}

#[test]
fn ex_4_15() {
    let (optimistic, pessimistic) = ex_4_15::answers().expect("runs");
    // If halts? says (try try) halts, the program runs forever -- the
    // budget proves it was still running.
    assert!(optimistic.contains("the step budget ran out"));
    // If halts? says it does not halt, the program halts anyway.
    assert_eq!(pessimistic, "halted");
}
