// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3.2

//! Section 2.3.2: symbolic differentiation, unsimplified and simplified.

use ch02::sec_2_3::{Expr, deriv, deriv_unsimplified};
use sicp_runtime::Symbol;

fn var(s: &str) -> Expr {
    Expr::Var(Symbol::from(s))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = Symbol::from("x");

    // `deriv_unsimplified` follows the naive `make-sum`/`make-product`:
    // every combination stays an unreduced `Expr::Sum`/`Expr::Product`.
    let sum = Expr::Sum(Box::new(var("x")), Box::new(Expr::Num(3)));
    println!("{}", deriv_unsimplified(&sum, &x)?);
    // => (+ 1 0)
    assert_eq!(deriv_unsimplified(&sum, &x)?.to_string(), "(+ 1 0)");

    let product = Expr::Product(Box::new(var("x")), Box::new(var("y")));
    println!("{}", deriv_unsimplified(&product, &x)?);
    // => (+ (* x 0) (* 1 y))
    assert_eq!(
        deriv_unsimplified(&product, &x)?.to_string(),
        "(+ (* x 0) (* 1 y))"
    );

    let nested = Expr::Product(
        Box::new(Expr::Product(Box::new(var("x")), Box::new(var("y")))),
        Box::new(Expr::Sum(Box::new(var("x")), Box::new(Expr::Num(3)))),
    );
    println!("{}", deriv_unsimplified(&nested, &x)?);
    // => (+ (* (* x y) (+ 1 0)) (* (+ (* x 0) (* 1 y)) (+ x 3)))
    assert_eq!(
        deriv_unsimplified(&nested, &x)?.to_string(),
        "(+ (* (* x y) (+ 1 0)) (* (+ (* x 0) (* 1 y)) (+ x 3)))"
    );

    // `deriv` folds `0 + a = a`, `1 * m = m`, and numeric combinations,
    // matching the book's improved examples on the same three inputs.
    println!("{}", deriv(&sum, &x)?);
    // => 1
    assert_eq!(deriv(&sum, &x)?.to_string(), "1");

    println!("{}", deriv(&product, &x)?);
    // => y
    assert_eq!(deriv(&product, &x)?.to_string(), "y");

    println!("{}", deriv(&nested, &x)?);
    // => (+ (* x y) (* y (+ x 3)))
    assert_eq!(
        deriv(&nested, &x)?.to_string(),
        "(+ (* x y) (* y (+ x 3)))"
    );

    Ok(())
}
