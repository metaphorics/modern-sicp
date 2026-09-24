// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.97.

mod term_lists {
    //! Exercise 2.96's pseudodivision-based `gcd_terms`, plus 2.97's
    //! `reduce_terms` built on top of it.
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

    /// Full term-list multiplication, needed here to build the sum of
    /// two rational functions: `n1 d2 + n2 d1` over `d1 d2`.
    pub fn mul_terms(l1: &[Term], l2: &[Term]) -> Result<Vec<Term>, SchemeError> {
        match l1.first() {
            None => Ok(Vec::new()),
            Some(t1) => {
                let head = mul_term_by_all_terms(t1, l2)?;
                add_terms(&head, &mul_terms(&l1[1..], l2)?)
            }
        }
    }

    fn scale_terms(terms: &[Term], factor: i128) -> Result<Vec<Term>, SchemeError> {
        terms
            .iter()
            .map(|t| Ok(make_term(order(t), Value::Int(coeff_int(t)? * factor))))
            .collect()
    }

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

    fn integerizing_factor(p: &[Term], q: &[Term]) -> Result<Option<i128>, SchemeError> {
        let (Some(pt), Some(qt)) = (p.first(), q.first()) else {
            return Ok(None);
        };
        if order(pt) < order(qt) {
            return Ok(None);
        }
        let exp = 1 + i64::from(order(pt)) - i64::from(order(qt));
        let exp = u32::try_from(exp).map_err(|_| SchemeError::Overflow)?;
        let c = coeff_int(qt)?;
        Ok(Some(c.checked_pow(exp).ok_or(SchemeError::Overflow)?))
    }

    fn pseudoremainder_terms(p: &[Term], q: &[Term]) -> Result<Vec<Term>, SchemeError> {
        let Some(factor) = integerizing_factor(p, q)? else {
            return Ok(p.to_vec());
        };
        let scaled = scale_terms(p, factor)?;
        let (_, rem) = div_terms(&scaled, q)?;
        Ok(rem)
    }

