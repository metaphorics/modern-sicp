// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.54: `require` as a special
//! form. The book completes `analyze-require` -- a new clause in the
//! dispatch whose execution procedure evaluates the predicate, succeeds
//! with `ok` when it holds, and fails otherwise. The edition's clause
//! shape is a [`Cont`]: the solution installs one `SpecialHook` clause
//! for `require`, so the variant recognizes the form at every nesting
//! depth -- inside `an-element-of` exactly as at the driver -- and the
//! base language keeps `require` installed as an ordinary procedure, so
//! the two routes are observationally equal.

use ch04::eval_support::{
    AMB_SEED, Amb, Cont, EvalResult, SchemeError, SpecialHook, is_tagged_list, is_true,
    operand_items, setup_amb_environment, with_eval_stack,
};
use sicp_runtime::{Env, Value};

mod ex_4_54 {
    use super::*;

    /// Whether `exp` is a `require` form, the book's `require?`.
    fn is_require(exp: &Value) -> bool {
        is_tagged_list(exp, "require")
    }

    /// The predicate of a `require` form, the book's
    /// `require-predicate`.
    fn require_predicate(exp: &Value) -> EvalResult {
        operand_items(exp)?
            .into_iter()
            .next()
            .ok_or_else(|| SchemeError::TypeMismatch("require with no predicate".to_owned()))
    }

    /// The book's `analyze-require` as a dispatch clause: evaluate the
    /// predicate, answer `ok` through the rest of the computation when
    /// it holds, fail when it does not.
    fn analyze_require(amb: &Amb, exp: &Value, env: &Rc<Env>, k: &Cont) -> Option<EvalResult> {
        if !is_require(exp) {
            return None;
        }
        let predicate = require_predicate(exp).ok()?;
        let k_outer = std::rc::Rc::clone(k);
        Some(amb.eval_with(
            &predicate,
            env,
            std::rc::Rc::new(move |amb, pred_value| {
                if is_true(&pred_value) {
                    k_outer(amb, Value::sym("ok"))
                } else {
                    amb.fail()
                }
            }),
        ))
    }

    use std::rc::Rc;

    /// The variant evaluator: the section's engine with `require` as a
    /// dispatch clause.
    pub struct RequireSpecial {
        amb: Amb,
    }

    impl RequireSpecial {
        /// Creates the variant from a seeded engine.
        ///
        /// # Errors
        /// [`SchemeError::ZeroSeed`] when `seed` is zero.
        pub fn new(seed: u64) -> Result<Self, SchemeError> {
            let amb = Amb::new(seed)?;
            let hook: SpecialHook = Rc::new(analyze_require);
            amb.install_special_hook(hook);
            Ok(Self { amb })
        }

        /// Starts a new problem, the driver's entry for a parsed form.
        ///
        /// # Errors
        /// [`SchemeError::Backtrack`] when every execution fails;
        /// whatever the evaluation raises otherwise.
        pub fn run_form(&self, form: &Value, env: &Rc<Env>) -> EvalResult {
            self.amb.run_form(form, env)
        }

        /// Reads `text` as one form and runs it as a new problem.
        ///
        /// # Errors
        /// The reader's parse error, or the evaluation's.
        pub fn run(&self, text: &str, env: &Rc<Env>) -> EvalResult {
            self.amb.run(text, env)
        }

        /// The next answer of the problem in flight.
        ///
        /// # Errors
        /// [`SchemeError::Backtrack`] when the search runs dry.
        pub fn try_again(&self) -> EvalResult {
            self.amb.try_again()
        }
    }

    /// The exercise's three probes.
    pub struct Report {
        /// `(require (> 2 1))` answers ok.
        pub holds: bool,
        /// `(require (> 1 2))` runs dry.
        pub exhausted: bool,
        /// The pruned search answers 2 then 4.
        pub evens: Vec<String>,
    }

    /// Runs the probes on one variant evaluator setup.
    ///
    /// # Panics
    /// Panics when a probe raises an object error.
    #[must_use]
    pub fn report() -> Report {
        with_eval_stack(move || {
            let ev = RequireSpecial::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            let library = "
                (define (an-element-of items)
                  (require (not (null? items)))
                  (amb (car items) (an-element-of (cdr items))))
                (define (even? n) (= (remainder n 2) 0))
            ";
            for form in sicp_runtime::read_program(library).expect("parses") {
                ev.run_form(&form, &env).expect("the library runs");
            }
            let holds = ev
                .run("(require (> 2 1))", &env)
                .map(|v| sicp_runtime::print_value(&v))
                .expect("the requirement holds")
                == "ok";
            let exhausted = matches!(
                ev.run("(require (> 1 2))", &env),
                Err(SchemeError::Backtrack)
            );
            let mut evens = Vec::new();
            if let Ok(first) = ev.run(
                "(let ((x (an-element-of '(1 2 3 4 5)))) \
                 (require (even? x)) x)",
                &env,
            ) {
                evens.push(sicp_runtime::print_value(&first));
                while let Ok(value) = ev.try_again() {
                    evens.push(sicp_runtime::print_value(&value));
                }
            }
            Report {
                holds,
                exhausted,
                evens,
            }
        })
    }
}

#[test]
fn ex_4_54() {
    let report = ex_4_54::report();
    // A held requirement answers ok.
    assert!(report.holds);
    // An unheld requirement fails: the search runs dry.
    assert!(report.exhausted);
    // Inside a search the clause prunes the odd branches.
    assert_eq!(report.evens, vec!["2", "4"]);
}
