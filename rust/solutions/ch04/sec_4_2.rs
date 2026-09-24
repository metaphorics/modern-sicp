// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.2: dispatch order and call-prefixed applications..

use ch04::eval_support::*;

mod ex_4_02 {
    use super::*;

    /// Louis Reasoner's reordered evaluator: the application clause
    /// runs before any special-form check, so `(define x 3)` reaches
    /// the operator lookup of an application.
    pub struct ApplicationsFirst;

    impl Evaluator for ApplicationsFirst {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if exp.is_pair() {
                let operator = ch04::sec_4_1::first_of(exp)?;
                let operands = operand_items(exp)?;
                let proc = self.eval(&operator, env)?;
                let args = self.list_of_values(&operands, env)?;
                return self.tail_apply(&proc, &args);
            }
            self.base_step(exp, env)
        }
    }

    /// The evaluator of Louis's part (b): procedure applications must
    /// start with the tag `call`; every other pair is a syntax error.
    pub struct CallSyntax;

    impl Evaluator for CallSyntax {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if !exp.is_pair() {
                return self.base_step(exp, env);
            }
            let special_form = [
                "quote", "set!", "define", "if", "lambda", "begin", "cond", "let",
            ];
            let head = ch04::sec_4_1::first_of(exp)?;
            let Value::Sym(head) = head else {
                return Err(SchemeError::TypeMismatch(format!(
                    "the application is missing its call tag: {exp}"
                )));
            };
            if &*head == "call" {
                let parts = exp.list_items()?;
                let operator = parts.get(1).cloned().unwrap_or(Value::Nil);
                let operands: Vec<Value> = parts.into_iter().skip(2).collect();
                let proc = self.eval(&operator, env)?;
                let args = self.list_of_values(&operands, env)?;
                return self.tail_apply(&proc, &args);
            }
            if special_form.contains(&&*head) {
                return self.base_step(exp, env);
            }
            Err(SchemeError::TypeMismatch(format!(
                "the application is missing its call tag: {exp}"
            )))
        }
    }

    /// Answers the error `(define x 3)` raises under applications-first
    /// dispatch and the value of the call-prefixed factorial at 5.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let error = run_with(&ApplicationsFirst, "(define x 3)")
            .expect_err("a define is not an application");
        let message = error.to_string();
        let program = "(define (factorial n)\n  (if (call = n 1)\n      1\n      (call * n (call factorial (call - n 1)))))\n(call factorial 5)";
        let (values, _) = run_with(&CallSyntax, program)?;
        let factorial = printed(&values).last().cloned().unwrap_or_default();
        // The untagged form is a syntax error in the new language.
        let bare = run_with(&CallSyntax, "(factorial 5)").expect_err("the call tag is required");
        assert!(bare.to_string().contains("call tag"));
        Ok((message, factorial))
    }
}

#[test]
fn ex_4_02() {
    let (define_error, call_factorial) = ex_4_02::answers().expect("runs");
    // Part (a): Louis's plan treats a definition as an application of
    // the unbound operator `define`.
    assert!(define_error.contains("unbound variable: define"));
    // Part (b): with `call` marking every application, the dispatch
    // order never matters again.
    assert_eq!(call_factorial, "120");
}
