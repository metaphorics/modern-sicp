// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.26: Ben implements `unless` as
//! a derived expression, which works under the strict base but is
//! syntax, not a value; Alyssa keeps `unless` a procedure, which the
//! lazy evaluator makes useful over delayed arms and keeps first-class.

use ch04::eval_support::*;

mod ex_4_26 {
    use super::*;

    /// Ben's side: `unless` as a derived expression, rewritten into the
    /// `if` it names.
    #[derive(Debug, Default)]
    pub struct WithUnlessSpecial;

    impl Evaluator for WithUnlessSpecial {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "unless") {
                return Ok(Step::Tail(unless_to_if(exp)?, Rc::clone(env)));
            }
            self.base_step(exp, env)
        }
    }

    /// Rewrites `(unless c u e)` into `(if c e u)`.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on a malformed `unless`.
    pub fn unless_to_if(exp: &Value) -> EvalResult {
        let items = exp.list_items()?;
        let [_head, condition, usual, exceptional] = items.as_slice() else {
            return Err(SchemeError::TypeMismatch(format!(
                "malformed unless: {exp}"
            )));
        };
        Ok(Value::list(vec![
            Value::sym("if"),
            condition.clone(),
            exceptional.clone(),
            usual.clone(),
        ]))
    }

    const UNLESS_PROCEDURE: &str = "\
(define (unless condition usual-value exceptional-value)
  (if condition exceptional-value usual-value))";

    /// Maps the error-armed triples through `unless` in both readings.
    const ARMED_MAP: &str = "\
(map (lambda (t) (unless (car t) (cadr t) (caddr t)))
     '((#t (/ 1 0) 42) (#f 7 (/ 1 0))))";

    /// The five behaviors the debate asks for: Ben's special form under
    /// the strict base, its failure as a first-class value, Alyssa's
    /// procedure under the lazy evaluator, the mapped armed triples
    /// under both, and `apply` handing the procedure its arms delayed.
    ///
    /// # Errors
    /// The unbound-name probe raises by design and its message travels
    /// in the answer.
    pub fn answers() -> Result<Answers, SchemeError> {
        let (values, _) = run_with(&WithUnlessSpecial, "(unless (= 1 1) (/ 1 0) 42)")?;
        let special_armed = printed(&values).last().cloned().unwrap_or_default();

        let unbound = run_with(&WithUnlessSpecial, "(list unless)")
            .expect_err("syntax is not a value")
            .to_string();

        let (values, _) = run_lazy(
            &Lazy,
            &format!("{UNLESS_PROCEDURE}\n(unless (= 1 1) (/ 1 0) 42)"),
        )?;
        let procedure_armed = printed(&values).last().cloned().unwrap_or_default();

        let (values, _) = run_with(&WithUnlessSpecial, ARMED_MAP)?;
        let special_mapped = printed(&values).last().cloned().unwrap_or_default();

        let (values, _) = run_lazy(&Lazy, &format!("{UNLESS_PROCEDURE}\n{ARMED_MAP}"))?;
        let procedure_mapped = printed_forced(&Lazy, &values).pop().unwrap_or_default();

        let (values, _) = run_lazy(
            &Lazy,
            &format!("{UNLESS_PROCEDURE}\n(apply unless '(#f 7 (/ 1 0)))"),
        )?;
        let procedure_applied = printed(&values).last().cloned().unwrap_or_default();

        Ok(Answers {
            special_armed,
            unbound,
            procedure_armed,
            special_mapped,
            procedure_mapped,
            procedure_applied,
        })
    }

    /// The asserted answers of the exercise, one per probe.
    pub struct Answers {
        /// Ben's special form over the armed call.
        pub special_armed: String,
        /// The message when the special form is used as a value.
        pub unbound: String,
        /// Alyssa's procedure over the same armed call.
        pub procedure_armed: String,
        /// The armed triples mapped through the special form.
        pub special_mapped: String,
        /// The armed triples mapped through the procedure.
        pub procedure_mapped: String,
        /// The procedure handed to `apply`, arms still delayed.
        pub procedure_applied: String,
    }
}

#[test]
fn ex_4_26() {
    let answers = ex_4_26::answers().expect("runs");
    // Ben: the derived expression evaluates only the chosen arm, so
    // the armed call answers 42 under the strict base.
    assert_eq!(answers.special_armed, "42");
    // Alyssa's rejoinder: as syntax, `unless` is no longer a value --
    // the name lookup fails where a procedure would be handed over.
    assert!(answers.unbound.contains("unbound variable: unless"));
    // The lazy procedure answers the same armed call.
    assert_eq!(answers.procedure_armed, "42");
    // Both readings map the armed triples: only the chosen arm of each
    // call is ever demanded.
    assert_eq!(answers.special_mapped, "(42 7)");
    assert_eq!(answers.procedure_mapped, "(42 7)");
    // And the procedure stays first-class: `apply` hands it the quoted
    // arms, the condition forces, the armed arm never does.
    assert_eq!(answers.procedure_applied, "7");
}
