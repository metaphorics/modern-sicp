// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.95.

mod term_lists {
    //! `gcd_terms` from exercise 2.94, restated here (plus a full
    //! `mul_terms` it did not need) so this exercise can run it on a
    //! bigger example.
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

    fn add_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
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

    /// Full term-list multiplication: distribute one term list over the
    /// other and add the partial products. Exercise 2.94 never needed
    /// this (it only divided); building `Q_1` and `Q_2` here does.
    pub fn mul_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
        match l1.first() {
            None => Ok(Vec::new()),
            Some(t1) => {
                let head = mul_term_by_all_terms(t1, l2)?;
                add_terms(&head, &mul_terms(&l1[1..], l2)?)
            }
        }
    }

    /// Long division on term lists (exercise 2.91), unmodified: this is
    /// exactly the version that requires every quotient coefficient to
    /// divide evenly.
    fn div_terms(
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

    /// Exercise 2.94's Euclidean loop, unmodified: `gcd(a, 0) = a`,
    /// otherwise `gcd(a, b) = gcd(b, remainder_terms(a, b))`.
    pub fn gcd_terms(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SchemeError> {
        if b.is_empty() {
            return Ok(a.to_vec());
        }
        let (_, rem) = div_terms(a, b)?;
        gcd_terms(b, &rem)
    }

    pub fn as_pairs(terms: &[Term]) -> Result<Vec<(u32, i128)>, SchemeError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }
}

mod ex_2_95 {
    use super::term_lists::{as_pairs, gcd_terms, mul_terms};
    use ch02::sec_2_5::make_term;
    use sicp_runtime::{SchemeError, Value};

    /// The two products, and the message `gcd_terms` fails with.
    type Answer = (Vec<(u32, i128)>, Vec<(u32, i128)>, String);

    /// `P_1 = x^2 - 2x + 1`, `P_2 = 11x^2 + 7`, `P_3 = 13x + 5`;
    /// `Q_1 = P_1 P_2`, `Q_2 = P_1 P_3`.
    ///
    /// With exact rational arithmetic `gcd_terms(Q_1, Q_2)` divides out
    /// to something proportional to `P_1`. This edition's `div_terms`
    /// insists on an exact integer quotient at every step, so instead of
    /// silently drifting to rational coefficients the very first
    /// division it tries -- `Q_1`'s leading coefficient 11 by `Q_2`'s
    /// leading coefficient 13 -- fails outright. That failure is this
    /// edition's own shape of "we may fail to get a valid divisor."
    ///
    /// # Errors
    /// Whatever building `Q_1` and `Q_2` raises (it does not, on this
    /// input); `gcd_terms`'s own failure is captured, not propagated.
    pub fn ex_2_95() -> Result<Answer, SchemeError> {
        let p1 = vec![
            make_term(2, Value::Int(1)),
            make_term(1, Value::Int(-2)),
            make_term(0, Value::Int(1)),
        ];
        let p2 = vec![make_term(2, Value::Int(11)), make_term(0, Value::Int(7))];
        let p3 = vec![make_term(1, Value::Int(13)), make_term(0, Value::Int(5))];
        let q1 = mul_terms(&p1, &p2)?;
        let q2 = mul_terms(&p1, &p3)?;
        let failure = match gcd_terms(&q1, &q2) {
            Ok(g) => format!("gcd_terms unexpectedly succeeded: {:?}", as_pairs(&g)?),
            Err(e) => e.to_string(),
        };
        Ok((as_pairs(&q1)?, as_pairs(&q2)?, failure))
    }
}

#[test]
fn ex_2_95() {
    let (q1, q2, failure) =
        ex_2_95::ex_2_95().expect("Q1 and Q2 build fine even though their gcd does not");
    assert_eq!(q1, vec![(4, 11), (3, -22), (2, 18), (1, -14), (0, 7)]);
    assert_eq!(q2, vec![(3, 13), (2, -21), (1, 3), (0, 5)]);
    assert_eq!(
        failure,
        "type mismatch: div_terms: 11 does not divide evenly by 13"
    );
}
