// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.92.

mod ex_2_92 {
    use ch02::sec_2_5::{
        add, install_generic_arithmetic, install_polynomial_package, make_polynomial, make_term,
        poly_variable,
    };
    use sicp_runtime::{OpTable, SicpError, Value};
    use std::rc::Rc;

    fn table() -> Result<Rc<OpTable>, SicpError> {
        let t = Rc::new(OpTable::new());
        install_generic_arithmetic(&t)?;
        install_polynomial_package(&t);
        Ok(t)
    }

    /// A global variable ordering, the book's own suggestion for
    /// imposing a canonical form: earlier entries are the "more
    /// principal" variable and stay on the outside.
    fn rank(order: &[&str], var: &str) -> Result<usize, SicpError> {
        order
            .iter()
            .position(|&v| v == var)
            .ok_or_else(|| SicpError::TypeMismatch(format!("{var} is not in the variable order")))
    }

    /// Embeds a polynomial as the constant-order coefficient of a new,
    /// trivial polynomial in `var`. This is how a lower-ranked
    /// polynomial becomes a coefficient of a higher-ranked one: exactly
    /// what "convert one polynomial to another polynomial that has the
    /// same principal variable" means when the whole lower polynomial
    /// becomes that coefficient.
    fn embed(table: &OpTable, var: &str, p: Value) -> Result<Value, SicpError> {
        make_polynomial(table, var, &[make_term(0, p)])
    }

    /// Adds two polynomials that may name different variables, by first
    /// bringing them under the higher-ranked (more principal) variable:
    /// the lower-ranked polynomial is embedded whole as a degree-0
    /// coefficient before the ordinary same-variable `add_poly` runs.
    /// This is the ordering-based canonical form the exercise asks for,
    /// scoped to addition, since the book itself only asks that the
    /// design "work" and warns it "is not easy" in general.
    fn add_with_ordering(
        table: &OpTable,
        order: &[&str],
        p: &Value,
        q: &Value,
    ) -> Result<Value, SicpError> {
        let pv = poly_variable(&ch02::sec_2_5::contents(p)?)?;
        let qv = poly_variable(&ch02::sec_2_5::contents(q)?)?;
        if pv.as_ref() == qv.as_ref() {
            return add(table, p, q);
        }
        if rank(order, pv.as_ref())? < rank(order, qv.as_ref())? {
            return add(table, p, &embed(table, pv.as_ref(), q.clone())?);
        }
        add(table, &embed(table, qv.as_ref(), p.clone())?, q)
    }

    /// `x^2` (principal variable `x`) plus `y + 1` (principal variable
    /// `y`), under the order `[x, y]`: `y + 1` has no `x^0` coefficient
    /// to collide with, so it becomes the whole constant coefficient of
    /// an `x`-polynomial, and `x` stays on the outside as the book's
    /// canonical form requires.
    pub fn ex_2_92() -> Result<String, SicpError> {
        let table = table()?;
        let order = ["x", "y"];

        let p = make_polynomial(&table, "x", &[make_term(2, Value::Int(1))])?;
        let q = make_polynomial(
            &table,
            "y",
            &[make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
        )?;

        let sum = add_with_ordering(&table, &order, &p, &q)?;
        Ok(sum.to_string())
    }
}

#[test]
fn ex_2_92() {
    let sum = ex_2_92::ex_2_92().expect("addition under a variable ordering");
    assert_eq!(
        sum,
        "(polynomial (x (2 1) (0 (polynomial (y (1 1) (0 1))))))"
    );
}
