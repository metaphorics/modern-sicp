// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.8: named let..

use ch04::eval_support::*;

mod ex_4_08 {
    use super::*;

    /// The evaluator whose `let` also accepts the named form.
    pub struct WithNamedLet;

    /// The named-let rewrite: the name is bound to the loop procedure
    /// in one frame, and the initializers run in that frame's outer
    /// environment.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on a malformed form.
    pub fn named_let_to_combination(exp: &Value) -> EvalResult {
        let items = exp.list_items()?;
        let name = items
            .get(1)
            .and_then(|v| match v {
                Value::Sym(name) => Some(name.clone()),
                _ => None,
            })
            .ok_or_else(|| SchemeError::TypeMismatch(format!("malformed named let: {exp}")))?;
        let bindings = items
            .get(2)
            .cloned()
            .ok_or_else(|| SchemeError::TypeMismatch(format!("malformed named let: {exp}")))?;
        let body: Vec<Value> = items.into_iter().skip(3).collect();
        let mut params: Vec<Symbol> = Vec::new();
        let mut inits = Vec::new();
        for binding in bindings.list_items()? {
            let pair = binding.list_items()?;
            let (Some(name), Some(init)) = (pair.first(), pair.get(1)) else {
                return Err(SchemeError::TypeMismatch(format!(
                    "malformed named let binding: {binding}"
                )));
            };
            let Value::Sym(param) = name else {
                return Err(SchemeError::TypeMismatch(format!(
                    "not a named let parameter: {name}"
                )));
            };
            params.push(Rc::clone(param));
            inits.push(init.clone());
        }
        // ((lambda (name)
        //    (set! name (lambda (param ...) body ...))
        //    (name init ...))
        //  '*ok*)
        // The loop procedure is built inside the wrapper's frame, so
        // its recursive self-call resolves through that frame.
        let loop_procedure = make_lambda(&params, None, &body);
        let set_form = Value::list(vec![
            Value::sym("set!"),
            Value::Sym(Rc::clone(&name)),
            loop_procedure,
        ]);
        let call = Value::list({
            let mut c = vec![Value::Sym(Rc::clone(&name))];
            c.extend(inits);
            c
        });
        let wrapper_body = Value::list(vec![Value::sym("begin"), set_form, call]);
        let wrapper = make_lambda(&[name], None, &[wrapper_body]);
        let seed = Value::list(vec![Value::sym("quote"), Value::sym("*ok*")]);
        Ok(Value::Pair(cons_cell(wrapper, Value::list(vec![seed]))))
    }

    impl Evaluator for WithNamedLet {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            if is_let(exp) {
                let named = matches!(exp.list_items()?.get(1), Some(Value::Sym(_)));
                let combination = if named {
                    named_let_to_combination(exp)?
                } else {
                    let_to_combination(exp)?
                };
                return Ok(Step::Tail(combination, Rc::clone(env)));
            }
            self.base_step(exp, env)
        }
    }

    /// Evaluates the book's named-let Fibonacci at 10.
    pub fn fib_named_let() -> Result<String, SchemeError> {
        let program = "(define (fib n)\n  (let fib-iter ((a 1) (b 0) (count n))\n    (if (= count 0)\n        b\n        (fib-iter (+ a b) a (- count 1)))))\n(fib 10)";
        let (values, _) = run_with(&WithNamedLet, program)?;
        Ok(printed(&values).last().cloned().unwrap_or_default())
    }
}

#[test]
fn ex_4_08() {
    assert_eq!(ex_4_08::fib_named_let().expect("runs"), "55");
}
