// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.23: `cond` and `let` enter
//! the evaluator through transformer machine operations, the book's
//! sanctioned cheat: `cond->if` and `let->combination` are operations,
//! and the dispatch grows two tests ahead of the application test.
//! Each new entry transforms `exp` and re-enters `eval-dispatch`, so
//! the rest of the controller never knows the forms existed.

use ch05::sec_5_2::{Fault, OpHandler};
use ch05::sec_5_4::{base_controller, make_evaluator, operation, splice_controller};
use sicp_runtime::{Value, display_value};

mod ex_5_23 {
    //! Exercise 5.23: extend the evaluator to handle derived
    //! expressions such as `cond` and `let`, assuming the syntax
    //! transformers are available as machine operations.

    use super::*;

    /// The typed fault a malformed derived form raises.
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

    /// The book's `sequence->exp`: one expression is itself, many are
    /// a `begin`, and a bodyless clause's value is its own test.
    fn sequence(words: &[Value]) -> Value {
        match words.len() {
            1 => words[0].clone(),
            _ => Value::list(
                std::iter::once(Value::sym("begin"))
                    .chain(words.iter().cloned())
                    .collect(),
            ),
        }
    }

    /// The book's `cond->if`: a chain of `if`s ending in the else
    /// body, in the `false` variable when there is none, and with a
    /// bodyless clause's value its own test.
    fn cond_to_if(clauses: &[Value]) -> Result<Value, Fault> {
        let Some((first, rest)) = clauses.split_first() else {
            return Ok(Value::sym("false"));
        };
        let items = items_of(first).ok_or_else(|| op_error("cond->if: a clause is not a list"))?;
        if is_else_clause(first) {
            if !rest.is_empty() {
                return Err(op_error("cond->if: else clause is not last"));
            }
            let actions = items.get(1..).unwrap_or_default();
            if actions.is_empty() {
                return Err(op_error("cond->if: the else clause has no actions"));
            }
            return Ok(sequence(actions));
        }
        let Some(test) = items.first() else {
            return Err(op_error("cond->if: a clause has no predicate"));
        };
        let actions = items.get(1..).unwrap_or_default();
        let consequent = match actions {
            [] => test.clone(),
            words => sequence(words),
        };
        Ok(Value::list(vec![
            Value::sym("if"),
            test.clone(),
            consequent,
            cond_to_if(rest)?,
        ]))
    }

    /// The book's `let->combination`: the body as a lambda over the
    /// binding names, applied to the binding initializers.
    fn let_to_combination(word: &Value) -> Result<Value, Fault> {
        let items = tagged(word, "let").ok_or_else(|| op_error("let->combination needs a let"))?;
        let bindings = items
            .get(1)
            .and_then(items_of)
            .ok_or_else(|| op_error("let->combination needs bindings"))?;
        let mut names = Vec::new();
        let mut inits = Vec::new();
        for binding in bindings {
            let pair = items_of(&binding)
                .ok_or_else(|| op_error("let->combination: a binding is not a list"))?;
            let name = pair
                .first()
                .cloned()
                .ok_or_else(|| op_error("let->combination: a binding has no name"))?;
            let init = pair
                .get(1)
                .cloned()
                .ok_or_else(|| op_error("let->combination: a binding has no initializer"))?;
            names.push(name);
            inits.push(init);
        }
        let body = items.get(2..).unwrap_or_default();
        if body.is_empty() {
            return Err(op_error("let->combination: the let has an empty body"));
        }
        let lambda = Value::list(
            std::iter::once(Value::sym("lambda"))
                .chain(std::iter::once(Value::list(names)))
                .chain(body.iter().cloned())
                .collect(),
        );
        Ok(Value::list(std::iter::once(lambda).chain(inits).collect()))
    }

    /// The derived-form dispatch tests, spliced ahead of the
    /// application test.
    const DERIVED_TESTS: &str = "
  (test (op cond?) (reg exp))
  (branch (label ev-cond))
  (test (op let?) (reg exp))
  (branch (label ev-let))";

    /// The two transformer entries: each transforms `exp` and
    /// re-enters `eval-dispatch`.
    const EV_DERIVED: &str = "ev-cond
  (assign exp (op cond->if) (reg exp))
  (goto (label eval-dispatch))
ev-let
  (assign exp (op let->combination) (reg exp))
  (goto (label eval-dispatch))";

    /// The syntax tests and the two transformers.
    fn operations() -> Vec<(&'static str, OpHandler)> {
        vec![
            operation("cond?", |args| {
                let [word] = args else {
                    return Err(op_error("cond?: needs one argument"));
                };
                Ok(Value::boolean(tagged(word, "cond").is_some()))
            }),
            operation("let?", |args| {
                let [word] = args else {
                    return Err(op_error("let?: needs one argument"));
                };
                Ok(Value::boolean(tagged(word, "let").is_some()))
            }),
            operation("cond->if", |args| {
                let [word] = args else {
                    return Err(op_error("cond->if: needs one argument"));
                };
                let items =
                    tagged(word, "cond").ok_or_else(|| op_error("cond->if needs a cond"))?;
                cond_to_if(&items[1..])
            }),
            operation("let->combination", |args| {
                let [word] = args else {
                    return Err(op_error("let->combination: needs one argument"));
                };
                let_to_combination(word)
            }),
        ]
    }

    /// The exercise's controller: the base controller with the
    /// derived-form tests spliced into the dispatch and the
    /// transformer entries appended ahead of the error entries.
    fn controller() -> String {
        splice_controller(&base_controller(), DERIVED_TESTS, EV_DERIVED)
    }

    /// Runs one session and answers the last value printed.
    fn value_of(source: &str) -> Result<String, Fault> {
        let mut evaluator = make_evaluator(&controller(), &operations(), source)?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        let at = transcript.len().saturating_sub(2);
        Ok(transcript[at].clone())
    }

    /// The three-clause classify runs, the bodyless clause, and the
    /// `let` whose body multiplies `a` by `b`: the sessions the
    /// answers are pinned from.
    #[test]
    fn ex_5_23() -> Result<(), Fault> {
        let classify = "(define (classify n) (cond ((= n 0) (quote zero)) ((= n 1) (quote one)) (else (quote many))))";
        assert_eq!(value_of(&format!("{classify}\n(classify 0)"))?, "zero");
        assert_eq!(value_of(&format!("{classify}\n(classify 1)"))?, "one");
        assert_eq!(value_of(&format!("{classify}\n(classify 7)"))?, "many");
        assert_eq!(
            value_of("(cond ((= 1 2)))")?,
            display_value(&Value::boolean(false))
        );
        assert_eq!(
            value_of("(cond ((= 1 1)))")?,
            display_value(&Value::boolean(true))
        );
        assert_eq!(value_of("(let ((a 2) (b 3)) (* a b))")?, "6");
        Ok(())
    }
}
