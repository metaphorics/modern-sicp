// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.24: `cond` as a new basic
//! controller form, not reduced to `if`. The controller gains
//! `ev-cond`, a loop over the clause list that tests each predicate
//! through `eval-dispatch` until one is true or an `else` turns up,
//! then evaluates the selected clause's actions with `ev-sequence`,
//! so the clause's last expression is still in tail position.

use ch05::sec_5_2::{Fault, OpHandler};
use ch05::sec_5_4::{compose_controller, make_evaluator, operation, splice_controller};
use sicp_runtime::Value;

mod ex_5_24 {
    //! Exercise 5.24: implement `cond` as a new basic special form
    //! without reducing it to `if`, with a loop that tests the
    //! predicates of successive clauses and `ev-sequence` for the
    //! actions of the true one.

    use super::*;

    /// The typed fault a malformed clause raises.
    fn op_error(message: impl Into<String>) -> Fault {
        Fault::Op {
            op: String::new(),
            message: message.into(),
            step: 0,
        }
    }

    /// The items of a proper list word.
    fn items_of(word: &Value) -> Option<Vec<Value>> {
        word.list_items().ok()
    }

    /// The items when `word` is the tagged list `(tag ...)`.
    fn tagged(word: &Value, tag: &str) -> Option<Vec<Value>> {
        let items = items_of(word)?;
        if matches!(items.first(), Some(Value::Sym(name)) if name.as_ref() == tag) {
            Some(items)
        } else {
            None
        }
    }

    /// Whether the clause is the trailing `(else ...)`.
    fn is_else_clause(clause: &Value) -> bool {
        tagged(clause, "else").is_some()
    }

    /// The clause word's `(predicate, actions)` pair.
    fn clause_parts(clause: &Value) -> Result<(Value, Vec<Value>), Fault> {
        let items = items_of(clause).ok_or_else(|| op_error("a cond clause is not a list"))?;
        let predicate = items
            .first()
            .cloned()
            .ok_or_else(|| op_error("a cond clause has no predicate"))?;
        Ok((predicate, items.into_iter().skip(1).collect()))
    }

    /// The `ev-cond` loop: the clause under test rides in `proc`,
    /// whose live value every enclosing argument loop has already
    /// saved on the stack. A selected clause with no actions returns
    /// the predicate's value, and no true clause with no `else`
    /// answers `false`, exactly what `cond->if` would have produced.
    const EV_COND: &str = "ev-cond
  (save continue)
  (assign unev (op cond-clauses) (reg exp))
ev-cond-loop
  (test (op no-clauses?) (reg unev))
  (branch (label ev-cond-no-true-clause))
  (assign val (op first-clause) (reg unev))
  (assign unev (op rest-clauses) (reg unev))
  (test (op cond-else-clause?) (reg val))
  (branch (label ev-cond-else))
  (assign exp (op cond-predicate) (reg val))
  (save val)
  (save unev)
  (save env)
  (save continue)
  (assign continue (label ev-cond-decide))
  (goto (label eval-dispatch))
ev-cond-decide
  (restore continue)
  (restore env)
  (restore unev)
  (restore proc)
  (test (op true?) (reg val))
  (branch (label ev-cond-selected))
  (goto (label ev-cond-loop))
ev-cond-selected
  (assign unev (op cond-actions) (reg proc))
  (goto (label ev-cond-actions))
ev-cond-else
  (assign unev (op cond-actions) (reg val))
  (goto (label ev-cond-actions))
ev-cond-actions
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-cond-empty-actions))
  (goto (label ev-sequence))
ev-cond-empty-actions
  (restore continue)
  (goto (reg continue))
ev-cond-no-true-clause
  (restore continue)
  (assign val (const #f))
  (goto (reg continue))";

    /// The dispatch tests for the basic form, spliced ahead of the
    /// application test.
    const COND_TEST: &str = "
  (test (op cond?) (reg exp))
  (branch (label ev-cond))";

    /// The clause operations the basic form needs.
    fn operations() -> Vec<(&'static str, OpHandler)> {
        vec![
            operation("cond?", |args| {
                let [word] = args else {
                    return Err(op_error("cond?: needs one argument"));
                };
                Ok(Value::boolean(tagged(word, "cond").is_some()))
            }),
            operation("cond-clauses", |args| {
                let [word] = args else {
                    return Err(op_error("cond-clauses: needs one argument"));
                };
                let items =
                    tagged(word, "cond").ok_or_else(|| op_error("cond-clauses needs a cond"))?;
                Ok(Value::list(items.into_iter().skip(1).collect()))
            }),
            operation("no-clauses?", |args| {
                let [word] = args else {
                    return Err(op_error("no-clauses?: needs one argument"));
                };
                let empty = items_of(word).is_some_and(|items| items.is_empty());
                Ok(Value::boolean(empty))
            }),
            operation("first-clause", |args| {
                let [word] = args else {
                    return Err(op_error("first-clause: needs one argument"));
                };
                let items = items_of(word).ok_or_else(|| op_error("first-clause needs clauses"))?;
                items
                    .first()
                    .cloned()
                    .ok_or_else(|| op_error("first-clause of no clauses"))
            }),
            operation("rest-clauses", |args| {
                let [word] = args else {
                    return Err(op_error("rest-clauses: needs one argument"));
                };
                let items = items_of(word).ok_or_else(|| op_error("rest-clauses needs clauses"))?;
                Ok(Value::list(items.into_iter().skip(1).collect()))
            }),
            operation("cond-else-clause?", |args| {
                let [word] = args else {
                    return Err(op_error("cond-else-clause?: needs one argument"));
                };
                Ok(Value::boolean(is_else_clause(word)))
            }),
            operation("cond-predicate", |args| {
                let [clause] = args else {
                    return Err(op_error("cond-predicate: needs one argument"));
                };
                Ok(clause_parts(clause)?.0)
            }),
            operation("cond-actions", |args| {
                let [clause] = args else {
                    return Err(op_error("cond-actions: needs one argument"));
                };
                Ok(Value::list(clause_parts(clause)?.1))
            }),
        ]
    }

    /// The exercise's controller: the cond test in the dispatch and
    /// the ev-cond entry ahead of the error entries.
    fn controller() -> String {
        splice_controller(&compose_controller(&[]), COND_TEST, EV_COND)
    }

    /// Runs one session and answers the last value printed.
    fn value_of(source: &str) -> Result<String, Fault> {
        let mut evaluator = make_evaluator(&controller(), &operations(), source)?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        let at = transcript.len().saturating_sub(2);
        Ok(transcript[at].clone())
    }

    /// The cond sessions: the three-clause classify with an `else`,
    /// the bodyless clause whose value is its test, and the cond with
    /// no true clause and no `else`.
    #[test]
    fn ex_5_24() -> Result<(), Fault> {
        let classify = "(define (classify n) (cond ((= n 0) (quote zero)) ((= n 1) (quote one)) (else (quote many))))";
        assert_eq!(value_of(&format!("{classify}\n(classify 0)"))?, "zero");
        assert_eq!(value_of(&format!("{classify}\n(classify 1)"))?, "one");
        assert_eq!(value_of(&format!("{classify}\n(classify 7)"))?, "many");
        assert_eq!(value_of("(cond ((= 1 1)))")?, "#t");
        assert_eq!(value_of("(cond ((= 1 2)))")?, "#f");
        assert_eq!(
            value_of("(cond ((= 1 2) (quote no)) ((= 2 2) (quote yes)))")?,
            "yes"
        );
        Ok(())
    }
}
