// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.46: operand order. The engine
//! evaluates an application's operands left to right --
//! `continue_args` walks the operand list from the front -- and the
//! demonstration pins it: the left operand displays first, the answers
//! enumerate in the (left, right) order the search implies, and a
//! `try-again` re-enters the walk without replaying the left operand's
//! side effects. The parser needs exactly this order: `parse-word`
//! consumes `*unparsed*` from the front, so a right-to-left evaluator
//! would match the last word of a sentence first and every parse would
//! fail.

use ch04::eval_support::{
    AMB_SEED, Amb, OutputSink, SchemeError, setup_amb_environment_in, with_eval_stack,
};

mod ex_4_46 {
    use super::*;

    /// The demonstration: each operand announces itself before choosing.
    const PROGRAM: &str = r"
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(list (begin (display 'left) (an-element-of '(1 2)))
      (begin (display 'right) (an-element-of '(1 2))))";

    /// The displayed text and the answers: the first, then two
    /// `try-again`s.
    ///
    /// # Panics
    /// Panics when the search raises an object error.
    #[must_use]
    pub fn trace() -> (String, Vec<String>) {
        with_eval_stack(move || {
            let (sink, cell) = OutputSink::buffer();
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment_in(&sink);
            let forms = sicp_runtime::read_program(PROGRAM).expect("parses");
            for form in &forms[..forms.len() - 1] {
                let _ = amb.run_form(form, &env);
            }
            let mut answers = Vec::new();
            if let Ok(first) = amb.run_form(forms.last().expect("a form"), &env) {
                answers.push(sicp_runtime::print_value(&first));
                for _ in 0..2 {
                    match amb.try_again() {
                        Ok(value) => answers.push(sicp_runtime::print_value(&value)),
                        Err(SchemeError::Backtrack) => break,
                        Err(error) => panic!("the search raised: {error}"),
                    }
                }
            }
            (cell.borrow().clone(), answers)
        })
    }
}

#[test]
fn ex_4_46() {
    let (displayed, answers) = ex_4_46::trace();
    // The answers enumerate left-major; the probe stops at two
    // try-agains after the first answer ((2 2) still exists beyond the
    // pinned prefix).
    assert_eq!(answers, vec!["(1 1)", "(1 2)", "(2 1)"]);
    // The left operand announced itself before the right one, exactly
    // once for the first answer. The third answer's `try-again` resumed
    // the LEFT choice, so the right operand expression re-evaluated and
    // announced itself again -- the book's failure continuations replay
    // the later operands the same way. Nothing is ever replayed for a
    // resume that stays after the done prefix, which is the
    // resumable-frame property.
    assert_eq!(displayed, "leftrightright");
}
