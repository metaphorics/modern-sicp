// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.14: the host `map` as a primitive fails. A bare host
// handler can only call host handlers; an object-language procedure is
// a `Closure` value, and the only way to apply it is back through the
// evaluator's apply..

use ch04::eval_support::*;

mod ex_4_14 {
    use super::*;

    /// Louis's map: a plain primitive that calls its procedure argument
    /// as a handler, the way a host map calls a host function.
    fn louis_map() -> Value {
        Value::Primitive {
            name: Rc::from("louis-map"),
            f: Rc::new(|args| {
                let [f, list, ..] = args else {
                    return Err(SchemeError::WrongArity {
                        procedure: "louis-map".to_owned(),
                        expected: "2".to_owned(),
                        got: args.len(),
                    });
                };
                let items = list.list_items()?;
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(f.call(&[item])?);
                }
                Ok(Value::list(out))
            }),
        }
    }

    /// Eva's map: an object-language procedure, applied by the
    /// evaluator itself.
    fn eva_program() -> &'static str {
        "(define (eva-map f xs)\n  (if (null? xs)\n      '()\n      (cons (f (car xs)) (eva-map f (cdr xs)))))\n(define (sq x) (* x x))\n"
    }

    /// Answers the error Louis's map raises on a compound procedure and
    /// the list Eva's map produces for the same data.
    pub fn answers() -> Result<(String, SchemeError), SchemeError> {
        let env = setup_environment();
        env.define(Rc::from("louis-map"), louis_map());
        let forms = read_program(&format!("{}(louis-map sq '(1 2 3))", eva_program()))?;
        for form in &forms[..forms.len() - 1] {
            Base.eval(form, &env)?;
        }
        let louis = Base
            .eval(&forms[forms.len() - 1], &env)
            .expect_err("the host map fails");
        let (values, _) = run_with(&Base, &format!("{}(eva-map sq '(1 2 3))", eva_program()))?;
        Ok((printed(&values).last().cloned().unwrap_or_default(), louis))
    }
}

#[test]
fn ex_4_14() {
    let (eva, louis) = ex_4_14::answers().expect("runs");
    // Eva's object-language map applies procedures through the
    // evaluator, so a compound procedure is just another value.
    assert_eq!(eva, "(1 4 9)");
    // Louis's primitive map calls its procedure argument as a bare
    // host handler, and a compound procedure is not one.
    assert!(louis.to_string().contains("not a procedure"), "{louis}");
}
