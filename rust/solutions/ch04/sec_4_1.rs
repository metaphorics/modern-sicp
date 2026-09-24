// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.1: operand evaluation order in list-of-values (A: the
// Rust host fixes the order, so the premise is reworded -- the base
// `list_of_values` is an explicit recursion this book writes out, and
// the exercise pins which order it evaluates in by construction)..

use ch04::eval_support::*;

mod ex_4_01 {
    use super::*;

    /// The right-to-left `list-of-values`: an exercise's own evaluator,
    /// identical to the base except that the operands are evaluated
    /// from the last to the first.
    pub struct RightToLeft;

    impl Evaluator for RightToLeft {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            self.base_step(exp, env)
        }

        fn list_of_values(&self, exps: &[Value], env: &Rc<Env>) -> EvalList {
            let mut values = Vec::with_capacity(exps.len());
            for exp in exps.iter().rev() {
                values.push(self.eval(exp, env)?);
            }
            values.reverse();
            Ok(values)
        }
    }

    /// Evaluates the order probe under both versions of
    /// `list-of-values` and answers the two display transcripts.
    pub fn order_transcripts() -> Result<(String, String), SchemeError> {
        let program = "(define (f a b) 'ok)\n(f (begin (display 1) 1) (begin (display 2) 2))";
        let (left_to_right, trace_left) = run_with(&Base, program)?;
        assert_eq!(printed(&left_to_right).last(), Some(&"ok".to_owned()));
        let (right_to_left, trace_right) = run_with(&RightToLeft, program)?;
        assert_eq!(printed(&right_to_left).last(), Some(&"ok".to_owned()));
        Ok((trace_left, trace_right))
    }
}

#[test]
fn ex_4_01() {
    let (left_to_right, right_to_left) = ex_4_01::order_transcripts().expect("runs");
    // The base evaluator's list-of-values evaluates left to right by
    // construction; the flipped recursion evaluates right to left.
    assert_eq!(left_to_right, "12");
    assert_eq!(right_to_left, "21");
}
