// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.6: let as a derived expression, re-derived through the
// seam even though the grammar carries it in the base, so the rewrite
// and its scoping rule are pinned..

use ch04::eval_support::*;

mod ex_4_06 {
    use super::*;

    /// The evaluator whose `let` is the derived expression of this
    /// exercise: the rewrite runs at every nesting depth.
    pub struct WithLetDerived;

    impl Evaluator for WithLetDerived {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_let(exp) {
                return Ok(Step::Tail(let_to_combination(exp)?, Rc::clone(env)));
            }
            self.base_step(exp, env)
        }
    }

    /// Answers the printed rewrite of the book's shape and the value
    /// the derived evaluator produces, inits evaluated in the outer
    /// environment.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let form = read("(let ((x 3) (y 4)) (+ x y))").expect("read");
        let rewritten = let_to_combination(&form)?;
        let (values, _) = run_with(&WithLetDerived, "(let ((x 3) (y 4)) (+ x y))")?;
        let value = printed(&values).last().cloned().unwrap_or_default();
        // The initializers still see the outer x, so y is 5, not 3.
        let scoping = "(define x 5)\n(let ((x 3) (y x)) y)";
        let (values, _) = run_with(&WithLetDerived, scoping)?;
        assert_eq!(printed(&values).last(), Some(&"5".to_owned()));
        assert_eq!(
            run_base("(define x 5)\n(let ((x 3) (y x)) y)").last(),
            Some(&"5".to_owned())
        );
        Ok((print_value(&rewritten), value))
    }
}

#[test]
fn ex_4_06() {
    let (rewritten, value) = ex_4_06::answers().expect("runs");
    // The rewrite is exactly the book's combination, and it evaluates
    // to the book's answer.
    assert_eq!(rewritten, "((lambda (x y) (+ x y)) 3 4)");
    assert_eq!(value, "7");
}
