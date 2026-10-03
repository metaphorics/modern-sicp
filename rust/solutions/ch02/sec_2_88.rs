// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.88.

mod ex_2_88 {
    use ch02::sec_2_5::{
        Term, add, coeff, contents, install_generic_arithmetic, install_polynomial_is_zero,
        install_polynomial_package, make_polynomial, make_term, order, poly_term_list,
        poly_variable,
    };
    use sicp_runtime::{OpTable, SicpError, Value};
    use std::rc::Rc;

    fn table() -> Result<Rc<OpTable>, SicpError> {
        let t = Rc::new(OpTable::new());
        install_generic_arithmetic(&t)?;
        install_polynomial_package(&t);
        // Nested polynomial coefficients need exercise 2.87's =zero? so
        // adjoin_term can drop a zero coefficient produced by a
        // self-subtraction.
        install_polynomial_is_zero(&t);
        Ok(t)
    }

    /// The book's hint: a generic negation. Ordinary numbers negate
    /// directly; a polynomial coefficient negates by recursing into its
    /// own term list, so a polynomial whose coefficients are themselves
    /// polynomials negates all the way down for free.
    fn negate(table: &OpTable, v: &Value) -> Result<Value, SicpError> {
        match v {
            Value::Int(n) => n.checked_neg().map(Value::Int).ok_or(SicpError::Overflow),
            Value::Real(x) => Ok(Value::Real(-x)),
            Value::Tagged { tag, .. } if tag.as_ref() == "polynomial" => {
                let bare = contents(v)?;
                let var = poly_variable(&bare)?;
                let negated_terms = negate_terms(table, &poly_term_list(&bare)?)?;
                make_polynomial(table, var.as_ref(), &negated_terms)
            }
            other => Err(SicpError::TypeMismatch(format!(
                "negate: unsupported coefficient {other}"
            ))),
        }
    }

    fn negate_terms(table: &OpTable, terms: &[Term]) -> Result<Vec<Term>, SicpError> {
        terms
            .iter()
            .map(|t| Ok(make_term(order(t), negate(table, coeff(t))?)))
            .collect()
    }

    /// `sub_poly`: add the negation, exactly like the book's
    /// `sub-poly` hint.
    fn sub_poly(table: &OpTable, p1: &Value, p2: &Value) -> Result<Value, SicpError> {
        add(table, p1, &negate(table, p2)?)
    }

    /// `(x^2 + 3x + 7) - (x^2 + 3x)` leaves the constant term, and
    /// subtracting a polynomial from itself leaves the empty polynomial;
    /// subtracting a polynomial coefficient of a polynomial recurses
    /// through the generic `sub` the same way addition recurses through
    /// generic `add`.
    pub fn ex_2_88() -> Result<(String, String, String), SicpError> {
        let table = table()?;

        let p1 = make_polynomial(
            &table,
            "x",
            &[
                make_term(2, Value::Int(1)),
                make_term(1, Value::Int(3)),
                make_term(0, Value::Int(7)),
            ],
        )?;
        let p2 = make_polynomial(
            &table,
            "x",
            &[make_term(2, Value::Int(1)), make_term(1, Value::Int(3))],
        )?;
        let diff = sub_poly(&table, &p1, &p2)?;
        let self_diff = sub_poly(&table, &p1, &p1)?;

        let y1 = make_polynomial(&table, "y", &[make_term(1, Value::Int(2))])?;
        let y2 = make_polynomial(&table, "y", &[make_term(0, Value::Int(1))])?;
        let nested = make_polynomial(&table, "x", &[make_term(0, y1)])?;
        let nested_sub = sub_poly(
            &table,
            &nested,
            &make_polynomial(&table, "x", &[make_term(0, y2)])?,
        )?;

        Ok((
            diff.to_string(),
            self_diff.to_string(),
            nested_sub.to_string(),
        ))
    }
}

#[test]
fn ex_2_88() {
    let (diff, self_diff, nested) = ex_2_88::ex_2_88().expect("polynomial subtraction");
    assert_eq!(diff, "(polynomial (x (0 7)))");
    assert_eq!(self_diff, "(polynomial (x))");
    assert_eq!(nested, "(polynomial (x (0 (polynomial (y (1 2) (0 -1))))))");
}
