// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.56, one module and one test.

mod ex_2_56 {
    use sicp_runtime::{SicpError, Symbol};
    use std::fmt;

    /// The section's `Expr`, extended with exponentiation: this
    /// exercise's own contribution, kept local rather than folded into
    /// the section library's closed `Expr`, per the section's running
    /// convention of one representation per exercise.
    #[derive(Clone)]
    enum Expr {
        Num(i128),
        Var(Symbol),
        Sum(Box<Expr>, Box<Expr>),
        Product(Box<Expr>, Box<Expr>),
        Pow(Box<Expr>, Box<Expr>),
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expr::Num(n) => write!(f, "{n}"),
                Expr::Var(x) => write!(f, "{x}"),
                Expr::Sum(a, b) => write!(f, "(+ {a} {b})"),
                Expr::Product(a, b) => write!(f, "(* {a} {b})"),
                Expr::Pow(base, exp) => write!(f, "(** {base} {exp})"),
            }
        }
    }

    fn is_number(e: &Expr, n: i128) -> bool {
        matches!(e, Expr::Num(x) if *x == n)
    }

    fn make_sum(a1: Expr, a2: Expr) -> Result<Expr, SicpError> {
        if is_number(&a1, 0) {
            return Ok(a2);
        }
        if is_number(&a2, 0) {
            return Ok(a1);
        }
        if let (Expr::Num(x), Expr::Num(y)) = (&a1, &a2) {
            return Ok(Expr::Num(x.checked_add(*y).ok_or(SicpError::Overflow)?));
        }
        Ok(Expr::Sum(Box::new(a1), Box::new(a2)))
    }

    fn make_product(m1: Expr, m2: Expr) -> Result<Expr, SicpError> {
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
            return Ok(Expr::Num(x.checked_mul(*y).ok_or(SicpError::Overflow)?));
        }
        Ok(Expr::Product(Box::new(m1), Box::new(m2)))
    }

    /// Builds `base ** exponent`, folding the rules that anything
    /// raised to the power 0 is 1, and anything raised to the power 1
    /// is itself.
    fn make_exponentiation(base: Expr, exponent: Expr) -> Expr {
        if is_number(&exponent, 0) {
            return Expr::Num(1);
        }
        if is_number(&exponent, 1) {
            return base;
        }
        Expr::Pow(Box::new(base), Box::new(exponent))
    }

    /// `deriv`, extended with the power rule
    /// `d(u^n)/dx = n * u^(n-1) * du/dx`, valid for a constant integer
    /// exponent.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] if an exponent is not a constant
    /// number; [`SicpError::Overflow`] on a numeric fold past the
    /// `i128` range.
    fn deriv(exp: &Expr, var: &Symbol) -> Result<Expr, SicpError> {
        match exp {
            Expr::Num(_) => Ok(Expr::Num(0)),
            Expr::Var(x) => Ok(Expr::Num(i128::from(x == var))),
            Expr::Sum(a1, a2) => make_sum(deriv(a1, var)?, deriv(a2, var)?),
            Expr::Product(m1, m2) => {
                let left = make_product((**m1).clone(), deriv(m2, var)?)?;
                let right = make_product(deriv(m1, var)?, (**m2).clone())?;
                make_sum(left, right)
            }
            Expr::Pow(base, exponent) => {
                let Expr::Num(n) = exponent.as_ref() else {
                    return Err(SicpError::TypeMismatch(
                        "exponent must be a constant integer".to_string(),
                    ));
                };
                let lowered = make_exponentiation((**base).clone(), Expr::Num(n - 1));
                let coefficient = make_product(Expr::Num(*n), lowered)?;
                make_product(coefficient, deriv(base, var)?)
            }
        }
    }

    /// Exercise 2.56: extending `deriv` with exponentiation
    ///
    /// Returns the printed derivative of `x^3` with respect to `x`.
    pub fn ex_2_56() -> String {
        let x = Symbol::from("x");
        let expr = Expr::Pow(
            Box::new(Expr::Var(Symbol::from("x"))),
            Box::new(Expr::Num(3)),
        );
        deriv(&expr, &x)
            .expect("no overflow or non-constant exponent on this example")
            .to_string()
    }
}

#[test]
fn ex_2_56() {
    assert_eq!(ex_2_56::ex_2_56(), "(* 3 (** x 2))".to_string());
}
