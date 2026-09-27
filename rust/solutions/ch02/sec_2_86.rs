// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.86.

mod ex_2_86 {
    use ch02::sec_2_5::{apply_generic, install_rational_package, make_rational};
    use sicp_runtime::{Key, OpTable, SchemeError, Value};
    use std::rc::Rc;

    fn one_tag(a: &str) -> Key {
        Key::pair(Key::sym(a), Key::Nil)
    }

    fn two_tags(a: &str, b: &str) -> Key {
        Key::pair(Key::sym(a), Key::pair(Key::sym(b), Key::Nil))
    }

    /// Converts any generic coefficient the table knows to `f64`, the way
    /// the trigonometric helpers below need a plain number to call `sqrt`
    /// and `atan2`. This is the "numbers, rationals, reals" leg of the
    /// exercise's "whole tower" requirement; polynomial coefficients stay
    /// out of scope for magnitude and angle, which the book itself never
    /// asks to generalize that far.
    #[expect(
        clippy::cast_precision_loss,
        reason = "promoting an exact integer or rational into a real for sqrt/atan2 is the operation's point"
    )]
    fn numeric_value(v: &Value) -> Result<f64, SchemeError> {
        match v {
            Value::Int(n) => Ok(*n as f64),
            Value::Real(x) => Ok(*x),
            // A bare rational pair: (numer . denom).
            Value::Pair(cell) => {
                let (Value::Int(n), Value::Int(d)) =
                    (cell.car.borrow().clone(), cell.cdr.borrow().clone())
                else {
                    return Err(SchemeError::TypeMismatch("rational pair".into()));
                };
                Ok(n as f64 / d as f64)
            }
            // A tagged rational or real, the shape `add`/`mul` hand back.
            Value::Tagged { tag, data } if tag.as_ref() == "rational" || tag.as_ref() == "real" => {
                numeric_value(data)
            }
            other => Err(SchemeError::TypeMismatch(format!(
                "not a generic number: {other}"
            ))),
        }
    }

    fn car(p: &Value) -> Result<Value, SchemeError> {
        match p {
            Value::Pair(cell) => Ok(cell.car.borrow().clone()),
            other => Err(SchemeError::TypeMismatch(format!("not a pair: {other}"))),
        }
    }

    fn cdr(p: &Value) -> Result<Value, SchemeError> {
        match p {
            Value::Pair(cell) => Ok(cell.cdr.borrow().clone()),
            other => Err(SchemeError::TypeMismatch(format!("not a pair: {other}"))),
        }
    }

    fn real_part(table: &OpTable, z: &Value) -> Result<Value, SchemeError> {
        apply_generic(table, "real_part", std::slice::from_ref(z))
    }

    fn imag_part(table: &OpTable, z: &Value) -> Result<Value, SchemeError> {
        apply_generic(table, "imag_part", std::slice::from_ref(z))
    }

    fn magnitude(table: &OpTable, z: &Value) -> Result<Value, SchemeError> {
        apply_generic(table, "magnitude", std::slice::from_ref(z))
    }

    fn generic_add(table: &OpTable, a: &Value, b: &Value) -> Result<Value, SchemeError> {
        ch02::sec_2_5::add(table, a, b)
    }

    fn generic_mul(table: &OpTable, a: &Value, b: &Value) -> Result<Value, SchemeError> {
        ch02::sec_2_5::mul(table, a, b)
    }

    /// Installs the rectangular package rebuilt over generic coefficients:
    /// `real_part`/`imag_part` are the pair's own generic parts,
    /// `magnitude` folds them through the generic tower into a plain
    /// real, matching the book's "handle numbers, rationals, reals, and
    /// complex numbers" requirement for the two parts of a complex
    /// number.
    fn install_generic_rectangular(table: &Rc<OpTable>) {
        let key = one_tag("rectangular");
        table.put(
            Key::sym("real_part"),
            key.clone(),
            Rc::new(|args: &[Value]| car(&args[0])),
        );
        table.put(
            Key::sym("imag_part"),
            key.clone(),
            Rc::new(|args: &[Value]| cdr(&args[0])),
        );
        let t = Rc::clone(table);
        table.put(
            Key::sym("magnitude"),
            key,
            Rc::new(move |args: &[Value]| {
                let re = car(&args[0])?;
                let im = cdr(&args[0])?;
                let sum_sq =
                    generic_add(&t, &generic_mul(&t, &re, &re)?, &generic_mul(&t, &im, &im)?)?;
                Ok(Value::tagged(
                    "real",
                    Value::Real(numeric_value(&sum_sq)?.sqrt()),
                ))
            }),
        );
        table.put(
            Key::sym("make-from-real-imag"),
            Key::sym("rectangular"),
            Rc::new(|args: &[Value]| {
                Ok(Value::tagged(
                    "rectangular",
                    Value::Pair(sicp_runtime::cons_cell(args[0].clone(), args[1].clone())),
                ))
            }),
        );
    }

    fn make_from_real_imag(table: &OpTable, re: Value, im: Value) -> Result<Value, SchemeError> {
        let make = table
            .get(&Key::sym("make-from-real-imag"), &Key::sym("rectangular"))
            .ok_or_else(|| SchemeError::TypeMismatch("no rectangular constructor".into()))?;
        let rect = make(&[re, im])?;
        Ok(Value::tagged("complex", rect))
    }

    /// Installs the complex package over the generic rectangular parts:
    /// the "complex" tag forwards each selector into the representation
    /// package underneath, and `add` combines real and imaginary generic
    /// parts directly, exactly as the un-generalized 2.4.3 package did,
    /// but every `+` is now the table's `add`.
    fn install_generic_complex(table: &Rc<OpTable>) {
        install_generic_rectangular(table);
        for name in ["real_part", "imag_part", "magnitude"] {
            let t = Rc::clone(table);
            table.put(
                Key::sym(name),
                one_tag("complex"),
                Rc::new(move |args: &[Value]| apply_generic(&t, name, args)),
            );
        }
        let t = Rc::clone(table);
        table.put(
            Key::sym("add"),
            two_tags("complex", "complex"),
            Rc::new(move |args: &[Value]| {
                let re = generic_add(&t, &real_part(&t, &args[0])?, &real_part(&t, &args[1])?)?;
                let im = generic_add(&t, &imag_part(&t, &args[0])?, &imag_part(&t, &args[1])?)?;
                make_from_real_imag(&t, re, im)
            }),
        );
        let t = Rc::clone(table);
        table.put(
            Key::sym("make-from-real-imag"),
            Key::sym("complex"),
            Rc::new(move |args: &[Value]| {
                make_from_real_imag(&t, args[0].clone(), args[1].clone())
            }),
        );
    }

    /// A complex number whose real and imaginary parts are rationals: `3`
    /// and `4`, giving magnitude 5 through the generic tower rather than a
    /// hardwired `f64` pair, and `add` on two such numbers doubling both
    /// generic parts.
    pub fn ex_2_86() -> Result<(String, String, String), SchemeError> {
        let table = Rc::new(OpTable::new());
        install_rational_package(&table);
        install_generic_complex(&table);

        let three = make_rational(&table, 3, 1)?;
        let four = make_rational(&table, 4, 1)?;
        let z = make_from_real_imag(&table, three, four)?;

        let re = real_part(&table, &z)?;
        let mag = magnitude(&table, &z)?;
        let doubled = generic_add(&table, &z, &z)?;

        Ok((re.to_string(), mag.to_string(), doubled.to_string()))
    }
}

#[test]
fn ex_2_86() {
    let (re, mag, doubled) = ex_2_86::ex_2_86().expect("generic complex parts");
    assert_eq!(re, "(rational (3 . 1))");
    assert_eq!(mag, "(real 5)");
    assert_eq!(
        doubled,
        "(complex (rectangular ((rational (6 . 1)) . (rational (8 . 1)))))"
    );
}
