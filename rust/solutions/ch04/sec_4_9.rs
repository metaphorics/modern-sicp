// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.9: iteration constructs as derived evaluation..

use ch04::eval_support::*;

mod ex_4_09 {
    use super::*;

    /// The evaluator with a `while` and an `until` loop, each evaluated
    /// directly: the predicate and the body re-enter the evaluator, so
    /// nested forms get the whole language.
    pub struct WithLoops;

    impl WithLoops {
        fn eval_while(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
            let items = exp.list_items()?;
            let predicate = items.get(1).cloned().unwrap_or(Value::Nil);
            let body: Vec<Value> = items.into_iter().skip(2).collect();
            loop {
                let tested = self.eval(&predicate, env)?;
                if !ch04::sec_4_1::is_true(&tested) {
                    return Ok(Value::boolean(false));
                }
                self.eval_sequence(&body, env)?;
            }
        }

        fn eval_until(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
            let items = exp.list_items()?;
            let predicate = items.get(1).cloned().unwrap_or(Value::Nil);
            let body: Vec<Value> = items.into_iter().skip(2).collect();
            loop {
                let tested = self.eval(&predicate, env)?;
                if ch04::sec_4_1::is_true(&tested) {
                    return Ok(Value::boolean(false));
                }
                self.eval_sequence(&body, env)?;
            }
        }
    }

    impl Evaluator for WithLoops {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "while") {
                return Ok(Step::Done(self.eval_while(exp, env)?));
            }
            if is_tagged_list(exp, "until") {
                return Ok(Step::Done(self.eval_until(exp, env)?));
            }
            self.base_step(exp, env)
        }
    }

    /// Answers the loop counters: while counts to 5, until runs while
    /// j stays at or below 12.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let program = "(define i 0)\n(while (< i 5) (set! i (+ i 1)))\ni\n(define j 10)\n(until (> j 12) (set! j (+ j 1)))\nj";
        let (values, _) = run_with(&WithLoops, program)?;
        Ok(printed(&values)
            .into_iter()
            .skip(2)
            .take(1)
            .chain(printed(&values).into_iter().skip(5).take(1))
            .collect())
    }
}

#[test]
fn ex_4_09() {
    let values = ex_4_09::answers().expect("runs");
    assert_eq!(values, vec!["5", "13"]);
}
