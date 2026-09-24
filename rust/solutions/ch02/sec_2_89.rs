// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.89.

mod ex_2_89 {
    /// Order/coefficient pairs, highest order first.
    type Pairs = Vec<(u32, i128)>;

    /// A dense term list: one coefficient per order, from the highest
    /// order down to 0, with every zero coefficient stored explicitly.
    /// Order is never stored; position in the vector is the order.
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Dense {
        /// `coeffs[0]` is the highest-order term; `coeffs.len() - 1` is
        /// the constant term.
        coeffs: Vec<i128>,
    }

    impl Dense {
        fn the_empty_termlist() -> Self {
            Dense { coeffs: Vec::new() }
        }

        fn is_empty(&self) -> bool {
            self.coeffs.is_empty()
        }

        /// The order of the leading (first) term.
        fn first_order(&self) -> u32 {
            u32::try_from(self.coeffs.len() - 1).expect("polynomial order fits in u32")
        }

        /// `(order, coeff)` of the first (highest-order) term.
        fn first_term(&self) -> (u32, i128) {
            (self.first_order(), self.coeffs[0])
        }

        /// Every term but the first: the book's `rest-terms`, which
        /// drops the leading position outright, exactly like the sparse
        /// package's `cdr`.
        fn rest_terms(&self) -> Self {
            Dense {
                coeffs: self.coeffs[1..].to_vec(),
            }
        }

        /// Conses a term onto the front, filling any gap between the new
        /// term's order and the list's current highest order with
        /// explicit zero coefficients. This is where a dense list pays
        /// for its density: those gap zeros are stored, never skipped.
        fn adjoin_term(order: u32, coeff: i128, rest: Self) -> Self {
            let next_order = if rest.is_empty() {
                0
            } else {
                rest.first_order() + 1
            };
            let gap =
                usize::try_from(order - next_order).expect("term list order is non-decreasing");
            let mut coeffs = Vec::with_capacity(1 + gap + rest.coeffs.len());
            coeffs.push(coeff);
            coeffs.extend(std::iter::repeat_n(0, gap));
            coeffs.extend(rest.coeffs);
            Dense { coeffs }
        }

        fn from_pairs(pairs: &[(u32, i128)]) -> Self {
            pairs
                .iter()
                .rev()
                .fold(Self::the_empty_termlist(), |acc, &(o, c)| {
                    Self::adjoin_term(o, c, acc)
                })
        }

        /// The materialized order/coefficient pairs, gaps included: this
        /// is the size the exercise's dense representation actually pays
        /// for, unlike the sparse list which never stores a zero.
        fn to_pairs(&self) -> Vec<(u32, i128)> {
            self.coeffs
                .iter()
                .enumerate()
                .map(|(i, &c)| {
                    (
                        self.first_order() - u32::try_from(i).expect("index fits in u32"),
                        c,
                    )
                })
                .collect()
        }
    }

    /// `add_terms`/`mul_terms` written only in terms of `first_term`,
    /// `rest_terms`, `adjoin_term`, and `is_empty`: the same algorithm
    /// the sparse package uses, working unchanged over the dense
    /// representation because it never assumes how a term list stores
    /// its zeros.
    fn add_terms(l1: &Dense, l2: &Dense) -> Dense {
        if l1.is_empty() {
            return l2.clone();
        }
        if l2.is_empty() {
            return l1.clone();
        }
        let (o1, c1) = l1.first_term();
        let (o2, c2) = l2.first_term();
        match o1.cmp(&o2) {
            std::cmp::Ordering::Greater => {
                Dense::adjoin_term(o1, c1, add_terms(&l1.rest_terms(), l2))
            }
            std::cmp::Ordering::Less => Dense::adjoin_term(o2, c2, add_terms(l1, &l2.rest_terms())),
            std::cmp::Ordering::Equal => {
                Dense::adjoin_term(o1, c1 + c2, add_terms(&l1.rest_terms(), &l2.rest_terms()))
            }
        }
    }

    fn mul_term_by_all_terms(order: u32, coeff: i128, l: &Dense) -> Dense {
        if l.is_empty() || coeff == 0 {
            return Dense::the_empty_termlist();
        }
        let (o, c) = l.first_term();
        Dense::adjoin_term(
            order + o,
            coeff * c,
            mul_term_by_all_terms(order, coeff, &l.rest_terms()),
        )
    }

    fn mul_terms(l1: &Dense, l2: &Dense) -> Dense {
        if l1.is_empty() {
            return Dense::the_empty_termlist();
        }
        let (o, c) = l1.first_term();
        add_terms(
            &mul_term_by_all_terms(o, c, l2),
            &mul_terms(&l1.rest_terms(), l2),
        )
    }

    /// `(x^2 + 3x + 7) + (5x^2 + 3x)`, `(x + 1)(x - 1)`, and a term list
    /// with a genuine internal gap (`x^3 + 1`, skipping orders 2 and 1),
    /// run through selectors and constructors built for the dense
    /// representation, using the same term-list algorithm the sparse
    /// package uses.
    pub fn ex_2_89() -> (Pairs, Pairs, Pairs) {
        let p1 = Dense::from_pairs(&[(2, 1), (1, 3), (0, 7)]);
        let p2 = Dense::from_pairs(&[(2, 5), (1, 3)]);
        let sum = add_terms(&p1, &p2).to_pairs();

        let a = Dense::from_pairs(&[(1, 1), (0, 1)]);
        let b = Dense::from_pairs(&[(1, 1), (0, -1)]);
        let product = mul_terms(&a, &b).to_pairs();

        let gapped = Dense::from_pairs(&[(3, 1), (0, 1)]).to_pairs();

        (sum, product, gapped)
    }
}

#[test]
fn ex_2_89() {
    let (sum, product, gapped) = ex_2_89::ex_2_89();
    assert_eq!(sum, vec![(2, 6), (1, 6), (0, 7)]);
    assert_eq!(product, vec![(2, 1), (1, 0), (0, -1)]);
    // The dense representation materializes the two zero-coefficient
    // gap terms that a sparse list would never store.
    assert_eq!(gapped, vec![(3, 1), (2, 0), (1, 0), (0, 1)]);
}
