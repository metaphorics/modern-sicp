// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.28: forcing the operator. Under
//! the lazy rules `(id +)` answers a thunk whose body is `+`, so the
//! application clause must force the operator before `apply` can
//! dispatch; a variant that skips the forcing fails with the thunk in
//! the operator position.

use ch04::eval_support::*;

mod ex_4_28 {
    use super::*;

    /// The probe: `id`'s body answers the delayed `+`, and the result
    /// is applied.
    const PROGRAM: &str = "(define (id x) x)\n((id +) 2 3)";

    /// The exercise's point, made by its negation: the same clause with
    /// the operator left unforced. Everything else is the section's
    /// lazy application.
    #[derive(Debug, Default)]
    pub struct NoOperatorForcing;

    impl Evaluator for NoOperatorForcing {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_application(exp) && !is_special_form(exp) {
                let operator = first_of(exp)?;
                let operands = operand_items(exp)?;
                // The one changed line: `eval`, not `actual-value`.
                let proc = self.eval(&operator, env)?;
                return match &proc {
                    Value::Closure(_) => {
                        let args: Vec<Value> = operands
                            .iter()
                            .map(|operand| delay_it(operand.clone(), env))
                            .collect();
                        self.apply_delayed(&proc, &args)
                    }
                    other => {
                        let args = sec_4_2::list_of_arg_values(self, &operands, env)?;
                        self.apply_procedure(other, &args).map(Step::Done)
                    }
                };
            }
            // The section's clauses for everything else, including the
            // forced `if` predicate.
            lazy_step(self, exp, env)
        }
    }

    impl LazyEval for NoOperatorForcing {
        fn force_value(&self, value: Value) -> EvalResult {
            force_memo(self, value)
        }

        fn delay_operand(&self, _proc: &Rc<Closure>, _position: usize) -> Option<bool> {
            Some(true)
        }
    }

    impl NoOperatorForcing {
        /// The compound half of the application, inlined: extend the
        /// captured environment with the delayed bindings and run the
        /// body.
        ///
        /// # Errors
        /// Whatever the body raises.
        fn apply_delayed(&self, proc: &Value, args: &[Value]) -> StepResult {
            let Value::Closure(closure) = proc else {
                return Err(SchemeError::NotProcedure(proc.clone()));
            };
            let frame = extend_environment(
                closure_name_of(closure),
                &closure.params,
                closure.rest.as_ref(),
                args,
                &closure.env,
            )?;
            self.step_sequence(&closure.body, &frame)
        }
    }

    fn closure_name_of(closure: &Closure) -> &str {
        closure.name.as_deref().unwrap_or("#[compound-procedure]")
    }

    /// The forced answer and the unforced error message.
    ///
    /// # Errors
    /// The unforced variant raises by design and its message travels in
    /// the answer.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let (values, _) = run_lazy(&Lazy, PROGRAM)?;
        let forced = printed(&values).last().cloned().unwrap_or_default();
        let error = run_with(&NoOperatorForcing, PROGRAM)
            .expect_err("the thunk reaches apply unforced")
            .to_string();
        Ok((forced, error))
    }
}

#[test]
fn ex_4_28() {
    let (forced, error) = ex_4_28::answers().expect("forced run succeeds");
    // With the forcing: the operator's thunk is demanded, `+` runs,
    // and 5 comes back.
    assert_eq!(forced, "5");
    // Without it: the thunk itself lands in the operator position, and
    // apply rejects it -- the demonstration the exercise asks for.
    assert!(error.contains("not a procedure"), "error was: {error}");
    assert!(error.contains("thunk"));
}
