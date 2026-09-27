// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.5: cond arrow clauses, with the test evaluated once..

use ch04::eval_support::*;

mod ex_4_05 {
    use super::*;

    /// The evaluator whose cond supports `(test => recipient)` beside
    /// the plain and `else` clauses, evaluated directly so the test
    /// runs exactly once.
    pub struct WithArrow;

    impl WithArrow {
        fn eval_cond(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
            for clause in operand_items(exp)? {
                let parts = clause.list_items()?;
                let predicate = parts.first().cloned().unwrap_or(Value::Nil);
                if ch04::sec_4_1::is_else_clause(&clause)? {
                    return self.eval_sequence(&parts[1..], env);
                }
                // The arrow clause selects its recipient by value.
                if parts.len() == 3 && matches!(&parts[1], Value::Sym(s) if &**s == "=>") {
                    let tested = self.eval(&predicate, env)?;
                    if ch04::sec_4_1::is_true(&tested) {
                        let recipient = self.eval(&parts[2], env)?;
                        return self.apply_procedure(&recipient, &[tested]);
                    }
                    continue;
                }
                let tested = self.eval(&predicate, env)?;
                if ch04::sec_4_1::is_true(&tested) {
                    return self.eval_sequence(&parts[1..], env);
                }
            }
            Ok(Value::boolean(false))
        }
    }

    impl Evaluator for WithArrow {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "cond") {
                return Ok(Step::Done(self.eval_cond(exp, env)?));
            }
            self.base_step(exp, env)
        }
    }

    /// The printed answers: the book's arrow-clause cond and a plain
    /// clause cond.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let program = "(cond ((assoc 'b '((a 1) (b 2))) => cadr) (else #f))\n(cond ((= 1 2) 'a) ((= 1 1) 'b))";
        let (values, _) = run_with(&WithArrow, program)?;
        Ok(printed(&values))
    }
}

#[test]
fn ex_4_05() {
    let values = ex_4_05::answers().expect("runs");
    // The arrow clause applies cadr to the entry assoc found.
    assert_eq!(values, vec!["2", "b"]);
}
