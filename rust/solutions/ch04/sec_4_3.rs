// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.3: data-directed dispatch in eval; the table is the
// crate's put/get registry keyed by (operation, tag) with the clause as
// the element..

use ch04::eval_support::*;

mod ex_4_03 {
    use super::*;

    /// One dispatch clause: the expression, its environment, and the
    /// dispatcher to recurse through.
    pub type Clause = Rc<dyn Fn(&Value, &Rc<Env>, &dyn Evaluator) -> StepResult>;

    /// Wraps one clause closure so every install has the table's type.
    fn clause(f: impl Fn(&Value, &Rc<Env>, &dyn Evaluator) -> StepResult + 'static) -> Clause {
        Rc::new(f)
    }

    /// The data-directed evaluator: `eval` is a table lookup keyed by
    /// the operation `eval` and the expression's head symbol.
    pub struct TableDriven {
        table: OpTable<Clause>,
    }

    impl Default for TableDriven {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TableDriven {
        /// Installs the section's clauses into the table, clause by
        /// clause, the way 2.73's differentiation table is filled.
        #[must_use]
        pub fn new() -> Self {
            let table = OpTable::new();
            let eval = Key::sym("eval");
            table.put(
                eval.clone(),
                Key::sym("quote"),
                clause(|exp, _env, _ev| Ok(Step::Done(text_of_quotation(exp)?))),
            );
            table.put(
                eval.clone(),
                Key::sym("if"),
                clause(|exp, env, ev: &dyn Evaluator| {
                    let (predicate, consequent, alternative) = ch04::sec_4_1::if_parts(exp)?;
                    let tested = ev.eval(&predicate, env)?;
                    Ok(Step::Tail(
                        if ch04::sec_4_1::is_true(&tested) {
                            consequent
                        } else {
                            alternative
                        },
                        Rc::clone(env),
                    ))
                }),
            );
            table.put(
                eval.clone(),
                Key::sym("define"),
                clause(|exp, env, ev| Ok(Step::Done(ev.eval_definition(exp, env)?))),
            );
            table.put(
                eval.clone(),
                Key::sym("set!"),
                clause(|exp, env, ev| Ok(Step::Done(ev.eval_assignment(exp, env)?))),
            );
            table.put(
                eval.clone(),
                Key::sym("lambda"),
                clause(|exp, env, _ev| {
                    let (params, rest) = ch04::sec_4_1::lambda_parameters(exp)?;
                    let body = ch04::sec_4_1::lambda_body(exp)?;
                    Ok(Step::Done(Value::Closure(Rc::new(Closure {
                        name: None,
                        params,
                        rest,
                        body,
                        env: Rc::clone(env),
                    }))))
                }),
            );
            table.put(
                eval.clone(),
                Key::sym("begin"),
                clause(|exp, env, ev| ev.step_sequence(&operand_items(exp)?, env)),
            );
            table.put(
                eval.clone(),
                Key::sym("cond"),
                clause(|exp, env, _ev| Ok(Step::Tail(cond_to_if(exp)?, Rc::clone(env)))),
            );
            table.put(
                eval,
                Key::sym("let"),
                clause(|exp, env, _ev| Ok(Step::Tail(let_to_combination(exp)?, Rc::clone(env)))),
            );
            Self { table }
        }

        /// The book's `put`: one new expression type, installed while
        /// the program runs, no edit to `step` anywhere.
        pub fn install(&self, tag: &str, clause: Clause) {
            self.table.put(Key::sym("eval"), Key::sym(tag), clause);
        }
    }

    impl Evaluator for TableDriven {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if ch04::sec_4_1::is_self_evaluating(exp) {
                return Ok(Step::Done(exp.clone()));
            }
            if is_variable(exp) {
                let Value::Sym(name) = exp else {
                    return Err(SchemeError::TypeMismatch("not a variable".to_owned()));
                };
                return Ok(Step::Done(lookup_variable_value(name, env)?));
            }
            let head = ch04::sec_4_1::first_of(exp)?;
            let clause = match &head {
                Value::Sym(head) => self
                    .table
                    .get(&Key::sym("eval"), &Key::Sym(Rc::clone(head))),
                _ => None,
            };
            if let Some(clause) = clause {
                return clause(exp, env, self);
            }
            // The leftover clause: any pair is an application.
            let operands = operand_items(exp)?;
            let proc = self.eval(&head, env)?;
            let args = self.list_of_values(&operands, env)?;
            self.tail_apply(&proc, &args)
        }
    }

    /// Answers the values of a quotation, an `if`, and a defined call
    /// through the table, then installs a brand-new form at runtime and
    /// evaluates it.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let evaluator = TableDriven::new();
        let program = "(quote (a b))\n(if (= 1 1) 42 0)\n(define (sq x) (* x x))\n(sq 7)";
        let (values, _) = run_with(&evaluator, program)?;
        let mut answers = printed(&values);
        // A form type the table never saw installs while the program
        // runs, with no edit to `step` anywhere.
        evaluator.install(
            "unless",
            Rc::new(|exp, env, ev| {
                let items = exp.list_items()?;
                let condition = items.get(1).cloned().unwrap_or(Value::Nil);
                let tested = ev.eval(&condition, env)?;
                if ch04::sec_4_1::is_true(&tested) {
                    return Ok(Step::Done(Value::boolean(false)));
                }
                let body: Vec<Value> = items.into_iter().skip(2).collect();
                ev.step_sequence(&body, env)
            }),
        );
        let (installed, _) = run_with(&evaluator, "(unless (= 1 2) 'ran)")?;
        answers.extend(printed(&installed));
        Ok(answers)
    }
}

#[test]
fn ex_4_03() {
    let values = ex_4_03::answers().expect("runs");
    assert_eq!(
        values,
        vec!["(a b)", "42", "ok", "49", "ran"],
        "table dispatch, application fallback, and a runtime-installed form"
    );
}
