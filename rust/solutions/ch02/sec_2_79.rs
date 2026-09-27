// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.79.

mod ex_2_79 {
    use ch02::sec_2_4::{imag_part_dispatch, real_part_dispatch};
    use ch02::sec_2_5::{
        apply_generic, contents, install_generic_arithmetic, make_complex_from_real_imag,
        make_rational,
    };
    use sicp_runtime::{Handler, Key, OpTable, SchemeError, Value};

    #[expect(
        clippy::cast_precision_loss,
        reason = "the exact-to-inexact promotion this comparison is defined by; i128 magnitudes here stay far under f64's mantissa"
    )]
    fn as_f64(v: &Value) -> Result<f64, SchemeError> {
        match v {
            Value::Int(n) => Ok(*n as f64),
            Value::Real(x) => Ok(*x),
            other => Err(SchemeError::TypeMismatch(format!("{other}"))),
        }
    }

    fn pair_car(v: &Value) -> Result<Value, SchemeError> {
        match v {
            Value::Pair(c) => Ok(c.car.borrow().clone()),
            other => Err(SchemeError::TypeMismatch(format!("{other}"))),
        }
    }

    fn pair_cdr(v: &Value) -> Result<Value, SchemeError> {
        match v {
            Value::Pair(c) => Ok(c.cdr.borrow().clone()),
            other => Err(SchemeError::TypeMismatch(format!("{other}"))),
        }
    }

    fn as_int(v: &Value) -> Result<i128, SchemeError> {
        let Value::Int(n) = v else {
            return Err(SchemeError::TypeMismatch("expected an integer".into()));
        };
        Ok(*n)
    }

    fn two_tags(a: &str, b: &str) -> Key {
        Key::pair(Key::sym(a), Key::pair(Key::sym(b), Key::Nil))
    }

    /// The exercise's generic `is_equ`, installed for the four number
    /// packages the section builds.
    #[expect(
        clippy::float_cmp,
        reason = "is_equ on reals and complex parts is exact equality by definition, not an approximate comparison"
    )]
    pub fn install_equ(table: &OpTable) {
        let ordinary: Handler =
            Rc::new(|args: &[Value]| Ok(Value::boolean(as_f64(&args[0])? == as_f64(&args[1])?)));
        table.put(
            Key::sym("is_equ"),
            two_tags("scheme-number", "scheme-number"),
            ordinary,
        );
        let rational: Handler = Rc::new(|args: &[Value]| {
            let numer = |v: &Value| as_int(&pair_car(v)?);
            let denom = |v: &Value| as_int(&pair_cdr(v)?);
            let cross = |a: i128, b: i128| a.checked_mul(b).ok_or(SchemeError::Overflow);
            Ok(Value::boolean(
                cross(numer(&args[0])?, denom(&args[1])?)?
                    == cross(numer(&args[1])?, denom(&args[0])?)?,
            ))
        });
        table.put(
            Key::sym("is_equ"),
            two_tags("rational", "rational"),
            rational,
        );
        table.put(
            Key::sym("is_equ"),
            two_tags("real", "real"),
            Rc::new(|args: &[Value]| {
                Ok(Value::boolean(
                    as_f64(&contents(&args[0])?)? == as_f64(&contents(&args[1])?)?,
                ))
            }),
        );
        // A complex number's parts sit one tag down (rectangular or
        // polar); the 2.4.2 dispatch selectors read them through that
        // second tag.
        table.put(
            Key::sym("is_equ"),
            two_tags("complex", "complex"),
            Rc::new(|args: &[Value]| {
                let parts = |z: &Value| -> Result<(f64, f64), SchemeError> {
                    Ok((
                        as_f64(&real_part_dispatch(z)?)?,
                        as_f64(&imag_part_dispatch(z)?)?,
                    ))
                };
                let z1 = parts(&args[0])?;
                let z2 = parts(&args[1])?;
                Ok(Value::boolean(z1 == z2))
            }),
        );
    }

    use std::rc::Rc;

    /// Equality questions across the three domains the exercise names.
    pub fn ex_2_79() -> Result<Vec<bool>, SchemeError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        install_equ(&table);
        let equ = |a: &Value, b: &Value| -> Result<bool, SchemeError> {
            match apply_generic(&table, "is_equ", &[a.clone(), b.clone()])? {
                Value::Bool(b) => Ok(b),
                other => Err(SchemeError::TypeMismatch(format!("{other}"))),
            }
        };
        Ok(vec![
            equ(&Value::Int(6), &Value::Int(6))?,
            equ(&Value::Int(6), &Value::Int(7))?,
            equ(&make_rational(&table, 1, 2)?, &make_rational(&table, 2, 4)?)?,
            equ(&make_rational(&table, 1, 2)?, &make_rational(&table, 1, 3)?)?,
            equ(
                &make_complex_from_real_imag(&table, 1.0, 2.0)?,
                &make_complex_from_real_imag(&table, 1.0, 2.0)?,
            )?,
            equ(
                &make_complex_from_real_imag(&table, 1.0, 2.0)?,
                &make_complex_from_real_imag(&table, 1.0, 3.0)?,
            )?,
        ])
    }
}

#[test]
fn ex_2_79() {
    let answers = ex_2_79::ex_2_79().expect("is_equ answers");
    assert_eq!(
        answers,
        vec![
            true,  // 6 = 6
            false, // 6 != 7
            true,  // 1/2 = 2/4
            false, // 1/2 != 1/3
            true,  // 1 + 2i = 1 + 2i
            false, // 1 + 2i != 1 + 3i
        ]
    );
}
