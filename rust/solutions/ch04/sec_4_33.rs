// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.33: quoted lists under the lazy
//! regime. With the 4.2.3 procedural `cons`/`car`/`cdr` defined, a
//! quoted list is ordinary data and the procedural `car` cannot message
//! it; the fix reroutes quote handling: a quoted list lifts into the
//! `cons` chain that builds the same elements as lazy pairs.

use ch04::eval_support::*;

mod ex_4_33 {
    use super::*;

    /// Ben's session prefix: the procedural pairs of 4.2.3 plus
    /// `list-ref`, typed at the driver.
    const DEFS: &str = "\
(define (cons x y) (lambda (m) (m x y)))
(define (car z) (z (lambda (p q) p)))
(define (cdr z) (z (lambda (p q) q)))
(define (list-ref items n) (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))";

    /// The section's lazy evaluator, with quoted lists lifted: a
    /// quotation of a non-empty proper list rewrites into the `cons`
    /// chain of its quoted elements before the driver runs it.
    #[derive(Debug, Default)]
    pub struct LazyLiftedQuote;

    impl Evaluator for LazyLiftedQuote {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_quoted(exp) {
                let datum = text_of_quotation(exp)?;
                if let Some(lifted) = sec_4_2::lifted_quote(&datum)? {
                    return Ok(Step::Tail(lifted, Rc::clone(env)));
                }
                return Ok(Step::Done(datum));
            }
            lazy_step(self, exp, env)
        }
    }

    impl LazyEval for LazyLiftedQuote {
        fn force_value(&self, value: Value) -> EvalResult {
            force_memo(self, value)
        }

        fn delay_operand(&self, _proc: &Rc<Closure>, _position: usize) -> Option<bool> {
            Some(true)
        }
    }

    /// The plain evaluator's error on `(car '(a b c))`, and the lifted
    /// session's three answers.
    ///
    /// # Errors
    /// The plain run raises by design and its message travels in the
    /// answer.
    pub fn answers() -> Result<(String, Vec<String>), SchemeError> {
        let plain_error = run_lazy(&Lazy, &format!("{DEFS}\n(car '(a b c))"))
            .expect_err("the procedural car cannot message an ordinary pair")
            .to_string();
        let (values, _) = run_lazy(
            &LazyLiftedQuote,
            &format!("{DEFS}\n(car '(a b c))\n(car (cdr '(a b c)))\n(list-ref '(a b c d) 3)"),
        )?;
        Ok((plain_error, printed(&values)))
    }
}

#[test]
fn ex_4_33() {
    let (plain, lifted) = ex_4_33::answers().expect("lifted session runs");
    // Plain: the quoted list is ordinary data; the procedural `car`
    // puts it in the operator position and the application fails.
    assert!(plain.contains("not a procedure"));
    assert!(plain.contains("(a b c)"));
    // Lifted: the quote builds true lazy pairs, so the procedural
    // selectors work, including `list-ref` over the quoted list.
    assert_eq!(lifted.last(), Some(&"d".to_owned()));
    assert!(lifted.contains(&"a".to_owned()));
    assert!(lifted.contains(&"b".to_owned()));
}
