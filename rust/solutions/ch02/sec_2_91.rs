// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.91.

mod ex_2_91 {
    use ch02::sec_2_5::{Term, coeff, make_term, order};
    use sicp_runtime::{SicpError, Value};

    type Pairs = Vec<(u32, i128)>;

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

    fn add_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SicpError> {
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

    /// `div_terms`: long division on term lists, the book's own recipe.
    /// Divide the leading term of the dividend by the leading term of the
    /// divisor to get the first term of the quotient; multiply that term
    /// by the whole divisor and subtract from the dividend; recurse on
    /// what remains. Coefficients here are ordinary integers, and the
    /// division only proceeds while it stays exact, matching this
    /// exercise's own worked example.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when the leading coefficient does
    /// not divide evenly or a coefficient is not an integer;
    /// [`SicpError::DivisionByZero`] for an empty divisor.
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
                "div_terms: {nc} does not divide evenly by {dc}; the quotient is not a polynomial"
            )));
        }

        let t = make_term(order(dend) - order(dvsr), Value::Int(nc / dc));
        let t_times_divisor = mul_term_by_all_terms(&t, divisor)?;
        let diff = sub_terms(dividend, &t_times_divisor)?;
        let (rest_q, rem) = div_terms(&diff, divisor)?;
        Ok((adjoin(t, rest_q), rem))
    }

    fn as_pairs(terms: &[Term]) -> Result<Pairs, SicpError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }

    /// `x^5 - 1` divided by `x^2 - 1` gives quotient `x^3 + x` and
    /// remainder `x - 1`, the book's own worked example.
    pub fn ex_2_91() -> Result<(Pairs, Pairs), SicpError> {
        let dividend = vec![make_term(5, Value::Int(1)), make_term(0, Value::Int(-1))];
        let divisor = vec![make_term(2, Value::Int(1)), make_term(0, Value::Int(-1))];
        let (q, r) = div_terms(&dividend, &divisor)?;
        Ok((as_pairs(&q)?, as_pairs(&r)?))
    }
}

#[test]
fn ex_2_91() {
    let (q, r) = ex_2_91::ex_2_91().expect("x^5 - 1 divides evenly by x^2 - 1 with a remainder");
    assert_eq!(q, vec![(3, 1), (1, 1)]);
    assert_eq!(r, vec![(1, 1), (0, -1)]);
}
