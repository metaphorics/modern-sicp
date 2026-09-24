// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.84.

mod ex_2_84 {
    use ch02::sec_2_4::rect_make_from_real_imag_tagged;
    use ch02::sec_2_5::{
        add, contents, install_generic_arithmetic, make_complex_from_real_imag, make_rational,
        type_tag,
    };
    use sicp_runtime::{Key, OpTable, SchemeError, Value};
    use std::rc::Rc;

    fn tag_list_key(args: &[Value]) -> Result<Key, SchemeError> {
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
    fn as_f64(v: &Value) -> Result<f64, SchemeError> {
        match v {
            Value::Int(n) => Ok(*n as f64),
            Value::Real(x) => Ok(*x),
            other => Err(SchemeError::TypeMismatch(format!("{other}"))),
        }
    }

    /// The raise operations of exercise 2.83, on one table with the
    /// arithmetic packages.
    pub fn install_raise(table: &OpTable) {
        table.put(
            Key::sym("raise"),
            one_tag("scheme-number"),
            Rc::new(|args: &[Value]| match &args[0] {
                Value::Int(n) => Ok(Value::tagged(
                    "rational",
                    Value::Pair(sicp_runtime::cons_cell(Value::Int(*n), Value::Int(1))),
                )),
                Value::Real(_) => Ok(Value::tagged("real", args[0].clone())),
                other => Err(SchemeError::TypeMismatch(format!("{other}"))),
            }),
        );
        table.put(
            Key::sym("raise"),
            one_tag("rational"),
            Rc::new(|args: &[Value]| {
                let Value::Pair(cell) = &args[0] else {
                    return Err(SchemeError::TypeMismatch("rational".into()));
                };
                let (Value::Int(n), Value::Int(d)) =
                    (cell.car.borrow().clone(), cell.cdr.borrow().clone())
                else {
                    return Err(SchemeError::TypeMismatch("rational".into()));
                };
                if d == 0 {
                    return Err(SchemeError::DivisionByZero);
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
    }

    /// Raises `v` one level, answering `None` at the top of the tower:
    /// the "compatible with the rest of the system" test for height, per
    /// the exercise — no global ranking table, just the raise entries.
    fn raise_step(table: &OpTable, v: &Value) -> Option<Value> {
        let key = one_tag(type_tag(v).ok()?.as_ref());
        let proc = table.get(&Key::sym("raise"), &key)?;
        // The handler works on the bare contents, as apply_generic
        // would hand them over.
        let bare = contents(v).ok()?;
        proc(std::slice::from_ref(&bare)).ok()
    }

    /// The exercise's `apply_generic`: on a miss, raise the arguments
    /// successively until the tags agree or the tower runs out.
    fn apply_generic_raise(
        table: &OpTable,
        op: &str,
        args: &[Value],
    ) -> Result<Value, SchemeError> {
        if let Some(proc) = table.get(&Key::sym(op), &tag_list_key(args)?) {
            let bare: Result<Vec<Value>, SchemeError> = args.iter().map(contents).collect();
            return proc(&bare?);
        }
        if let [a1, a2] = args {
            let mut raised: Vec<Value> = vec![a1.clone(), a2.clone()];
            for _ in 0..8 {
                let t1 = type_tag(&raised[0])?;
                let t2 = type_tag(&raised[1])?;
                if t1 == t2 {
                    break;
                }
                // Lift whichever argument still has a raise entry; a
                // stuck argument has reached the top.
                match raise_step(table, &raised[0]) {
                    Some(next) => raised[0] = next,
                    None => match raise_step(table, &raised[1]) {
                        Some(next) => raised[1] = next,
                        None => break,
                    },
                }
            }
            if tag_list_key(&raised)? == tag_list_key(args)?
                || type_tag(&raised[0])? != type_tag(&raised[1])?
            {
                return Err(SchemeError::UserRaised {
                    message: "No method for these types".into(),
                    irritants: vec![Value::sym(op)],
                });
            }
            return apply_generic_raise(table, op, &raised);
        }
        Err(SchemeError::UserRaised {
            message: "No method for these types".into(),
            irritants: vec![Value::sym(op)],
        })
    }

    /// Mixed arithmetic: integer plus complex, rational plus complex,
    /// and a rational raised against a real.
    pub fn ex_2_84() -> Result<(String, String), SchemeError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        install_raise(&table);

        let int_plus_complex = apply_generic_raise(
            &table,
            "add",
            &[
                Value::Int(3),
                make_complex_from_real_imag(&table, 1.0, 2.0)?,
            ],
        )?;
        let rational_plus_complex = apply_generic_raise(
            &table,
            "add",
            &[
                make_rational(&table, 1, 2)?,
                make_complex_from_real_imag(&table, 0.5, 1.0)?,
            ],
        )?;
        // Plain generic add still misses: the raise-aware dispatcher is
        // the one that coerces.
        let plain = add(
            &table,
            &Value::Int(3),
            &make_complex_from_real_imag(&table, 0.0, 0.0)?,
        );
        assert!(plain.is_err());

        Ok((
            int_plus_complex.to_string(),
            rational_plus_complex.to_string(),
        ))
    }
}

#[test]
fn ex_2_84() {
    let (a, b) = ex_2_84::ex_2_84().expect("raise dispatch");
    assert_eq!(a, "(complex (rectangular (4 . 2)))");
    assert_eq!(b, "(complex (rectangular (1 . 1)))");
}
