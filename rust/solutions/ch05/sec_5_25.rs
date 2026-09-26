// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.25: normal-order evaluation
//! in the controller, based on the lazy evaluator of 4.2.
//!
//! The plan, made before the controller was written:
//!
//! 1. Thunks are machine words (the tagged `thunk` word of an
//!    expression and an environment), built by one operation,
//!    `make-thunk`, so the base word set carries the laziness and
//!    nothing else does.
//! 2. Bindings of thunked operands go through the unmodified
//!    environment: the parameter is bound to the thunk word itself,
//!    which `lookup-variable-value` answers like any bound value.
//!    Frames keep their shape, so `define-variable!` and
//!    `set-variable-value!` work unchanged.
//! 3. The controller changes in exactly three places: the argument
//!    loop makes a thunk per operand and adjoins it (no saves, no
//!    recursion, which is the laziness); `ev-variable` forces a
//!    thunked binding the first time the variable is read and
//!    memoizes by storing the forced value over the binding with
//!    `set-variable-value!`; `primitive-apply` forces any thunks left
//!    in `argl` before the primitive sees them, saving `proc`,
//!    `unev`, `argl`, and `continue` around each nested evaluation.
//!
//! One deviation is recorded: memoization is per binding, not per
//! thunk object. The book memoizes by mutating the thunk pair, which
//! the edition's immutable values cannot do; a thunk aliased into two
//! variables is recomputed. Every session here observes
//! single-variable references, where the two agree.

use ch05::sec_5_2::{Fault, OpHandler};
use ch05::sec_5_4::{
    compose_controller, is_thunk, make_evaluator, operation, splice_controller, thunk_parts,
    thunk_word,
};
use sicp_runtime::Value;

mod ex_5_25 {
    //! Exercise 5.25: modify the evaluator so that it uses
    //! normal-order evaluation, based on the lazy evaluator of 4.2.

    use super::*;

    /// The typed fault a malformed thunk use raises.
    fn op_error(message: impl Into<String>) -> Fault {
        Fault::Op {
            op: String::new(),
            message: message.into(),
            step: 0,
        }
    }

    /// The lazy argument loop replaces the book's save-heavy loop:
    /// making a thunk evaluates nothing, which is the laziness.
    const LAZY_ARGUMENT_LOOP: &str = "ev-appl-did-operator
  (restore unev)
  (restore env)
  (assign argl (op empty-arglist))
  (assign proc (reg val))
ev-appl-operand-loop
  (test (op no-operands?) (reg unev))
  (branch (label ev-appl-args-done))
  (assign exp
          (op first-operand)
          (reg unev))
  (assign val
          (op make-thunk)
          (reg exp)
          (reg env))
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (assign unev
          (op rest-operands)
          (reg unev))
  (goto (label ev-appl-operand-loop))
ev-appl-args-done
  (goto (label apply-dispatch))";

    /// Variable reads force and memoize: the forced value is stored
    /// over the binding, so the second read of the same variable is a
    /// plain value.
    const LAZY_EV_VARIABLE: &str = "ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (test (op thunk?) (reg val))
  (branch (label ev-variable-thunk))
  (goto (reg continue))
ev-variable-thunk
  (save exp)
  (save env)
  (save continue)
  (assign exp (op thunk-expression) (reg val))
  (assign env (op thunk-environment) (reg val))
  (assign continue (label ev-variable-forced))
  (goto (label eval-dispatch))
ev-variable-forced
  (restore continue)
  (restore env)
  (restore exp)
  (perform
   (op set-variable-value!) (reg exp) (reg val) (reg env))
  (goto (reg continue))";

    /// Primitive application forces the arguments first: values are
    /// rebuilt in order into `unev`, then moved back to `argl`.
    const LAZY_PRIMITIVE_APPLY: &str = "primitive-apply
  (assign unev (op empty-arglist))
force-args-loop
  (test (op no-args?) (reg argl))
  (branch (label force-args-done))
  (assign val (op first-arg) (reg argl))
  (assign argl (op rest-args) (reg argl))
  (test (op thunk?) (reg val))
  (branch (label force-args-one))
  (goto (label force-args-keep))
force-args-one
  (save proc)
  (save unev)
  (save argl)
  (save continue)
  (assign exp (op thunk-expression) (reg val))
  (assign env (op thunk-environment) (reg val))
  (assign continue (label force-args-back))
  (goto (label eval-dispatch))
force-args-back
  (restore continue)
  (restore argl)
  (restore unev)
  (restore proc)
force-args-keep
  (assign unev
          (op adjoin-arg)
          (reg val)
          (reg unev))
  (goto (label force-args-loop))
force-args-done
  (assign argl (reg unev))
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (restore continue)
  (goto (reg continue))";

