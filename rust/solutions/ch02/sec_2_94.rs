// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.94.

use std::rc::Rc;

use ch02::sec_2_5::{
    contents, install_generic_arithmetic, install_polynomial_package, make_polynomial, make_term,
    poly_term_list,
};
use sicp_runtime::{OpTable, SicpError, Value};

mod term_lists {
    //! The `div_terms` division of exercise 2.91, restated here for
    //! exercise 2.94's `remainder_terms` and `gcd_terms` to build on.
    use ch02::sec_2_5::{Term, coeff, make_term, order};
    use sicp_runtime::{SicpError, Value};

    fn coeff_int(t: &Term) -> Result<i128, SicpError> {
        match coeff(t) {
            Value::Int(n) => Ok(*n),
            other => Err(SicpError::TypeMismatch(format!(
                "only integer coefficients are supported here: {other}"
            ))),
        }
    }

    fn adjoin(t: Term, rest: Vec<Term>) -> Vec<Term> {
        if coeff(&t) == &Value::Int(0) {
            return rest;
        }
        let mut out = Vec::with_capacity(rest.len() + 1);
        out.push(t);
        out.extend(rest);
        out
    }

    pub fn add_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SicpError> {
        match (l1.first(), l2.first()) {
            (None, _) => Ok(l2.to_vec()),
            (_, None) => Ok(l1.to_vec()),
            (Some(t1), Some(t2)) => match order(t1).cmp(&order(t2)) {
                std::cmp::Ordering::Greater => Ok(adjoin(t1.clone(), add_terms(&l1[1..], l2)?)),
                std::cmp::Ordering::Less => Ok(adjoin(t2.clone(), add_terms(l1, &l2[1..])?)),
                std::cmp::Ordering::Equal => Ok(adjoin(
                    make_term(order(t1), Value::Int(coeff_int(t1)? + coeff_int(t2)?)),
                    add_terms(&l1[1..], &l2[1..])?,
                )),
            },
        }
    }

    fn negate(terms: &[Term]) -> Result<Vec<Term>, SicpError> {
        terms
            .iter()
            .map(|t| Ok(make_term(order(t), Value::Int(-coeff_int(t)?))))
            .collect()
    }

    fn sub_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SicpError> {
        add_terms(l1, &negate(l2)?)
    }

    fn mul_term_by_all_terms(t: &Term, terms: &[Term]) -> Result<Vec<Term>, SicpError> {
        terms
            .iter()
            .map(|u| {
                Ok(make_term(
                    order(t) + order(u),
                    Value::Int(coeff_int(t)? * coeff_int(u)?),
                ))
            })
            .collect()
    }

    /// Long division on term lists: exercise 2.91.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when the leading coefficient does
    /// not divide evenly; [`SicpError::DivisionByZero`] for an empty
    /// divisor.
    pub fn div_terms(
        dividend: &[Term],
        divisor: &[Term],
    ) -> Result<(Vec<Term>, Vec<Term>), SicpError> {
        if dividend.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }
        let Some(dvsr) = divisor.first() else {
            return Err(SicpError::DivisionByZero);
        };
        let dend = &dividend[0];
        if order(dend) < order(dvsr) {
            return Ok((Vec::new(), dividend.to_vec()));
        }
        let (nc, dc) = (coeff_int(dend)?, coeff_int(dvsr)?);
        if dc == 0 {
            return Err(SicpError::DivisionByZero);
        }
        if nc % dc != 0 {
            return Err(SicpError::TypeMismatch(format!(
                "div_terms: {nc} does not divide evenly by {dc}"
            )));
        }
        let t = make_term(order(dend) - order(dvsr), Value::Int(nc / dc));
        let diff = sub_terms(dividend, &mul_term_by_all_terms(&t, divisor)?)?;
        let (rest_q, rem) = div_terms(&diff, divisor)?;
        Ok((adjoin(t, rest_q), rem))
    }

    /// Exercise 2.94: `remainder_terms` is just the second half of
    /// `div_terms`'s pair, and `gcd_terms` is Euclid's algorithm run on
    /// term lists instead of integers: `gcd(a, 0) = a`, otherwise
    /// `gcd(a, b) = gcd(b, remainder_terms(a, b))`.
    ///
    /// # Errors
    /// Whatever [`div_terms`] raises along the way.
    pub fn gcd_terms(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SicpError> {
        if b.is_empty() {
            return Ok(a.to_vec());
        }
        let (_, remainder) = div_terms(a, b)?;
        gcd_terms(b, &remainder)
    }

    pub fn as_pairs(terms: &[Term]) -> Result<Vec<(u32, i128)>, SicpError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }
}

