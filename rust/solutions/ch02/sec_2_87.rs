// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.87.

mod ex_2_87 {
    use ch02::sec_2_5::{
        add, adjoin_term, install_generic_arithmetic, install_polynomial_is_zero,
        install_polynomial_package, is_zero, make_polynomial, make_term, the_empty_termlist,
    };
    use sicp_runtime::{OpTable, SicpError, Value};
    use std::rc::Rc;

    fn table() -> Result<Rc<OpTable>, SicpError> {
        let t = Rc::new(OpTable::new());
        install_generic_arithmetic(&t)?;
        install_polynomial_package(&t);
        install_polynomial_is_zero(&t);
        Ok(t)
    }

    /// Exercise 2.87: `is_zero` for polynomials answers true exactly when
    /// every coefficient in the term list is itself `=zero?`, so
    /// `adjoin_term` can drop a zero polynomial coefficient the same way
    /// it drops a zero number. The observable proof is a polynomial
    /// coefficient of a polynomial: `y - y` (an all-zero termlist) added
    /// as the top coefficient of an `x`-polynomial disappears from the
    /// printed result, exactly as a zero-number coefficient would.
    pub fn ex_2_87() -> Result<(String, bool, bool), SicpError> {
        let table = table()?;

        let zero_poly = make_polynomial(&table, "y", &the_empty_termlist())?;
        let nonzero_poly = make_polynomial(&table, "y", &[make_term(0, Value::Int(1))])?;

        let is_zero_poly = is_zero(&table, &zero_poly)?;
        let is_zero_nonzero = is_zero(&table, &nonzero_poly)?;

        // adjoin_term uses the generic =zero? underneath, so a zero
        // polynomial coefficient never enters the term list: only the
        // nonzero coefficient survives.
        let terms = adjoin_term(
            &table,
            make_term(1, zero_poly),
            &adjoin_term(&table, make_term(0, nonzero_poly), &the_empty_termlist())?,
        )?;
        let p = make_polynomial(&table, "x", &terms)?;
        let sum = add(&table, &p, &p)?;

        Ok((sum.to_string(), is_zero_poly, is_zero_nonzero))
    }
}

#[test]
fn ex_2_87() {
    let (sum, is_zero_poly, is_zero_nonzero) = ex_2_87::ex_2_87().expect("polynomial =zero?");
    assert!(is_zero_poly, "an empty term list is the zero polynomial");
    assert!(!is_zero_nonzero);
    // Only the order-0 term survives; the zero-polynomial coefficient at
    // order 1 was dropped by adjoin_term before make_polynomial ever saw
    // it, and doubling the constant polynomial coefficient doubles its
    // own inner constant term.
    assert_eq!(sum, "(polynomial (x (0 (polynomial (y (0 2))))))");
}
