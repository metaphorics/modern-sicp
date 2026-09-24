// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.7: let* as nested lets; the derived rewrite suffices
// because the nested lets re-enter the dispatcher..

use ch04::eval_support::*;

mod ex_4_07 {
    use super::*;

    /// The evaluator with `let*` as a derived expression.
    pub struct WithLetStar;

    /// The book's `let*->nested-lets`: each binding becomes one `let`
    /// whose body is the next; no bindings leaves the body alone.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on a malformed form.
    pub fn let_star_to_nested_lets(exp: &Value) -> EvalResult {
        let items = exp.list_items()?;
        let bindings = items
            .get(1)
            .cloned()
            .ok_or_else(|| SchemeError::TypeMismatch(format!("malformed let*: {exp}")))?;
        let body: Vec<Value> = items.into_iter().skip(2).collect();
        let mut nested = match body.as_slice() {
            [] => return Err(SchemeError::TypeMismatch("empty let* body".to_owned())),
            [one] => one.clone(),
            many => ch04::sec_4_1::make_begin(many),
        };
        for binding in bindings.list_items()?.into_iter().rev() {
            nested = Value::list(vec![Value::sym("let"), Value::list(vec![binding]), nested]);
        }
        Ok(nested)
    }

    impl Evaluator for WithLetStar {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "let*") {
                return Ok(Step::Tail(let_star_to_nested_lets(exp)?, Rc::clone(env)));
            }
            self.base_step(exp, env)
        }
    }

    /// Evaluates the book's example and one nested `let*` inside
    /// another's body, answering the printed values.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let book = "(let* ((x 3)\n       (y (+ x 2))\n       (z (+ x y 5)))\n  (* x z))";
        let inner = "(let* ((x 3) (y (+ x 2))) (let* ((z x)) y))";
        let (values, _) = run_with(&WithLetStar, &format!("{book}\n{inner}"))?;
        Ok(printed(&values))
    }
}

#[test]
fn ex_4_07() {
    let values = ex_4_07::answers().expect("runs");
    // The book's example: x=3, y=5, z=11, and x*z is 39.
    assert_eq!(values[0], "39");
    // Adding the clause is sufficient: the nested lets of one let*,
    // and of a let* inside another's body, all re-enter this step.
    assert_eq!(values[1], "5");
}
