// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.57, one module and one test.

mod ex_2_57 {
    use sicp_runtime::{SchemeError, Symbol};
    use std::fmt;

    /// The section's `Expr`, with `Sum` and `Product` holding any
    /// number of terms instead of exactly two: this exercise's own
    /// widened representation.
    #[derive(Clone)]
    enum Expr {
        Num(i128),
        Var(Symbol),
        Sum(Vec<Expr>),
        Product(Vec<Expr>),
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expr::Num(n) => write!(f, "{n}"),
                Expr::Var(x) => write!(f, "{x}"),
                Expr::Sum(terms) => write_terms(f, "+", terms),
                Expr::Product(terms) => write_terms(f, "*", terms),
            }
        }
    }

    fn write_terms(f: &mut fmt::Formatter<'_>, op: &str, terms: &[Expr]) -> fmt::Result {
        write!(f, "({op}")?;
        for term in terms {
            write!(f, " {term}")?;
        }
        write!(f, ")")
    }

    fn is_number(e: &Expr, n: i128) -> bool {
        matches!(e, Expr::Num(x) if *x == n)
    }

    /// The addend of a sum is its first term: the book's `addend`,
    /// changed only in that `terms` may hold more than two elements.
    fn addend(terms: &[Expr]) -> Expr {
        terms[0].clone()
    }

    /// The augend of a sum is its second term when there are exactly
    /// two, or the sum of the rest when there are more: the book's
    /// `addend`/`augend` split is the only place n-ary support enters,
    /// leaving `deriv` untouched below.
    fn augend(terms: &[Expr]) -> Expr {
        if terms.len() == 2 {
            terms[1].clone()
        } else {
            Expr::Sum(terms[1..].to_vec())
        }
    }

    fn multiplier(terms: &[Expr]) -> Expr {
        terms[0].clone()
    }

    fn multiplicand(terms: &[Expr]) -> Expr {
        if terms.len() == 2 {
            terms[1].clone()
        } else {
            Expr::Product(terms[1..].to_vec())
        }
    }

    fn make_sum(a1: Expr, a2: Expr) -> Result<Expr, SchemeError> {
        if is_number(&a1, 0) {
            return Ok(a2);
        }
        if is_number(&a2, 0) {
            return Ok(a1);
        }
        if let (Expr::Num(x), Expr::Num(y)) = (&a1, &a2) {
            return Ok(Expr::Num(x.checked_add(*y).ok_or(SchemeError::Overflow)?));
        }
        Ok(Expr::Sum(vec![a1, a2]))
    }

    fn make_product(m1: Expr, m2: Expr) -> Result<Expr, SchemeError> {
        if is_number(&m1, 0) || is_number(&m2, 0) {
            return Ok(Expr::Num(0));
        }
        if is_number(&m1, 1) {
            return Ok(m2);
        }
        if is_number(&m2, 1) {
            return Ok(m1);
        }
        if let (Expr::Num(x), Expr::Num(y)) = (&m1, &m2) {
            return Ok(Expr::Num(x.checked_mul(*y).ok_or(SchemeError::Overflow)?));
        }
        Ok(Expr::Product(vec![m1, m2]))
    }

    /// `deriv`, unchanged from the two-term version except that
    /// `Sum`/`Product` carry a `Vec` of terms instead of two fixed
    /// fields; every case still goes through `addend`/`augend` and
    /// `multiplier`/`multiplicand`, so widening those four functions
    /// to handle more than two terms is the entire adaptation.
    ///
    /// # Errors
    /// [`SchemeError::Overflow`] on a numeric fold past the `i128`
    /// range.
    fn deriv(exp: &Expr, var: &Symbol) -> Result<Expr, SchemeError> {
        match exp {
            Expr::Num(_) => Ok(Expr::Num(0)),
            Expr::Var(x) => Ok(Expr::Num(i128::from(x == var))),
            Expr::Sum(terms) => make_sum(deriv(&addend(terms), var)?, deriv(&augend(terms), var)?),
            Expr::Product(terms) => {
                let left = make_product(multiplier(terms), deriv(&multiplicand(terms), var)?)?;
                let right = make_product(deriv(&multiplier(terms), var)?, multiplicand(terms))?;
                make_sum(left, right)
            }
        }
    }

    /// Exercise 2.57: sums and products of arbitrary numbers of terms
    ///
    /// Returns the printed derivative of `x * y * (x + 3)` with
    /// respect to `x`, using a three-term product and a two-term sum.
    pub fn ex_2_57() -> String {
        let x = Symbol::from("x");
        let expr = Expr::Product(vec![
            Expr::Var(Symbol::from("x")),
            Expr::Var(Symbol::from("y")),
            Expr::Sum(vec![Expr::Var(Symbol::from("x")), Expr::Num(3)]),
        ]);
        deriv(&expr, &x)
            .expect("no overflow on this example")
            .to_string()
    }
}

#[test]
fn ex_2_57() {
    assert_eq!(ex_2_57::ex_2_57(), "(+ (* x y) (* y (+ x 3)))".to_string());
}
