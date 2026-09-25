// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.30: the eval-sequence debate.
//! The text's sequence evaluates non-final expressions without
//! forcing; Cy's proposed sequence forces them with `actual-value`.
//! Ben's `for-each` works under both, because applying a procedure is
//! what forces; Cy's `p2` is where the two readings part: the text's
//! sequence never demands the `set!`, so `x` keeps its original value.

use ch04::eval_support::*;

mod ex_4_30 {
    use super::*;

    /// Cy's proposed `eval-sequence`: every non-final expression is
    /// forced with `actual-value`; the last stays a tail.
    #[derive(Debug, Default)]
    pub struct LazyCy;

    impl LazyEval for LazyCy {
        fn force_value(&self, value: Value) -> EvalResult {
            force_memo(self, value)
        }

        fn delay_operand(&self, _proc: &Rc<Closure>, _position: usize) -> Option<bool> {
            Some(true)
        }
    }

    impl Evaluator for LazyCy {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            lazy_step(self, exp, env)
        }

        fn step_sequence(&self, exps: &[Value], env: &Rc<Env>) -> StepResult {
            let Some((last, head)) = exps.split_last() else {
                return Err(SchemeError::TypeMismatch(
                    "empty sequence: EVAL-SEQUENCE".to_owned(),
                ));
            };
            for exp in head {
                self.actual_value(exp, env)?;
            }
            Ok(Step::Tail(last.clone(), Rc::clone(env)))
        }
    }

    /// Ben's `for-each`, with a counter folded into the procedure it
    /// applies.
    const FOR_EACH: &str = "\
(define count 0)
(define (for-each proc items)
  (if (null? items)
      'done
      (begin (proc (car items)) (for-each proc (cdr items)))))
(for-each (lambda (x) (set! count (+ count 1)) (display x)) (list 57 321 88))
count";

    /// Cy's procedures.
    const P1: &str = "(define (p1 x) (set! x (cons x '(2))) x)\n(p1 1)";
    const P2: &str = "(define (p2 x) (define (p e) e x) (p (set! x (cons x '(2)))))\n(p2 1)";

    /// The `for-each` answer under the text's sequence, the displayed
    /// trace included.
    pub fn for_each_text_sequence() -> Result<(String, Vec<String>), SchemeError> {
        run_session(&Lazy, FOR_EACH)
    }

    /// The `for-each` answer under Cy's forced sequence.
    pub fn for_each_cy_sequence() -> Result<(String, Vec<String>), SchemeError> {
        run_session(&LazyCy, FOR_EACH)
    }

    /// `(p1 1)` under both sequences.
    pub fn p1(ev: &impl LazyEval) -> Result<String, SchemeError> {
        run_last(ev, P1)
    }

    /// `(p2 1)` under both sequences.
    pub fn p2(ev: &impl LazyEval) -> Result<String, SchemeError> {
        run_last(ev, P2)
    }

    fn run_session(
        ev: &impl LazyEval,
        program: &str,
    ) -> Result<(String, Vec<String>), SchemeError> {
        let (values, displayed) = run_lazy(ev, program)?;
        Ok((displayed, printed(&values)))
    }

    fn run_last(ev: &impl LazyEval, program: &str) -> Result<String, SchemeError> {
        let (values, _) = run_lazy(ev, program)?;
        Ok(printed(&values).last().cloned().unwrap_or_default())
    }
}

#[test]
fn ex_4_30() {
    let (text_display, text_values) = ex_4_30::for_each_text_sequence().expect("runs");
    let (cy_display, cy_values) = ex_4_30::for_each_cy_sequence().expect("runs");
    // Part (a), Ben right: applying `proc` delays its argument, but
    // `display` is a strict primitive, so the element prints and the
    // counter runs under the text's sequence already.
    assert_eq!(text_display, "5732188");
    assert_eq!(&text_values[text_values.len() - 2..], &["done", "3"]);
    // Part (c), Cy's change does not affect the example: the same
    // trace under the forced sequence.
    assert_eq!(cy_display, "5732188");
    assert_eq!(&cy_values[cy_values.len() - 2..], &["done", "3"]);

    // Part (b), `(p1 1)`: the `set!` is a non-final expression the
    // evaluator evaluates either way, so both sequences answer the
    // extended pair.
    let text_p1 = ex_4_30::p1(&Lazy).expect("runs");
    let cy_p1 = ex_4_30::p1(&ex_4_30::LazyCy).expect("runs");
    assert_eq!(text_p1, "(1 2)");
    assert_eq!(cy_p1, "(1 2)");

    // `(p2 1)` is where the readings part: the text's sequence
    // evaluates `e` without forcing, so the `set!` inside the thunk
    // never runs and `x` keeps its original value; Cy forces `e`, the
    // `set!` fires, and the extended pair comes back.
    let text_p2 = ex_4_30::p2(&Lazy).expect("runs");
    let cy_p2 = ex_4_30::p2(&ex_4_30::LazyCy).expect("runs");
    assert_eq!(text_p2, "1");
    assert_eq!(cy_p2, "(1 2)");
}
