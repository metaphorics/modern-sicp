// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.1.2

//! Section 2.1.2: abstraction barriers, illustrated by an alternate
//! rational-number representation that defers the `gcd` reduction from
//! construction time to access time.
//!
//! This is a separate, self-contained pair of functions rather than a
//! second constructor on `ch02::sec_2_1::Rational`, since the whole point
//! is that `add_rat`, `sub_rat`, and the rest of the package above the
//! `numer`/`denom`/`make-rat` barrier do not change at all when the
//! representation below the barrier changes.

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The alternate `make-rat` of this subsection: stores the arguments
/// unreduced.
fn make_rat_lazy(n: i128, d: i128) -> (i128, i128) {
    (n, d)
}

/// The alternate `numer`: divides out the `gcd` at access time.
fn numer_lazy((n, d): (i128, i128)) -> i128 {
    n / gcd(n, d)
}

/// The alternate `denom`: divides out the `gcd` at access time.
fn denom_lazy((n, d): (i128, i128)) -> i128 {
    d / gcd(n, d)
}

fn main() {
    // Built from 6 and 9, unreduced at construction time.
    let six_ninths = make_rat_lazy(6, 9);

    let numer = numer_lazy(six_ninths);
    let denom = denom_lazy(six_ninths);
    println!("{numer}/{denom}");
    // => 2/3
    assert_eq!((numer, denom), (2, 3));

    // The two representations agree on every rational number; only when
    // the `gcd` step runs differs, not what `numer` and `denom` report to
    // the programs above the barrier.
    let eighth_of_a_ton = make_rat_lazy(100, 800);
    let eighth_numer = numer_lazy(eighth_of_a_ton);
    let eighth_denom = denom_lazy(eighth_of_a_ton);
    println!("{eighth_numer}/{eighth_denom}");
    // => 1/8
    assert_eq!((eighth_numer, eighth_denom), (1, 8));
}
