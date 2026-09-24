// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.80.

mod ex_2_80 {
    use ch02::sec_2_4::{imag_part_dispatch, real_part_dispatch};
    use ch02::sec_2_5::{
        apply_generic, contents, install_generic_arithmetic, make_complex_from_mag_ang,
        make_complex_from_real_imag, make_rational, make_real,
    };
    use sicp_runtime::{Handler, Key, OpTable, SchemeError, Value};
    use std::rc::Rc;

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

    fn one_tag(a: &str) -> Key {
        Key::pair(Key::sym(a), Key::Nil)
    }

    /// The exercise's generic `is_zero`, installed for the number
    /// packages of the section.
    pub fn install_zero(table: &OpTable) {
        let ordinary: Handler = Rc::new(|args: &[Value]| {
            let zero = match &args[0] {
                Value::Int(n) => *n == 0,
                Value::Real(x) => *x == 0.0,
                other => {
                    return Err(SchemeError::TypeMismatch(format!(
                        "is_zero: not an ordinary number: {other}"
                    )));
                }
            };
            Ok(Value::boolean(zero))
        });
        table.put(Key::sym("is_zero"), one_tag("scheme-number"), ordinary);
        table.put(
            Key::sym("is_zero"),
            one_tag("rational"),
            Rc::new(|args: &[Value]| {
                let Value::Int(numer) = pair_car(&args[0])? else {
                    return Err(SchemeError::TypeMismatch("rational numer".into()));
                };
                Ok(Value::boolean(numer == 0))
            }),
        );
        table.put(
            Key::sym("is_zero"),
            one_tag("real"),
            Rc::new(|args: &[Value]| Ok(Value::boolean(as_f64(&contents(&args[0])?)? == 0.0))), // real contents are the bare f64 under the tag
        );
        table.put(
            Key::sym("is_zero"),
            one_tag("complex"),
            Rc::new(|args: &[Value]| {
                // The bare contents carry the rectangular or polar tag
                // one level down; the 2.4.2 dispatch selectors read the
                // parts through that second tag.
                let re = as_f64(&real_part_dispatch(&args[0])?)?;
                let im = as_f64(&imag_part_dispatch(&args[0])?)?;
                Ok(Value::boolean(re == 0.0 && im == 0.0))
            }),
        );
    }

    fn is_zero(table: &OpTable, v: &Value) -> Result<bool, SchemeError> {
        match apply_generic(table, "is_zero", std::slice::from_ref(v))? {
            Value::Bool(b) => Ok(b),
            other => Err(SchemeError::TypeMismatch(format!("{other}"))),
        }
    }

    /// Zero questions across the four domains: true where the value is
    /// zero, false where it is not.
    pub fn ex_2_80() -> Result<Vec<bool>, SchemeError> {
        let table = OpTable::new();
        install_generic_arithmetic(&table)?;
        install_zero(&table);
        Ok(vec![
            is_zero(&table, &Value::Int(0))?,
            is_zero(&table, &Value::Int(2))?,
            is_zero(&table, &make_rational(&table, 0, 5)?)?,
            is_zero(&table, &make_rational(&table, 3, 5)?)?,
            is_zero(&table, &make_real(&table, 0.0)?)?,
            is_zero(&table, &make_complex_from_real_imag(&table, 0.0, 0.0)?)?,
            is_zero(&table, &make_complex_from_mag_ang(&table, 0.0, 1.0)?)?,
            is_zero(&table, &make_complex_from_real_imag(&table, 1.0, 0.0)?)?,
        ])
    }
}

#[test]
fn ex_2_80() {
    let answers = ex_2_80::ex_2_80().expect("is_zero answers");
    assert_eq!(
        answers,
        vec![true, false, true, false, true, true, true, false]
    );
}
