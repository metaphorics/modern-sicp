// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.13: make-unbound! removes the binding from the frame the
// expression evaluates in -- the first frame, the one the unbinding
// form owns. A name the first frame does not bind is an error: silently
// unbinding an outer name from an inner scope would break shadowing..

use ch04::eval_support::*;

mod ex_4_13 {
    use super::*;

    /// The evaluator with `make-unbound!` installed.
    pub struct WithUnbound;

    impl Evaluator for WithUnbound {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_tagged_list(exp, "make-unbound!") {
                let items = exp.list_items()?;
                let Some(Value::Sym(name)) = items.get(1).cloned() else {
                    return Err(SchemeError::TypeMismatch(format!(
                        "not a name to unbind: {exp}"
                    )));
                };
                let removed = env.frame.borrow_mut().remove(&*name);
                return match removed {
                    Some(_) => Ok(Step::Done(Value::sym("ok"))),
                    None => Err(SchemeError::TypeMismatch(format!(
                        "make-unbound!: not bound in the first frame: {name}"
                    ))),
                };
            }
            self.base_step(exp, env)
        }
    }

    /// Answers the lookup after unbinding a shadowing name and the
    /// error of unbinding a name the first frame does not bind.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let program = "(define x 1)\n(let ((x 2)) (make-unbound! x))\nx";
        let (values, _) = run_with(&WithUnbound, program)?;
        let shadowed_gone = printed(&values).last().cloned().unwrap_or_default();
        let again =
            run_with(&WithUnbound, "(make-unbound! x)").expect_err("x is not in the first frame");
        let message = again.to_string();
        assert!(message.contains("make-unbound!"), "{message}");
        Ok(vec![shadowed_gone, message])
    }
}

#[test]
fn ex_4_13() {
    let values = ex_4_13::answers().expect("runs");
    // Unbinding the inner x leaves the outer one visible.
    assert_eq!(values[0], "1");
    assert_eq!(
        values[1],
        "type mismatch: make-unbound!: not bound in the first frame: x"
    );
}
