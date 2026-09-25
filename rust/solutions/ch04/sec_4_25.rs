// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.25: `unless` breaks under
//! applicative order. The lazy evaluator delays `unless`'s arms, so the
//! recursion bottoms out; strict primitives evaluate both arms, so the
//! same definition only stops at a budget.

use ch04::eval_support::*;
use std::cell::Cell;

mod ex_4_25 {
    use super::*;

    /// The strict applicative-order evaluator with a step budget: the
    /// host stand-in for "runs forever", which a test observes as an
    /// error rather than a hang.
    #[derive(Debug, Default)]
    pub struct BoundedStrict {
        fuel: Cell<u32>,
    }

    impl BoundedStrict {
        /// A strict evaluator with `fuel` steps before the descent
        /// error.
        #[must_use]
        pub fn new(fuel: u32) -> Self {
            Self {
                fuel: Cell::new(fuel),
            }
        }
    }

    impl Evaluator for BoundedStrict {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if self.fuel.get() == 0 {
                return Err(SchemeError::TypeMismatch(
                    "the step budget ran out: factorial is still descending".to_owned(),
                ));
            }
            self.fuel.set(self.fuel.get() - 1);
            self.base_step(exp, env)
        }
    }

    const UNLESS_FACTORIAL: &str = "\
(define (unless condition usual-value exceptional-value)
  (if condition exceptional-value usual-value))
(define (factorial n)
  (unless (= n 1) (* n (factorial (- n 1))) 1))";

    /// The three behaviors the exercise asks about: the lazy
    /// `factorial`, the armed `unless` under strict primitives, and the
    /// strict `factorial` under a budget.
    ///
    /// # Errors
    /// The lazy run must not raise; the strict runs raise by design and
    /// their messages travel in the answer.
    pub fn answers() -> Result<(String, String, String), SchemeError> {
        let (lazy_values, _) = run_lazy(&Lazy, &format!("{UNLESS_FACTORIAL}\n(factorial 5)"))?;
        let lazy_factorial = printed(&lazy_values).last().cloned().unwrap_or_default();

        let armed_error = run_with(
            &Base,
            "(define (unless condition usual-value exceptional-value) \
             (if condition exceptional-value usual-value))\n\
             (unless (= 1 1) (/ 1 0) 42)",
        )
        .expect_err("strict primitives evaluate the armed arm");

        let descent_error = run_with(
            &BoundedStrict::new(200),
            &format!("{UNLESS_FACTORIAL}\n(factorial 5)"),
        )
        .expect_err("the strict factorial never bottoms out");

        Ok((
            lazy_factorial,
            armed_error.to_string(),
            descent_error.to_string(),
        ))
    }
}

#[test]
fn ex_4_25() {
    let (lazy_factorial, armed, descending) = ex_4_25::answers().expect("lazy run succeeds");
    // Lazy: `(unless ...)` binds its arms as thunks, the condition
    // forces, and only the chosen arm is demanded -- the recursion
    // bottoms out at n = 1 and 120 comes back.
    assert_eq!(lazy_factorial, "120");
    // The edition's strict-primitive rule: under applicative order both
    // arms evaluate before `unless` is entered, so the armed division
    // fires (compare exercise 1.6).
    assert_eq!(armed, "division by zero");
    // Strict `factorial` still descends forever; the budget turns the
    // non-termination into the observable error.
    assert!(descending.contains("still descending"));
}
