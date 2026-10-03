// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.96.

mod term_lists {
    //! Exercise 2.94's term-list arithmetic, plus 2.96's fix: multiply
    //! the dividend by an integerizing factor before dividing, so every
    //! quotient this edition's `div_terms` computes stays an integer.
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

    /// Full term-list multiplication, needed here to build `Q_1` and
    /// `Q_2` the same way exercise 2.95 did.
    pub fn mul_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SicpError> {
        match l1.first() {
            None => Ok(Vec::new()),
            Some(t1) => {
                let head = mul_term_by_all_terms(t1, l2)?;
                add_terms(&head, &mul_terms(&l1[1..], l2)?)
            }
        }
    }

    fn scale_terms(terms: &[Term], factor: i128) -> Result<Vec<Term>, SicpError> {
        terms
            .iter()
            .map(|t| Ok(make_term(order(t), Value::Int(coeff_int(t)? * factor))))
            .collect()
    }

    fn div_terms(dividend: &[Term], divisor: &[Term]) -> Result<(Vec<Term>, Vec<Term>), SicpError> {
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

    /// The integerizing factor for dividing `p` by `q`: `q`'s leading
    /// coefficient raised to `1 + O_1 - O_2`, where `O_1` is `p`'s order
    /// and `O_2` is `q`'s. `None` when `p` is already lower order than
    /// `q` -- `div_terms` answers that case without dividing anything,
    /// so no factor is needed (and the exponent could go negative).
    fn integerizing_factor(p: &[Term], q: &[Term]) -> Result<Option<i128>, SicpError> {
        let (Some(pt), Some(qt)) = (p.first(), q.first()) else {
            return Ok(None);
        };
        if order(pt) < order(qt) {
            return Ok(None);
        }
        let exp = 1 + i64::from(order(pt)) - i64::from(order(qt));
        let exp = u32::try_from(exp).map_err(|_| SicpError::Overflow)?;
        let c = coeff_int(qt)?;
        Ok(Some(c.checked_pow(exp).ok_or(SicpError::Overflow)?))
    }

    /// Exercise 2.96(a): just like `remainder_terms`, but it multiplies
    /// the dividend by the integerizing factor before calling
    /// `div_terms`, so the division never meets a coefficient that
    /// fails to divide evenly.
    pub fn pseudoremainder_terms(p: &[Term], q: &[Term]) -> Result<Vec<Term>, SicpError> {
        let Some(factor) = integerizing_factor(p, q)? else {
            return Ok(p.to_vec());
        };
        let scaled = scale_terms(p, factor)?;
        let (_, rem) = div_terms(&scaled, q)?;
        Ok(rem)
    }

    /// Euclid's loop over `pseudoremainder_terms` instead of the plain
    /// remainder: 2.96(a)'s answer, before 2.96(b)'s content removal.
    pub fn gcd_terms_raw(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SicpError> {
        if b.is_empty() {
            return Ok(a.to_vec());
        }
        let rem = pseudoremainder_terms(a, b)?;
        gcd_terms_raw(b, &rem)
    }

    fn gcd_i128(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        i128::try_from(a).unwrap_or(i128::MAX)
    }

    /// The (integer) content of a term list: the greatest common divisor
    /// of every coefficient, or 1 for the empty list (nothing to divide
    /// out).
    fn content(terms: &[Term]) -> Result<i128, SicpError> {
        let g = terms
            .iter()
            .try_fold(0i128, |g, t| Ok(gcd_i128(g, coeff_int(t)?)))?;
        Ok(if g == 0 { 1 } else { g })
    }

    /// Exercise 2.96(b): `gcd_terms_raw` divided through by its own
    /// coefficients' content, so the answer's coefficients are no larger
    /// than they need to be. The GCD is only defined up to a nonzero
    /// rational factor, so the sign is pinned to a positive leading
    /// coefficient -- the same convention the integer `gcd` follows.
    pub fn gcd_terms(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SicpError> {
        let raw = gcd_terms_raw(a, b)?;
        let c = content(&raw)?;
        let mut gcd: Vec<Term> = raw
            .iter()
            .map(|t| Ok(make_term(order(t), Value::Int(coeff_int(t)? / c))))
            .collect::<Result<_, _>>()?;
        let negative = match gcd.first() {
            Some(t) => coeff_int(t)? < 0,
            None => false,
        };
        if negative {
            gcd = negate(&gcd)?;
        }
        Ok(gcd)
    }

    pub fn as_pairs(terms: &[Term]) -> Result<Vec<(u32, i128)>, SicpError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }
}

mod ex_2_96 {
    use super::term_lists::{as_pairs, gcd_terms, gcd_terms_raw, mul_terms};
    use ch02::sec_2_5::make_term;
    use sicp_runtime::{SicpError, Value};

    /// The raw and content-reduced GCD of `Q_1` and `Q_2`.
    type Answer = (Vec<(u32, i128)>, Vec<(u32, i128)>);

    /// The same `Q_1`, `Q_2` from exercise 2.95, now run through the
    /// pseudodivision-based `gcd_terms`: part (a) alone produces integer
    /// coefficients larger than `P_1`'s; part (b)'s content removal
    /// brings the answer back down to `P_1` itself.
    pub fn ex_2_96() -> Result<Answer, SicpError> {
        let p1 = vec![
            make_term(2, Value::Int(1)),
            make_term(1, Value::Int(-2)),
            make_term(0, Value::Int(1)),
        ];
        let p2 = vec![make_term(2, Value::Int(11)), make_term(0, Value::Int(7))];
        let p3 = vec![make_term(1, Value::Int(13)), make_term(0, Value::Int(5))];
        let q1 = mul_terms(&p1, &p2)?;
        let q2 = mul_terms(&p1, &p3)?;
        let raw = gcd_terms_raw(&q1, &q2)?;
        let reduced = gcd_terms(&q1, &q2)?;
        Ok((as_pairs(&raw)?, as_pairs(&reduced)?))
    }
}

#[test]
fn ex_2_96() {
    let (raw, reduced) = ex_2_96::ex_2_96().expect("pseudodivision-based gcd of Q1, Q2");
    assert_eq!(raw, vec![(2, 1458), (1, -2916), (0, 1458)]);
    assert_eq!(reduced, vec![(2, 1), (1, -2), (0, 1)]);
}