    fn gcd_terms_raw(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SchemeError> {
        if b.is_empty() {
            return Ok(a.to_vec());
        }
        let rem = pseudoremainder_terms(a, b)?;
        gcd_terms_raw(b, &rem)
    }

    /// Euclid's `gcd` on plain integers, exactly as `reduce_integers`
    /// below needs it.
    pub fn gcd_i128(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        i128::try_from(a).unwrap_or(i128::MAX)
    }

    fn content(terms: &[Term]) -> Result<i128, SchemeError> {
        let g = terms
            .iter()
            .try_fold(0i128, |g, t| Ok(gcd_i128(g, coeff_int(t)?)))?;
        Ok(if g == 0 { 1 } else { g })
    }

    /// Exercise 2.96's fully reduced `gcd_terms`: pseudodivision, then
    /// content removed from the answer's own coefficients. The GCD is
    /// only defined up to a nonzero rational factor, so the sign is
    /// pinned to a positive leading coefficient -- without this the
    /// integerizing factor of `reduce_terms` can inherit a minus sign
    /// and flip both reduced parts.
    fn gcd_terms(a: &[Term], b: &[Term]) -> Result<Vec<Term>, SchemeError> {
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

    /// Exercise 2.97(a): `n` and `d` reduced to lowest terms. Multiplies
    /// both by the integerizing factor built from the GCD's leading
    /// coefficient and order, divides both by the GCD (exactly, by
    /// construction), then removes the redundant common factor left in
    /// the numerator and denominator together.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when `n` and `d` are both empty (no
    /// order to reduce by); whatever the term-list arithmetic raises.
    pub fn reduce_terms(n: &[Term], d: &[Term]) -> Result<(Vec<Term>, Vec<Term>), SchemeError> {
        let gcd = gcd_terms(n, d)?;
        let Some(leading) = gcd.first() else {
            return Err(SchemeError::TypeMismatch(
                "reduce_terms: n and d have no nonzero common divisor".into(),
            ));
        };
        let o1 = std::cmp::max(n.first().map_or(0, order), d.first().map_or(0, order));
        let o2 = order(leading);
        let exp =
            u32::try_from(1 + i64::from(o1) - i64::from(o2)).map_err(|_| SchemeError::Overflow)?;
        let factor = coeff_int(leading)?
            .checked_pow(exp)
            .ok_or(SchemeError::Overflow)?;
        let (nn, _) = div_terms(&scale_terms(n, factor)?, &gcd)?;
        let (dd, _) = div_terms(&scale_terms(d, factor)?, &gcd)?;
        let redundant = content(&nn.iter().chain(&dd).cloned().collect::<Vec<_>>())?;
        let shrink = |terms: &[Term]| -> Result<Vec<Term>, SchemeError> {
            terms
                .iter()
                .map(|t| Ok(make_term(order(t), Value::Int(coeff_int(t)? / redundant))))
                .collect()
        };
        Ok((shrink(&nn)?, shrink(&dd)?))
    }

    pub fn as_pairs(terms: &[Term]) -> Result<Vec<(u32, i128)>, SchemeError> {
        terms
            .iter()
            .map(|t| Ok((order(t), coeff_int(t)?)))
            .collect()
    }
}

mod ex_2_97 {
    use super::term_lists::{add_terms, as_pairs, gcd_i128, mul_terms, reduce_terms};
    use ch02::sec_2_5::{Term, make_term};
    use sicp_runtime::{SchemeError, Value};

    /// A polynomial in one variable, the shape `reduce_poly` works over:
    /// stripped of its variable, `reduce_terms` does the arithmetic, and
    /// the variable is reattached to build the answer back up.
    struct Poly {
        var: &'static str,
        terms: Vec<Term>,
    }

    /// Exercise 2.97(a)'s `reduce_poly`: analogous to `add_poly`, checks
    /// the two polys share a variable, then defers to `reduce_terms`.
    fn reduce_poly(p: &Poly, q: &Poly) -> Result<(Poly, Poly), SchemeError> {
        if p.var != q.var {
            return Err(SchemeError::TypeMismatch(format!(
                "reduce_poly: polys not in the same variable: {} {}",
                p.var, q.var
            )));
        }
        let (nn, dd) = reduce_terms(&p.terms, &q.terms)?;
        Ok((
            Poly {
                var: p.var,
                terms: nn,
            },
            Poly {
                var: q.var,
                terms: dd,
            },
        ))
    }

    /// Exercise 2.97(b)'s `reduce_integers`, the texi's own snippet
    /// verbatim, with this file's `gcd_i128` playing the book's `gcd`.
    fn reduce_integers(n: i128, d: i128) -> Result<(i128, i128), SchemeError> {
        if d == 0 {
            return Err(SchemeError::DivisionByZero);
        }
        let g = gcd_i128(n, d);
        Ok((n / g, d / g))
    }

    /// Either side `reduce` can dispatch on: a plain integer or a
    /// polynomial, standing in for the book's `scheme-number` and
    /// `polynomial` tags.
    enum Numeric {
        Int(i128),
        Poly(Poly),
    }

    fn numeric_tag(n: &Numeric) -> &'static str {
        match n {
            Numeric::Int(_) => "scheme-number",
            Numeric::Poly(_) => "polynomial",
        }
    }

    /// Exercise 2.97(b)'s generic `reduce`: dispatches to `reduce_poly`
    /// or `reduce_integers` by the shape of its arguments. The book
    /// dispatches at runtime through `apply_generic` and a shared table;
    /// here the two packages this exercise needs are not installed on
    /// one, so a closed `enum` plus `match` plays the same role, checked
    /// exhaustively at compile time instead of looked up at run time.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] naming both operands' tags when `n`
    /// and `d` are not the same shape; whatever `reduce_poly` or
    /// `reduce_integers` raises.
    fn reduce(n: Numeric, d: Numeric) -> Result<(Numeric, Numeric), SchemeError> {
        match (n, d) {
            (Numeric::Int(n), Numeric::Int(d)) => {
                let (nn, dd) = reduce_integers(n, d)?;
                Ok((Numeric::Int(nn), Numeric::Int(dd)))
            }
            (Numeric::Poly(n), Numeric::Poly(d)) => {
                let (nn, dd) = reduce_poly(&n, &d)?;
                Ok((Numeric::Poly(nn), Numeric::Poly(dd)))
            }
            (n, d) => Err(SchemeError::TypeMismatch(format!(
                "reduce: no method for these types: ({} {})",
                numeric_tag(&n),
                numeric_tag(&d)
            ))),
        }
    }

    /// What the original `make_rat` did for integers, generalized: calls
    /// `reduce` before the numerator and denominator are combined into a
    /// rational function.
    fn make_rational_fn(n: Poly, d: Poly) -> Result<(Poly, Poly), SchemeError> {
        match reduce(Numeric::Poly(n), Numeric::Poly(d))? {
            (Numeric::Poly(nn), Numeric::Poly(dd)) => Ok((nn, dd)),
            _ => Err(SchemeError::TypeMismatch(
                "reduce: expected polynomials back".into(),
            )),
        }
    }

    /// `n1/d1 + n2/d2 = (n1 d2 + n2 d1) / (d1 d2)`, reduced through
    /// `make_rational_fn` the way the section's rational package always
    /// reduced integer fractions.
    fn add_rational_fns(a: (Poly, Poly), b: (Poly, Poly)) -> Result<(Poly, Poly), SchemeError> {
        let (n1, d1) = a;
        let (n2, d2) = b;
        let numer = Poly {
            var: n1.var,
            terms: add_terms(
                &mul_terms(&n1.terms, &d2.terms)?,
                &mul_terms(&n2.terms, &d1.terms)?,
            )?,
        };
        let denom = Poly {
            var: d1.var,
            terms: mul_terms(&d1.terms, &d2.terms)?,
        };
        make_rational_fn(numer, denom)
    }

    /// The reduced `(6, 8)` pair, and the final reduced numerator and
    /// denominator of the extended exercise's rational-function sum.
    type Answer = ((i128, i128), Vec<(u32, i128)>, Vec<(u32, i128)>);

    /// `reduce(6, 8)` through the generic dispatcher, and the extended
    /// exercise's own example: `(x+1)/(x^3-1) + x/(x^2-1)`, which should
    /// come out reduced to lowest terms.
    pub fn ex_2_97() -> Result<Answer, SchemeError> {
        let integers = match reduce(Numeric::Int(6), Numeric::Int(8))? {
            (Numeric::Int(n), Numeric::Int(d)) => (n, d),
            _ => {
                return Err(SchemeError::TypeMismatch(
                    "reduce: expected integers back".into(),
                ));
            }
        };

        let p1 = Poly {
            var: "x",
            terms: vec![make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
        };
        let p2 = Poly {
            var: "x",
            terms: vec![make_term(3, Value::Int(1)), make_term(0, Value::Int(-1))],
        };
        let p3 = Poly {
            var: "x",
            terms: vec![make_term(1, Value::Int(1))],
        };
        let p4 = Poly {
            var: "x",
            terms: vec![make_term(2, Value::Int(1)), make_term(0, Value::Int(-1))],
        };

        let rf1 = make_rational_fn(p1, p2)?;
        let rf2 = make_rational_fn(p3, p4)?;
        let (numer, denom) = add_rational_fns(rf1, rf2)?;

        Ok((integers, as_pairs(&numer.terms)?, as_pairs(&denom.terms)?))
    }
}

#[test]
fn ex_2_97() {
    let (integers, numer, denom) =
        ex_2_97::ex_2_97().expect("reduce dispatches to integers and to polynomials alike");
    assert_eq!(integers, (3, 4));
    // (x + 1)/(x^3 - 1) + x/(x^2 - 1)
    //   = (x^3 + 2x^2 + 3x + 1) / (x^4 + x^3 - x - 1), reduced to lowest terms.
    assert_eq!(numer, vec![(3, 1), (2, 2), (1, 3), (0, 1)]);
    assert_eq!(denom, vec![(4, 1), (3, 1), (1, -1), (0, -1)]);
}
