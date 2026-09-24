// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.83.

mod ex_2_83 {
    use ch02::sec_2_4::rect_make_from_real_imag_tagged;
    use ch02::sec_2_5::{apply_generic, install_generic_arithmetic};
    use sicp_runtime::{Key, OpTable, SchemeError, Value};
    use std::rc::Rc;

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

    /// The raise operations of the tower integer -> rational -> real ->
    /// complex, each installed one level up from its own type.
    pub fn install_raise(table: &OpTable) {
        // An exact integer rises into a rational with denominator 1.
        table.put(
            Key::sym("raise"),
            one_tag("scheme-number"),
            Rc::new(|args: &[Value]| {
                match &args[0] {
                    Value::Int(n) => Ok(Value::tagged(
                        "rational",
                        Value::Pair(sicp_runtime::cons_cell(Value::Int(*n), Value::Int(1))),
                    )),
                    // An inexact ordinary number is already a real value;
                    // its raise puts on the tower's real tag.
                    Value::Real(_) => Ok(Value::tagged("real", args[0].clone())),
                    other => Err(SchemeError::TypeMismatch(format!("{other}"))),
                }
            }),
        );
        // A rational rises to the real nearest it.
        table.put(
            Key::sym("raise"),
            one_tag("rational"),
            Rc::new(|args: &[Value]| {
                // The handler sees the bare numerator/denominator pair.
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
        // A real rises into the complex plane with zero imaginary part.
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

    /// The generic `raise`: one level up, or an error at the top.
    pub fn raise(table: &OpTable, v: &Value) -> Result<Value, SchemeError> {
        apply_generic(table, "raise", std::slice::from_ref(v))
    }

    /// Raising an ordinary number through the whole tower, and the
    /// refusal at the top.
    pub fn ex_2_83() -> Result<(String, String, String), SchemeError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        install_raise(&table);

        let level1 = raise(&table, &Value::Int(5))?;
        let level2 = raise(&table, &level1)?;
        let level3 = raise(&table, &level2)?;
        // The top of the tower has no raise of its own.
        if raise(&table, &level3).is_ok() {
            return Err(SchemeError::TypeMismatch(
                "complex must have no raise".into(),
            ));
        }
        Ok((level1.to_string(), level2.to_string(), level3.to_string()))
    }
}

#[test]
fn ex_2_83() {
    let (r1, r2, r3) = ex_2_83::ex_2_83().expect("raise chain");
    assert_eq!(r1, "(rational (5 . 1))");
    assert_eq!(r2, "(real 5)");
    assert_eq!(r3, "(complex (rectangular (5 . 0)))");
}
