// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.10: new syntax, unchanged eval -- a reader-level transform..

use ch04::eval_support::*;

mod ex_4_10 {
    use super::*;

    /// Rewrites the new surface syntax into the section's: `defun`
    /// spells a definition, `fun` spells a lambda. `eval` and `apply`
    /// never learn about it.
    #[must_use]
    pub fn syntaxize(exp: &Value) -> Value {
        match exp {
            Value::Pair(cell) => {
                let car = syntaxize(&cell.car.borrow());
                let cdr = syntaxize(&cell.cdr.borrow());
                Value::Pair(cons_cell(car, cdr))
            }
            Value::Sym(s) => match &**s {
                "defun" => Value::sym("define"),
                "fun" => Value::sym("lambda"),
                _ => exp.clone(),
            },
            other => other.clone(),
        }
    }

    /// Evaluates a `defun`-spelled program through the transform and
    /// answers its value, then the error the same program raises
    /// without the transform.
    pub fn answers() -> Result<(String, String), SchemeError> {
        let env = setup_environment();
        let program = "(defun (sq x) (* x x))\n(sq 7)";
        let forms = read_program(program)?;
        let mut value = Value::Nil;
        for form in &forms {
            value = Base.eval(&syntaxize(form), &env)?;
        }
        let without = Base
            .eval(&forms[0].clone(), &env)
            .expect_err("defun is no form");
        Ok((print_value(&value), without.to_string()))
    }
}

#[test]
fn ex_4_10() {
    let (with_transform, without) = ex_4_10::answers().expect("runs");
    assert_eq!(with_transform, "49");
    // Unchanged eval sees an application of an unbound operator.
    assert!(without.contains("defun"));
}
