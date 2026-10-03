// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.93.

mod ex_2_93 {
    use ch02::sec_2_5::{add, install_generic_arithmetic, install_polynomial_package, mul};
    use sicp_runtime::{Key, OpTable, SicpError, Value, cons_cell};
    use std::rc::Rc;

    fn car(p: &Value) -> Result<Value, SicpError> {
        match p {
            Value::Pair(cell) => Ok(cell.car.borrow().clone()),
            other => Err(SicpError::TypeMismatch(format!("not a pair: {other}"))),
        }
    }

    fn cdr(p: &Value) -> Result<Value, SicpError> {
        match p {
            Value::Pair(cell) => Ok(cell.cdr.borrow().clone()),
            other => Err(SicpError::TypeMismatch(format!("not a pair: {other}"))),
        }
    }

    fn two_tags(a: &str, b: &str) -> Key {
        Key::pair(Key::sym(a), Key::pair(Key::sym(b), Key::Nil))
    }

    /// The rational-function package of this exercise: `numer` and
    /// `denom` may be any generic value the table's `add`/`mul` know how
    /// to combine, including a polynomial. Unlike the 2.1.1 rational
    /// package, this one does not reduce to lowest terms yet; that is
    /// exercise 2.97.
    fn install_rational_function_package(table: &Rc<OpTable>) {
        table.put(
            Key::sym("make"),
            Key::sym("rational-function"),
            Rc::new(|args: &[Value]| {
                Ok(Value::tagged(
                    "rational-function",
                    Value::Pair(cons_cell(args[0].clone(), args[1].clone())),
                ))
            }),
        );
        let t = Rc::clone(table);
        table.put(
            Key::sym("add"),
            two_tags("rational-function", "rational-function"),
            Rc::new(move |args: &[Value]| {
                let (n1, d1) = (car(&args[0])?, cdr(&args[0])?);
                let (n2, d2) = (car(&args[1])?, cdr(&args[1])?);
                let numer = add(&t, &mul(&t, &n1, &d2)?, &mul(&t, &n2, &d1)?)?;
                let denom = mul(&t, &d1, &d2)?;
                make_rational_function(&t, numer, denom)
            }),
        );
        let t = Rc::clone(table);
        table.put(
            Key::sym("mul"),
            two_tags("rational-function", "rational-function"),
            Rc::new(move |args: &[Value]| {
                let (n1, d1) = (car(&args[0])?, cdr(&args[0])?);
                let (n2, d2) = (car(&args[1])?, cdr(&args[1])?);
                let numer = mul(&t, &n1, &n2)?;
                let denom = mul(&t, &d1, &d2)?;
                make_rational_function(&t, numer, denom)
            }),
        );
    }

    fn make_rational_function(table: &OpTable, n: Value, d: Value) -> Result<Value, SicpError> {
        let make = table
            .get(&Key::sym("make"), &Key::sym("rational-function"))
            .ok_or_else(|| SicpError::TypeMismatch("no rational-function constructor".into()))?;
        make(&[n, d])
    }

    /// `(p / q) + (p / q)` doubles the numerator over the squared
    /// denominator, without reducing: `2pq / q^2`, using the ordinary
    /// polynomial `add`/`mul` for its arithmetic on the way, exactly as
    /// the un-generalized 2.1.1 rational package used `+`/`*` on plain
    /// numbers.
    pub fn ex_2_93() -> Result<String, SicpError> {
        let table = Rc::new(OpTable::new());
        install_generic_arithmetic(&table)?;
        install_polynomial_package(&table);
        install_rational_function_package(&table);

        let p = ch02::sec_2_5::make_polynomial(
            &table,
            "x",
            &[
                ch02::sec_2_5::make_term(1, Value::Int(1)),
                ch02::sec_2_5::make_term(0, Value::Int(1)),
            ],
        )?;
        let q = ch02::sec_2_5::make_polynomial(
            &table,
            "x",
            &[
                ch02::sec_2_5::make_term(2, Value::Int(1)),
                ch02::sec_2_5::make_term(0, Value::Int(-1)),
            ],
        )?;
        let rf = make_rational_function(&table, p, q)?;
        let doubled = add(&table, &rf, &rf)?;
        Ok(doubled.to_string())
    }
}

#[test]
fn ex_2_93() {
    let doubled = ex_2_93::ex_2_93().expect("rational functions over polynomial coefficients");
    // 2pq / q^2 = 2(x + 1)(x^2 - 1) / (x^2 - 1)^2, unreduced.
    assert_eq!(
        doubled,
        "(rational-function ((polynomial (x (3 2) (2 2) (1 -2) (0 -2))) . (polynomial (x (4 1) (2 -2) (0 1)))))"
    );
}