mod generic {
    //! Exercise 2.94's generic `greatest_common_divisor`: the book's
    //! "install in the system a generic operation" as two table
    //! entries, one entry point dispatching on the argument tags,
    //! reducing to [`gcd_poly`] for polynomials and to ordinary Euclid
    //! for integers.
    use super::term_lists::gcd_terms;
    use ch02::sec_2_5::{apply_generic, make_polynomial, poly_term_list, poly_variable};
    use sicp_runtime::{Key, OpTable, SicpError, Value};
    use std::rc::Rc;

    /// Folds a slice of keys into the list-shaped key the table stores
    /// for an ordered tag list, the book's `'(t1 t2)`.
    fn key_list(keys: &[Key]) -> Key {
        let mut out = Key::Nil;
        for k in keys.iter().rev() {
            out = Key::pair(k.clone(), out);
        }
        out
    }

    /// Euclid's `gcd` on integers: what the operation reduces to for
    /// ordinary numbers.
    fn gcd_i128(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        i128::try_from(a).unwrap_or(i128::MAX)
    }

    /// The book's `gcd-poly`: signals an error unless the two polys
    /// share a variable, then runs [`gcd_terms`] on their term lists.
    ///
    /// # Errors
    /// [`SicpError::UserRaised`] when the polys are not in the same
    /// variable; whatever [`gcd_terms`] raises.
    fn gcd_poly(table: &OpTable, p1: &Value, p2: &Value) -> Result<Value, SicpError> {
        let v1 = poly_variable(p1)?;
        let v2 = poly_variable(p2)?;
        if v1 != v2 {
            return Err(SicpError::UserRaised {
                message: "Polys not in same var: GCD-POLY".to_string(),
                irritants: vec![p1.clone(), p2.clone()],
            });
        }
        let g = gcd_terms(&poly_term_list(p1)?, &poly_term_list(p2)?)?;
        make_polynomial(table, v1.as_ref(), &g)
    }

    /// Installs the generic operation under the tags it dispatches on:
    /// `(integer, integer)` and `(polynomial, polynomial)`.
    pub fn install_greatest_common_divisor(table: &Rc<OpTable>) {
        let int_key = key_list(&[Key::sym("integer"), Key::sym("integer")]);
        table.put(
            Key::sym("greatest_common_divisor"),
            int_key,
            Rc::new(|args: &[Value]| match (&args[0], &args[1]) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(gcd_i128(*a, *b))),
                _ => Err(SicpError::TypeMismatch(
                    "greatest_common_divisor: operands are not integers".into(),
                )),
            }),
        );
        let poly_key = key_list(&[Key::sym("polynomial"), Key::sym("polynomial")]);
        let t = Rc::clone(table);
        table.put(
            Key::sym("greatest_common_divisor"),
            poly_key,
            Rc::new(move |args: &[Value]| gcd_poly(&t, &args[0], &args[1])),
        );
    }

    /// The generic entry point the texi's listing calls:
    /// one [`apply_generic`] dispatch over the installed entries.
    ///
    /// # Errors
    /// Whatever [`apply_generic`] raises when no entry matches the two
    /// arguments' tags, and whatever the dispatched handler raises.
    pub fn greatest_common_divisor(
        table: &OpTable,
        x: &Value,
        y: &Value,
    ) -> Result<Value, SicpError> {
        apply_generic(table, "greatest_common_divisor", &[x.clone(), y.clone()])
    }
}

mod ex_2_94 {
    use super::term_lists::{as_pairs, gcd_terms};
    use ch02::sec_2_5::make_term;
    use sicp_runtime::{SicpError, Value};

