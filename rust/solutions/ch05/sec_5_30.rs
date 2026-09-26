// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.30: error signaling inside
//! the evaluator.
//!
//! (a) An unbound variable lookup answers a distinguished condition
//! code, a tagged word no user value can spell, and `ev-variable`
//! tests for it and goes to `signal-error`.
//!
//! (b) Every primitive application is checked: `apply-primitive-
//! procedure` answers the condition code when the primitive refuses
//! the application (a wrong operand count, `car` of a non-pair,
//! division by zero), and `primitive-apply` tests for it. Both paths
//! land in the base controller's `signal-error`, whose `user-print`
//! reports the condition's detail and whose return to the driver
//! loop leaves a clean stack, reinitialized at the next interaction.

use ch05::sec_5_2::{Fault, OpHandler};
use ch05::sec_5_4::{
    apply_object_primitive, compose_controller, condition_word, environment_of, is_condition,
    make_evaluator, operation, primitive_name, splice_controller,
};
use sicp_runtime::Value;

mod ex_5_30 {
    //! Exercise 5.30: our evaluator currently catches only one kind
    //! of error, unknown expression types. Change the lookup
    //! operation to return a distinguished condition code for an
    //! unbound variable, arrange for `primitive-apply` to check a
    //! condition code from a checking `apply-primitive-procedure`,
    //! and make the structure work.

    use super::*;

    /// The typed fault a malformed operation use raises.
    fn op_error(message: impl Into<String>) -> Fault {
        Fault::Op {
            op: String::new(),
            message: message.into(),
            step: 0,
        }
    }

    /// The detail a `Fault::Op` carries, the message the condition
    /// code reports.
    fn fault_message(fault: &Fault) -> String {
        match fault {
            Fault::Op { message, .. } => message.clone(),
            other => other.to_string(),
        }
    }

    /// The checking lookup: an unbound variable answers the
    /// condition code instead of escaping the machine.
    fn checking_lookup() -> (&'static str, OpHandler) {
        operation("lookup-variable-value", |args| {
            let [word, base] = args else {
                return Err(op_error(
                    "lookup-variable-value needs a variable and an environment",
                ));
            };
            let Value::Sym(name) = word else {
                return Err(op_error("lookup-variable-value needs a variable"));
            };
            let environment = environment_of(base, "lookup-variable-value")?;
            match environment.lookup(name) {
                Ok(value) => Ok(value),
                Err(_) => Ok(condition_word(
                    "unbound-variable",
                    &format!("unbound variable: {name}"),
                )),
            }
        })
    }

    /// The checking primitive application: a refused application
    /// answers the condition code instead of escaping the machine.
    fn checking_primitive_apply() -> (&'static str, OpHandler) {
        operation("apply-primitive-procedure", |args| {
            let [proc, argl] = args else {
                return Err(op_error(
                    "apply-primitive-procedure needs a procedure and an operand list",
                ));
            };
            let Some(name) = primitive_name(proc) else {
                return Err(op_error(
                    "apply-primitive-procedure needs a primitive procedure",
                ));
            };
            let values = argl
                .list_items()
                .map_err(|_| op_error("apply-primitive-procedure needs an operand list"))?;
            match apply_object_primitive(name, &values) {
                Ok(value) => Ok(value),
                Err(fault) => Ok(condition_word("primitive-failure", &fault_message(&fault))),
            }
        })
    }

    /// The two condition-code tests.
    fn condition_tests() -> Vec<(&'static str, OpHandler)> {
        vec![
            operation("variable-lookup-failed?", |args| {
                let [word] = args else {
                    return Err(op_error("variable-lookup-failed?: needs one argument"));
                };
                Ok(Value::boolean(is_condition(word, "unbound-variable")))
            }),
            operation("primitive-application-failed?", |args| {
                let [word] = args else {
                    return Err(op_error(
                        "primitive-application-failed?: needs one argument",
                    ));
                };
                Ok(Value::boolean(is_condition(word, "primitive-failure")))
            }),
        ]
    }

    /// `ev-variable` tests the lookup's condition code before
    /// continuing.
    const EV_VARIABLE_CHECKING: &str = "ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (test (op variable-lookup-failed?) (reg val))
  (branch (label variable-lookup-failed))
  (goto (reg continue))
variable-lookup-failed
  (goto (label signal-error))";

    /// `primitive-apply` tests the application's condition code
    /// before restoring `continue`.
    const PRIMITIVE_APPLY_CHECKING: &str = "primitive-apply
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (test (op primitive-application-failed?) (reg val))
  (branch (label primitive-application-failed))
  (restore continue)
  (goto (reg continue))
primitive-application-failed
  (goto (label signal-error))";

    /// The checking evaluator's controller: the base controller with
    /// the two checking entries.
    fn controller() -> String {
        let base = compose_controller(&[
            ("ev-variable", EV_VARIABLE_CHECKING),
            ("primitive-apply", PRIMITIVE_APPLY_CHECKING),
        ]);
        splice_controller(&base, "", "")
    }

    /// The checking operations, overriding the base table's names.
    fn operations() -> Vec<(&'static str, OpHandler)> {
        let mut tests = condition_tests();
        vec![
            checking_lookup(),
            checking_primitive_apply(),
            tests.remove(0),
            tests.remove(0),
        ]
    }

    /// Builds the checking evaluator, runs `source` to the end of the
    /// queue, and answers the transcript.
    fn run(source: &str) -> Result<Vec<String>, Fault> {
        let mut evaluator = make_evaluator(&controller(), &operations(), source)?;
        evaluator.run()?;
        Ok(evaluator.transcript())
    }

    /// The caught failures, each pinned, and one clean factorial that
    /// still answers 120, so the checks did not slow the correct path
    /// down or change its answers.
    #[test]
    fn ex_5_30() -> Result<(), Fault> {
        // (b) car of a symbol: the primitive's refusal becomes a
        // condition, signal-error reports the detail, and the run
        // returns to the driver loop.
        let car_failure = run("(car 5)")?;
        assert!(car_failure.contains(&"car: not a pair: 5".to_owned()));

        // (b) division by zero, the exercise's named case.
        let division_failure = run("(/ 1 0)")?;
        assert!(division_failure.contains(&"division by zero".to_owned()));

        // (a) the unbound variable, the lookup's condition code.
        let unbound_failure = run("no-such-variable")?;
        assert!(unbound_failure.contains(&"unbound variable: no-such-variable".to_owned()));

        // (b) a wrong operand count.
        let arity_failure = run("(cons 1)")?;
        assert!(arity_failure.contains(&"cons: needs two arguments".to_owned()));

        // The clean path: correct answers, unchanged.
        let clean =
            run("(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial 5)")?;
        let at = clean.len().saturating_sub(2);
        assert_eq!(clean[at], "120");
        Ok(())
    }
}
