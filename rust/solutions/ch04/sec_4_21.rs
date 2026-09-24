// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.21: recursion without define..

use ch04::eval_support::*;

mod ex_4_21 {
    use super::*;

    /// The book's applicative-order factorial and the completed `f`,
    /// both as plain expressions the base evaluator runs.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let fact = "((lambda (n) ((lambda (fact) (fact fact n))\n  (lambda (ft k) (if (= k 1) 1 (* k (ft ft (- k 1)))))))\n 10)";
        let f = "(define (f x)\n  ((lambda (even? odd?)\n     (even? even? odd? x))\n   (lambda (ev? od? n)\n     (if (= n 0) true (od? ev? od? (- n 1))))\n   (lambda (ev? od? n)\n     (if (= n 0) false (ev? ev? od? (- n 1))))))\n(f 10)";
        let program = format!("{fact}\n{f}");
        let (values, _) = run_with(&Base, &program)?;
        Ok(printed(&values))
    }
}

#[test]
fn ex_4_21() {
    let values = ex_4_21::answers().expect("runs");
    assert_eq!(values, vec!["3628800", "ok", "#t"]);
}