    /// `gcd(x^2 - 1, x - 1)`: `x^2 - 1` divides evenly by `x - 1`, giving
    /// remainder 0 on the first step, so the GCD is `x - 1` itself.
    pub fn ex_2_94() -> Result<Vec<(u32, i128)>, SicpError> {
        let p1 = vec![make_term(2, Value::Int(1)), make_term(0, Value::Int(-1))];
        let p2 = vec![make_term(1, Value::Int(1)), make_term(0, Value::Int(-1))];
        let gcd = gcd_terms(&p1, &p2)?;
        as_pairs(&gcd)
    }
}

#[test]
fn ex_2_94() {
    let gcd = ex_2_94::ex_2_94().expect("gcd of x^2 - 1 and x - 1");
    assert_eq!(gcd, vec![(1, 1), (0, -1)]);
}

/// The printed form of the book's `greatest_common_divisor` answer,
/// plus the two remainders that "check your result by hand" divides to
/// zero.
type BookExample = (String, Vec<(u32, i128)>, Vec<(u32, i128)>);

/// The texi's test listing: the book's p1 and p2 through the installed
/// generic operation.
fn book_example() -> Result<BookExample, SicpError> {
    let table = Rc::new(OpTable::new());
    install_generic_arithmetic(&table)?;
    install_polynomial_package(&table);
    generic::install_greatest_common_divisor(&table);
    let p1 = make_polynomial(
        &table,
        "x",
        &[
            make_term(4, Value::Int(1)),
            make_term(3, Value::Int(-1)),
            make_term(2, Value::Int(-2)),
            make_term(1, Value::Int(2)),
        ],
    )?;
    let p2 = make_polynomial(
        &table,
        "x",
        &[make_term(3, Value::Int(1)), make_term(1, Value::Int(-1))],
    )?;
    let g = generic::greatest_common_divisor(&table, &p1, &p2)?;
    let g_terms = poly_term_list(&contents(&g)?)?;
    let (_, r1) = term_lists::div_terms(&poly_term_list(&contents(&p1)?)?, &g_terms)?;
    let (_, r2) = term_lists::div_terms(&poly_term_list(&contents(&p2)?)?, &g_terms)?;
    Ok((
        g.to_string(),
        term_lists::as_pairs(&r1)?,
        term_lists::as_pairs(&r2)?,
    ))
}

#[test]
fn ex_2_94_book_example() {
    let (display, r1, r2) = book_example().expect("the book's gcd example runs");
    // Euclid's loop determines the GCD only up to a constant factor;
    // this run's unit leaves -(x^2 - x), not the book's x^2 - x.
    assert_eq!(display, "(polynomial (x (2 -1) (1 1)))");
    // "check your result by hand": the answer divides both inputs
    // exactly, leaving no remainder.
    assert_eq!(r1, Vec::new());
    assert_eq!(r2, Vec::new());
}

#[test]
fn ex_2_94_integer_branch() {
    let table = Rc::new(OpTable::new());
    install_generic_arithmetic(&table).expect("packages install");
    install_polynomial_package(&table);
    generic::install_greatest_common_divisor(&table);
    let g = generic::greatest_common_divisor(&table, &Value::Int(36), &Value::Int(24))
        .expect("integer gcd");
    assert_eq!(g, Value::Int(12));
    let px = make_polynomial(&table, "x", &[make_term(1, Value::Int(1))]).expect("px builds");
    let err = generic::greatest_common_divisor(&table, &Value::Int(36), &px)
        .expect_err("no entry dispatches across the two tags");
    assert!(err.to_string().contains("No method"), "{err}");
}

#[test]
fn ex_2_94_signals_different_variables() {
    let table = Rc::new(OpTable::new());
    install_generic_arithmetic(&table).expect("packages install");
    install_polynomial_package(&table);
    generic::install_greatest_common_divisor(&table);
    let px = make_polynomial(&table, "x", &[make_term(1, Value::Int(1))]).expect("px builds");
    let py = make_polynomial(&table, "y", &[make_term(1, Value::Int(1))]).expect("py builds");
    let err = generic::greatest_common_divisor(&table, &px, &py)
        .expect_err("gcd_poly signals different variables");
    assert!(err.to_string().contains("not in same var"), "{err}");
}
