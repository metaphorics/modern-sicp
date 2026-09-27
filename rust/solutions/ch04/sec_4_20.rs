// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.20: letrec as a derived expression..

use ch04::eval_support::*;

mod ex_4_20 {
    use super::*;

    /// The book's `letrec->let`: every name bound to the unassigned
    /// marker first, then one `set!` per name, then the body.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on a malformed form.
    pub fn letrec_to_let(exp: &Value) -> EvalResult {
        let items = exp.list_items()?;
        let bindings = items
            .get(1)
            .cloned()
            .ok_or_else(|| SchemeError::TypeMismatch(format!("malformed letrec: {exp}")))?;
        let body: Vec<Value> = items.into_iter().skip(2).collect();
        let mut names = Vec::new();
        let mut inits = Vec::new();
        for binding in bindings.list_items()? {
            let pair = binding.list_items()?;
            let (Some(name), Some(init)) = (pair.first(), pair.get(1)) else {
                return Err(SchemeError::TypeMismatch(format!(
                    "malformed letrec binding: {binding}"
                )));
            };
            names.push(name.clone());
            inits.push(init.clone());
        }
        let outer_bindings = Value::list(
            names
                .iter()
                .map(|name| {
                    Value::list(vec![
                        name.clone(),
                        Value::list(vec![Value::sym("quote"), unassigned()]),
                    ])
                })
                .collect(),
        );
        let mut forms = Vec::new();
        for (name, init) in names.iter().zip(&inits) {
            forms.push(Value::list(vec![
                Value::sym("set!"),
                name.clone(),
                init.clone(),
            ]));
        }
        forms.extend(body);
        let body_form = if forms.len() == 1 {
            forms.remove(0)
        } else {
            ch04::sec_4_1::make_begin(&forms)
        };
        Ok(Value::list(vec![
            Value::sym("let"),
            outer_bindings,
            body_form,
        ]))
    }

    /// The evaluator with `letrec` installed and the unassigned check
    /// its semantics need.
    pub struct WithLetrec;

    impl Evaluator for WithLetrec {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "letrec") {
                return Ok(Step::Tail(letrec_to_let(exp)?, Rc::clone(env)));
            }
            if is_variable(exp) {
                let Value::Sym(name) = exp else {
                    return Err(SchemeError::TypeMismatch("not a variable".to_owned()));
                };
                let value = lookup_variable_value(name, env)?;
                if value == unassigned() {
                    return Err(SchemeError::TypeMismatch(format!(
                        "the variable {name} is read before its define runs"
                    )));
                }
                return Ok(Step::Done(value));
            }
            self.base_step(exp, env)
        }
    }

    /// Answers the mutual-recursion value and the premature-read error.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let mutual = "(define (f x)\n  (letrec ((even? (lambda (n) (if (= n 0) true (odd? (- n 1)))))\n             (odd? (lambda (n) (if (= n 0) false (even? (- n 1))))))\n    (even? x)))\n(f 10)";
        let premature = "(letrec ((a (* b 2)) (b 3)) a)";
        let (values, _) = run_with(&WithLetrec, mutual)?;
        let mutual_value = printed(&values).last().cloned().unwrap_or_default();
        let error = run_with(&WithLetrec, premature).expect_err("b is read unassigned");
        Ok((mutual_value, error.to_string()))
    }
}

#[test]
fn ex_4_20() {
    let (mutual, premature) = ex_4_20::answers().expect("runs");
    assert_eq!(mutual, "#t");
    assert!(premature.contains("before its define runs"), "{premature}");
}
