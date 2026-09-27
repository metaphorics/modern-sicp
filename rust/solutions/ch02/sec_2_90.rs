// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.90, extended by this edition's
//! tailored addition, exercise 2.90a.

mod ex_2_90 {
    /// Order/coefficient pairs, highest order first, zero coefficients
    /// dropped: the canonical form both representations are compared in.
    pub type Pairs = Vec<(u32, i128)>;

    /// The shape every term-list representation in this system must
    /// support: the book's `first-term`, `rest-terms`, `adjoin-term`, and
    /// `empty-termlist?`. `add_terms`/`mul_terms` below are written only
    /// against this trait, so the same algorithm runs unchanged over
    /// whichever representation a polynomial happens to carry.
    pub trait TermList: Clone {
        fn the_empty_termlist() -> Self;
        fn is_empty(&self) -> bool;
        fn first_term(&self) -> (u32, i128);
        fn rest_terms(&self) -> Self;
        fn adjoin_term(order: u32, coeff: i128, rest: Self) -> Self;
        fn from_pairs(pairs: &[(u32, i128)]) -> Self;
        /// The pairs this representation actually stores, in whatever
        /// order the representation walks them; this is what the
        /// exercise 2.90a benchmark counts.
        fn stored_pairs(&self) -> Pairs;
    }

    /// The book's original sparse list: only nonzero terms are stored.
    #[derive(Clone, Debug)]
    pub struct Sparse(Pairs);

    impl TermList for Sparse {
        fn the_empty_termlist() -> Self {
            Sparse(Vec::new())
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        fn first_term(&self) -> (u32, i128) {
            self.0[0]
        }

        fn rest_terms(&self) -> Self {
            Sparse(self.0[1..].to_vec())
        }

        fn adjoin_term(order: u32, coeff: i128, rest: Self) -> Self {
            if coeff == 0 {
                return rest;
            }
            let mut pairs = Vec::with_capacity(1 + rest.0.len());
            pairs.push((order, coeff));
            pairs.extend(rest.0);
            Sparse(pairs)
        }

        fn from_pairs(pairs: &[(u32, i128)]) -> Self {
            pairs
                .iter()
                .rev()
                .fold(Self::the_empty_termlist(), |acc, &(o, c)| {
                    Self::adjoin_term(o, c, acc)
                })
        }

        fn stored_pairs(&self) -> Pairs {
            self.0.clone()
        }
    }

    /// The dense list of exercise 2.89: every order down to 0 is stored,
    /// including zero coefficients.
    #[derive(Clone, Debug)]
    pub struct Dense {
        coeffs: Vec<i128>,
    }

    impl Dense {
        fn first_order(&self) -> u32 {
            u32::try_from(self.coeffs.len() - 1).expect("polynomial order fits in u32")
        }
    }

    impl TermList for Dense {
        fn the_empty_termlist() -> Self {
            Dense { coeffs: Vec::new() }
        }

        fn is_empty(&self) -> bool {
            self.coeffs.is_empty()
        }

        fn first_term(&self) -> (u32, i128) {
            (self.first_order(), self.coeffs[0])
        }

        fn rest_terms(&self) -> Self {
            Dense {
                coeffs: self.coeffs[1..].to_vec(),
            }
        }

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

        fn stored_pairs(&self) -> Pairs {
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

    /// The one `add_terms` the book asks for, written against the trait:
    /// it never inspects which representation `T` is.
    pub fn add_terms<T: TermList>(l1: &T, l2: &T) -> T {
        if l1.is_empty() {
            return l2.clone();
        }
        if l2.is_empty() {
            return l1.clone();
        }
        let (o1, c1) = l1.first_term();
        let (o2, c2) = l2.first_term();
        match o1.cmp(&o2) {
            std::cmp::Ordering::Greater => T::adjoin_term(o1, c1, add_terms(&l1.rest_terms(), l2)),
            std::cmp::Ordering::Less => T::adjoin_term(o2, c2, add_terms(l1, &l2.rest_terms())),
            std::cmp::Ordering::Equal => {
                T::adjoin_term(o1, c1 + c2, add_terms(&l1.rest_terms(), &l2.rest_terms()))
            }
        }
    }

    fn mul_term_by_all_terms<T: TermList>(order: u32, coeff: i128, l: &T) -> T {
        if l.is_empty() || coeff == 0 {
            return T::the_empty_termlist();
        }
        let (o, c) = l.first_term();
        T::adjoin_term(
            order + o,
            coeff * c,
            mul_term_by_all_terms(order, coeff, &l.rest_terms()),
        )
    }

    /// The one `mul_terms` the book asks for, also written against the
    /// trait. This, together with `add_terms`, is the redesign the
    /// exercise wants: install both sparse and dense term lists, and let
    /// the same generic operations serve either one — the polynomial
    /// system of 2.5.3 no longer has to choose.
    pub fn mul_terms<T: TermList>(l1: &T, l2: &T) -> T {
        if l1.is_empty() {
            return T::the_empty_termlist();
        }
        let (o, c) = l1.first_term();
        add_terms(
            &mul_term_by_all_terms(o, c, l2),
            &mul_terms(&l1.rest_terms(), l2),
        )
    }

    /// Sorts and drops zero coefficients: the canonical form the
    /// property test in 2.90a compares across representations.
    pub fn canonical(mut pairs: Pairs) -> Pairs {
        pairs.retain(|&(_, c)| c != 0);
        pairs.sort_by_key(|a| std::cmp::Reverse(a.0));
        pairs
    }

    /// `(x^2 + 3x + 7)` as a sparse list plus `(5x^2 + 3x)` as a dense
    /// list represent the same polynomials this system now supports in
    /// either shape; each representation's own generic `add_terms`
    /// produces the same answer, up to representation.
    pub fn ex_2_90() -> (Pairs, Pairs) {
        let sparse_sum = add_terms(
            &Sparse::from_pairs(&[(2, 1), (1, 3), (0, 7)]),
            &Sparse::from_pairs(&[(2, 5), (1, 3)]),
        );
        let dense_sum = add_terms(
            &Dense::from_pairs(&[(2, 1), (1, 3), (0, 7)]),
            &Dense::from_pairs(&[(2, 5), (1, 3)]),
        );
        (
            canonical(sparse_sum.stored_pairs()),
            canonical(dense_sum.stored_pairs()),
        )
    }
}

/// Exercise 2.90a: the sparse and dense packages must agree on every
/// answer, and the sparse package must skip the zero coefficients the
/// dense package has to materialize.
mod ex_2_90a {
    use super::ex_2_90::{Dense, Pairs, Sparse, TermList, canonical, mul_terms};