    /// The thunk operations the three fragments name.
    fn operations() -> Vec<(&'static str, OpHandler)> {
        vec![
            operation("make-thunk", |args| {
                let [expression, environment] = args else {
                    return Err(op_error(
                        "make-thunk needs an expression and an environment",
                    ));
                };
                Ok(thunk_word(expression.clone(), environment.clone()))
            }),
            operation("thunk?", |args| {
                let [word] = args else {
                    return Err(op_error("thunk?: needs one argument"));
                };
                Ok(Value::boolean(is_thunk(word)))
            }),
            operation("thunk-expression", |args| {
                let [word] = args else {
                    return Err(op_error("thunk-expression: needs one argument"));
                };
                thunk_parts(word)
                    .map(|(expression, _)| expression)
                    .ok_or_else(|| op_error("thunk-expression needs a thunk"))
            }),
            operation("thunk-environment", |args| {
                let [word] = args else {
                    return Err(op_error("thunk-environment: needs one argument"));
                };
                thunk_parts(word)
                    .map(|(_, environment)| environment)
                    .ok_or_else(|| op_error("thunk-environment needs a thunk"))
            }),
            operation("no-args?", |args| {
                let [word] = args else {
                    return Err(op_error("no-args?: needs one argument"));
                };
                let empty = word.list_items().is_ok_and(|items| items.is_empty());
                Ok(Value::boolean(empty))
            }),
            operation("first-arg", |args| {
                let [word] = args else {
                    return Err(op_error("first-arg: needs one argument"));
                };
                word.list_items()
                    .map_err(|_| op_error("first-arg needs an operand list"))?
                    .into_iter()
                    .next()
                    .ok_or_else(|| op_error("first-arg of an empty list"))
            }),
            operation("rest-args", |args| {
                let [word] = args else {
                    return Err(op_error("rest-args: needs one argument"));
                };
                let items = word
                    .list_items()
                    .map_err(|_| op_error("rest-args needs an operand list"))?;
                Ok(Value::list(items.into_iter().skip(1).collect()))
            }),
        ]
    }

    /// The lazy evaluator's controller: the base fragments with the
    /// three replaced blocks and the base argument loop dropped.
    fn controller() -> String {
        let base = compose_controller(&[
            ("ev-variable", LAZY_EV_VARIABLE),
            ("ev-appl-did-operator", ""),
            ("argument-loop", LAZY_ARGUMENT_LOOP),
            ("primitive-apply", LAZY_PRIMITIVE_APPLY),
        ]);
        splice_controller(&base, "", "")
    }

    /// Builds the lazy evaluator, runs `source` to the end of the
    /// queue, and answers the transcript.
    fn run(source: &str) -> Result<Vec<String>, Fault> {
        let mut evaluator = make_evaluator(&controller(), &operations(), source)?;
        evaluator.run()?;
        Ok(evaluator.transcript())
    }

    /// The last value printed by a run.
    fn last_value(transcript: &[String]) -> &str {
        let at = transcript.len().saturating_sub(2);
        transcript[at].as_str()
    }

    /// The three proof groups: factorial still answers 120 under
    /// normal order; a procedure whose argument is never used never
    /// evaluates it; and a thunked operand runs once, not once per
    /// reference.
    #[test]
    fn ex_5_25() -> Result<(), Fault> {
        let factorial_lines =
            run("(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial 5)")?;
        assert_eq!(last_value(&factorial_lines), "120");

        let lazy_lines = run("(define (always-42 x) 42)\n(always-42 (car (quote ())))")?;
        assert_eq!(last_value(&lazy_lines), "42");

        let memo_lines = run("(define count 0)\n\
             (define (bump) (set! count (+ count 1)) count)\n\
             (define (use-twice x) (cons x x))\n\
             (use-twice (bump))\n\
             count")?;
        let values: Vec<&str> = memo_lines
            .iter()
            .filter(|line| line.as_str() == "(1 . 1)" || line.as_str() == "1")
            .map(String::as_str)
            .collect();
        assert_eq!(
            values,
            vec!["(1 . 1)", "1"],
            "the transcript: {memo_lines:?}"
        );
        Ok(())
    }
}
