// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.94.

mod term_lists {
    //! The `div_terms` division of exercise 2.91, restated here for
    //! exercise 2.94's `remainder_terms` and `gcd_terms` to build on.
    use ch02::sec_2_5::{Term, coeff, make_term, order};
    use sicp_runtime::{SchemeError, Value};

    fn coeff_int(t: &Term) -> Result<i128, SchemeError> {
        match coeff(t) {
            Value::Int(n) => Ok(*n),
            other => Err(SchemeError::TypeMismatch(format!(
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

    pub fn add_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
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

    fn negate(terms: &[Term]) -> Result<Vec<Term>, SchemeError> {
        terms
            .iter()
            .map(|t| Ok(make_term(order(t), Value::Int(-coeff_int(t)?))))
            .collect()
    }

    fn sub_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
        add_terms(l1, &negate(l2)?)
    }

    fn mul_term_by_all_terms(t: &Term, terms: &[Term]) -> Result<Vec<Term>, SchemeError> {
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
    /// [`SchemeError::TypeMismatch`] when the leading coefficient does
    /// not divide evenly; [`SchemeError::DivisionByZero`] for an empty
    /// divisor.
    pub fn div_terms(
        dividend: &[Term],
        divisor: &[Term],
    ) -> Result<(Vec<Term>, Vec<Term>), SchemeError> {
        if dividend.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }
        let Some(dvsr) = divisor.first() else {
            return Err(SchemeError::DivisionByZero);
        };
        let dend = &dividend[0];
        if order(dend) < order(dvsr) {
            return Ok((Vec::new(), dividend.to_vec()));
        }
        let (nc, dc) = (coeff_int(dend)?, coeff_int(dvsr)?);
        if dc == 0 {
            return Err(SchemeError::DivisionByZero);
        }
        if nc % dc != 0 {
            return Err(SchemeError::TypeMismatch(format!(
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
    pub fn gcd_terms(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SchemeError> {
        if b.is_empty() {
            return Ok(a.to_vec());
        }
        let (_, remainder) = div_terms(a, b)?;
        gcd_terms(b, &remainder)
    }

    pub fn as_pairs(terms: &[Term]) -> Result<Vec<(u32, i128)>, SchemeError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }
}

mod ex_2_94 {
    use super::term_lists::{as_pairs, gcd_terms};
    use ch02::sec_2_5::make_term;
    use sicp_runtime::{SchemeError, Value};

    /// `gcd(x^2 - 1, x - 1)`: `x^2 - 1` divides evenly by `x - 1`, giving
    /// remainder 0 on the first step, so the GCD is `x - 1` itself.
    pub fn ex_2_94() -> Result<Vec<(u32, i128)>, SchemeError> {
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