    /// A tiny fixed-seed linear congruential generator: deterministic
    /// across runs, with no external dependency, so the property test
    /// below is reproducible in `cargo nextest`.
    struct Lcg(u64);

    impl Lcg {
        fn next_u64(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            self.0
        }

        /// A small random polynomial: a handful of terms at random
        /// non-decreasing orders with random small coefficients.
        fn random_pairs(&mut self, term_count: u32) -> Pairs {
            let mut order = 0u32;
            let mut pairs = Vec::new();
            for _ in 0..term_count {
                order += 1 + u32::try_from(self.next_u64() % 4).expect("small modulus fits u32");
                let coeff = i128::from(self.next_u64() % 21) - 10;
                if coeff != 0 {
                    pairs.push((order, coeff));
                }
            }
            pairs.reverse();
            pairs
        }
    }

    fn agree(a: &[(u32, i128)], b: &[(u32, i128)]) -> (Pairs, Pairs) {
        let sparse = mul_terms(&Sparse::from_pairs(a), &Sparse::from_pairs(b));
        let dense = mul_terms(&Dense::from_pairs(a), &Dense::from_pairs(b));
        (
            canonical(sparse.stored_pairs()),
            canonical(dense.stored_pairs()),
        )
    }

    /// Runs the property over twenty random polynomial pairs: sparse and
    /// dense multiplication must land on the same canonical answer every
    /// time.
    pub fn property_test_agrees() -> bool {
        let mut rng = Lcg(0x2E9A_5B3C_1F07_D9E1);
        (0..20).all(|_| {
            let a = rng.random_pairs(4);
            let b = rng.random_pairs(4);
            let (sparse, dense) = agree(&a, &b);
            sparse == dense
        })
    }

    /// The book's own example: multiplying `x^100 + 1` by `x^100 - 1`
    /// (giving `x^200 - 1`), measuring how many terms each
    /// representation stores in the product.
    pub fn benchmark_x100_product() -> (usize, usize) {
        let a = vec![(100, 1i128), (0, 1)];
        let b = vec![(100, 1i128), (0, -1)];

        let sparse_product = mul_terms(&Sparse::from_pairs(&a), &Sparse::from_pairs(&b));
        let dense_product = mul_terms(&Dense::from_pairs(&a), &Dense::from_pairs(&b));

        (
            sparse_product.stored_pairs().len(),
            dense_product.stored_pairs().len(),
        )
    }
}

#[test]
fn ex_2_90() {
    let (sparse, dense) = ex_2_90::ex_2_90();
    let expected = vec![(2, 6), (1, 6), (0, 7)];
    assert_eq!(sparse, expected);
    assert_eq!(dense, expected);
}

#[test]
fn ex_2_90a_sparse_and_dense_agree() {
    assert!(
        ex_2_90a::property_test_agrees(),
        "sparse and dense multiplication must land on the same canonical polynomial"
    );
}

#[test]
fn ex_2_90a_sparse_skips_the_zeros_dense_must_store() {
    let (sparse_terms, dense_terms) = ex_2_90a::benchmark_x100_product();
    // x^200 - 1: exactly two nonzero terms, however either
    // representation gets there.
    assert_eq!(sparse_terms, 2);
    // The dense product spans every order from 200 down to 0.
    assert_eq!(dense_terms, 201);
    assert!(sparse_terms < dense_terms);
}
