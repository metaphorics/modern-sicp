// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.52: `if-fail`. The form
//! evaluates its first expression against the continuation as usual;
//! when the expression's whole search runs dry -- the `Backtrack` that
//! survives it -- the second expression evaluates against the same
//! continuation instead, so its value is the answer.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_52 {
    use super::*;

    /// The section library the examples search with.
    const LIBRARY: &str = r"
(define (require p) (if (not p) (amb)))
(define (even? n) (= (remainder n 2) 0))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))";

    /// The value of one `if-fail` program, or the raised error's text:
    /// the worker boundary carries `Send` data only, so the typed error
    /// renders before it crosses.
    ///
    /// # Panics
    /// Panics when the program raises an object error.
    pub fn run(program: &str) -> Result<String, String> {
        let program = format!("{LIBRARY}\n{program}");
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            let forms = sicp_runtime::read_program(&program).expect("parses");
            for form in &forms[..forms.len() - 1] {
                let _ = amb.run_form(form, &env);
            }
            amb.run_form(forms.last().expect("a form"), &env)
                .map(|value| sicp_runtime::print_value(&value))
                .map_err(|error: SchemeError| error.to_string())
        })
    }
}

#[test]
fn ex_4_52() {
    // No even element: the first expression's search runs dry and the
    // alternative's value is the answer.
    assert_eq!(
        ex_4_52::run(
            "(if-fail (let ((x (an-element-of '(1 3 5)))) \
             (require (even? x)) x) 'all-odd)"
        )
        .expect("the program runs"),
        "all-odd"
    );
    // With 8 in the list the first expression succeeds and answers 8.
    assert_eq!(
        ex_4_52::run(
            "(if-fail (let ((x (an-element-of '(1 3 5 8)))) \
             (require (even? x)) x) 'all-odd)"
        )
        .expect("the program runs"),
        "8"
    );
}
