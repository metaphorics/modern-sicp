// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.4: and and or as special forms, with short circuit..

use ch04::eval_support::*;

mod ex_4_04 {
    use super::*;

    /// The evaluator with `and` and `or` installed as special forms.
    pub struct WithAndOr;

    impl WithAndOr {
        /// The book's `eval-and`: the first false stops the evaluation
        /// and is the value; otherwise the last value is the answer.
        fn eval_and(&self, exps: &[Value], env: &Rc<Env>) -> EvalResult {
            let mut value = Value::boolean(true);
            for exp in exps {
                let tested = self.eval(exp, env)?;
                if !ch04::sec_4_1::is_true(&tested) {
                    return Ok(Value::boolean(false));
                }
                value = tested;
            }
            Ok(value)
        }

        /// The book's `eval-or`: the first true value stops the
        /// evaluation and is the value; otherwise the answer is false.
        fn eval_or(&self, exps: &[Value], env: &Rc<Env>) -> EvalResult {
            for exp in exps {
                let tested = self.eval(exp, env)?;
                if ch04::sec_4_1::is_true(&tested) {
                    return Ok(tested);
                }
            }
            Ok(Value::boolean(false))
        }
    }

    impl Evaluator for WithAndOr {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "and") {
                let exps = operand_items(exp)?;
                return Ok(Step::Done(self.eval_and(&exps, env)?));
            }
            if is_tagged_list(exp, "or") {
                let exps = operand_items(exp)?;
                return Ok(Step::Done(self.eval_or(&exps, env)?));
            }
            self.base_step(exp, env)
        }
    }

    /// The printed values of the and/or probes, short circuit included:
    /// a later operand that would raise never evaluates.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let program = "(and)\n(and 1 2 3)\n(and 1 #f (car 5))\n(or)\n(or #f 7 (car 5))\n(or #f #f)";
        let (values, _) = run_with(&WithAndOr, program)?;
        Ok(printed(&values))
    }
}

#[test]
fn ex_4_04() {
    let values = ex_4_04::answers().expect("runs");
    assert_eq!(values, vec!["#t", "3", "#f", "#f", "7", "#f"]);
}
