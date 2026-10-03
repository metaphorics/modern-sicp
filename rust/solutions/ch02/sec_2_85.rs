// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.85.

mod ex_2_85 {
    use ch02::sec_2_4::{real_part_dispatch, rect_make_from_real_imag_tagged};
    use ch02::sec_2_5::{
        contents, install_generic_arithmetic, is_equ, make_complex_from_real_imag, type_tag,
    };
    use sicp_runtime::{Key, OpTable, SicpError, Value};
    use std::rc::Rc;

    fn tag_list_key(args: &[Value]) -> Result<Key, SicpError> {
        let mut key = Key::Nil;
        for a in args.iter().rev() {
            key = Key::pair(Key::Sym(type_tag(a)?), key);
        }
        Ok(key)
    }

    fn one_tag(a: &str) -> Key {
        Key::pair(Key::sym(a), Key::Nil)
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "the exact-to-inexact promotion `raise` performs; i128 magnitudes here stay far under f64's mantissa"
    )]
    fn as_f64(v: &Value) -> Result<f64, SicpError> {
        match v {
            Value::Int(n) => Ok(*n as f64),
            Value::Real(x) => Ok(*x),
            other => Err(SicpError::TypeMismatch(format!("{other}"))),
        }
    }

    /// The raise entries of exercises 2.83 and 2.84, plus the projections:
    /// complex to real drops the imaginary part, real to integer rounds
    /// (the footnote's `round`), and a rational with denominator 1 drops
    /// to an ordinary integer.
    pub fn install_raise_and_project(table: &OpTable) {
        table.put(
            Key::sym("raise"),
            one_tag("integer"),
            Rc::new(|args: &[Value]| match &args[0] {
                Value::Int(n) => Ok(Value::tagged(
                    "rational",
                    Value::Pair(sicp_runtime::cons_cell(Value::Int(*n), Value::Int(1))),
                )),
                other => Err(SicpError::TypeMismatch(format!("{other}"))),
            }),
        );
        table.put(
            Key::sym("raise"),
            one_tag("rational"),
            Rc::new(|args: &[Value]| {
                let Value::Pair(cell) = &args[0] else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                let (Value::Int(n), Value::Int(d)) =
                    (cell.car.borrow().clone(), cell.cdr.borrow().clone())
                else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                if d == 0 {
                    return Err(SicpError::DivisionByZero);
                }
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "the exact-to-inexact promotion raising a rational to a real performs"
                )]
                let value = n as f64 / d as f64;
                Ok(Value::tagged("real", Value::Real(value)))
            }),
        );
        table.put(
            Key::sym("raise"),
            one_tag("real"),
            Rc::new(|args: &[Value]| {
                let x = as_f64(&args[0])?;
                Ok(Value::tagged(
                    "complex",
                    rect_make_from_real_imag_tagged(x, 0.0),
                ))
            }),
        );
        table.put(
            Key::sym("project"),
            one_tag("complex"),
            Rc::new(|args: &[Value]| {
                let re = as_f64(&real_part_dispatch(&args[0])?)?;
                Ok(Value::tagged("real", Value::Real(re)))
            }),
        );
        table.put(
            Key::sym("project"),
            one_tag("real"),
            Rc::new(|args: &[Value]| {
                // round: the footnote's projection; NaN saturates to 0
                // and the equ? check below rejects any such guess.
                let Value::Real(x) = &args[0] else {
                    return Err(SicpError::TypeMismatch("project: real".into()));
                };
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "round() already produced the nearest integral value; this is the footnote's own projection"
                )]
                let n = x.round() as i128;
                Ok(Value::Int(n))
            }),
        );
        table.put(
            Key::sym("project"),
            one_tag("rational"),
            Rc::new(|args: &[Value]| {
                let Value::Pair(cell) = &args[0] else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                let (Value::Int(n), Value::Int(d)) =
                    (cell.car.borrow().clone(), cell.cdr.borrow().clone())
                else {
                    return Err(SicpError::TypeMismatch("rational".into()));
                };
                if d != 1 {
                    return Err(SicpError::TypeMismatch(
                        "a fraction does not project to an integer".into(),
                    ));
                }
                Ok(Value::Int(n))
            }),
        );
    }

    fn generic(table: &OpTable, op: &str, v: &Value) -> Option<Value> {
        let key = one_tag(type_tag(v).ok()?.as_ref());
        let proc = table.get(&Key::sym(op), &key)?;
        let bare = contents(v).ok()?;
        proc(std::slice::from_ref(&bare)).ok()
    }

    /// The exercise's `drop`: project, raise back, and keep the lower
    /// form only when the round trip answers something equal.
    pub fn drop_value(table: &OpTable, v: &Value) -> Result<Value, SicpError> {
        let mut current = v.clone();
        for _ in 0..8 {
            let Some(projected) = generic(table, "project", &current) else {
                return Ok(current);
            };
            // Raise the projection back through the tower until it sits
            // at the type we started from.
            let mut raised = projected.clone();
            let start_tag = type_tag(&current)?;
            for _ in 0..8 {
                if type_tag(&raised)? == start_tag {
                    break;
                }
                match generic(table, "raise", &raised) {
                    Some(next) => raised = next,
                    None => break,
                }
            }
            if !is_equ(table, &current, &raised)? {
                return Ok(current);
            }
            current = projected;
        }
        Ok(current)
    }

    /// The rewritten `apply_generic`: dispatch as before, then simplify
    /// arithmetic answers by dropping.
    fn apply_generic_drop(table: &OpTable, op: &str, args: &[Value]) -> Result<Value, SicpError> {
        if let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) {
            let bare: Result<Vec<Value>, SicpError> = args.iter().map(contents).collect();
            let result = proc(&bare?)?;
            return match op {
                "add" | "sub" | "mul" | "div" => drop_value(table, &result),
                _ => Ok(result),
            };
        }
        Err(SicpError::UserRaised {
            message: "No method for these types".into(),
            irritants: vec![Value::sym(op)],
        })
    }

    /// The book's examples: (2 + 3i) + (4 - 3i) lands as the integer 6,
    /// 1.5 + 0i lowers only as far as the real 1.5, 1 + 0i reaches the
    /// integer 1, and 2 + 3i cannot be lowered at all.
    pub fn ex_2_85() -> Result<(String, String, String, String), SicpError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        install_raise_and_project(&table);

        let complex = |re: f64, im: f64| make_complex_from_real_imag(&table, re, im);
        let six = apply_generic_drop(&table, "add", &[complex(2.0, 3.0)?, complex(4.0, -3.0)?])?;
        let one_and_a_half = drop_value(&table, &complex(1.5, 0.0)?)?;
        let one = drop_value(&table, &complex(1.0, 0.0)?)?;
        let two_plus_three_i = drop_value(&table, &complex(2.0, 3.0)?)?;

        Ok((
            six.to_string(),
            one_and_a_half.to_string(),
            one.to_string(),
            two_plus_three_i.to_string(),
        ))
    }
}

#[test]
fn ex_2_85() {
    let (six, one_and_a_half, one, complex) = ex_2_85::ex_2_85().expect("drop");
    assert_eq!(six, "6");
    assert_eq!(one_and_a_half, "(real 1.5)");
    assert_eq!(one, "1");
    assert_eq!(complex, "(complex (rectangular (2 . 3)))");
}
