// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.33: the averager. `a + b = s`
//! feeds one factor of a multiplier whose other factor is a constant
//! 2, and the product is constrained equal to `c`, using the
//! section's own `adder`, `multiplier`, and `constant` primitives
//! unchanged.

use ch03::sec_3_3::{Connector, Informant, Network, adder, constant, multiplier};

/// The book's `averager a b c`: `c = (a + b) / 2`, built from a sum
/// connector and a constant-2 half connector so the multiplier can run
/// in either direction.
pub fn averager(network: &Network, a: &Connector, b: &Connector, c: &Connector) {
    let sum = network.connector();
    let half = network.connector();
    adder(network, a, b, &sum);
    multiplier(network, c, &half, &sum);
    constant(network, 2.0, &half);
}

mod ex_3_33 {
    use super::{Informant, Network, averager};

    /// Exercise 3.33: an averager constraint from adder and multiplier
    ///
    /// Sets `a` and `b`, reads the derived average off `c`; then, in
    /// a second direction, forgets both, sets `c` and `a`, and reads
    /// the constraint deriving the missing `b`.
    #[must_use]
    pub fn ex_3_33() -> (f64, f64) {
        let network = Network::new();
        let a = network.connector();
        let b = network.connector();
        let c = network.connector();
        averager(&network, &a, &b, &c);

        let user = Informant::user();
        a.set_value(6.0, &user)
            .expect("a fresh connector accepts its first value");
        b.set_value(14.0, &user)
            .expect("a fresh connector accepts its first value");
        let average = c.value().expect("the constraint derives the average");

        a.forget_value(&user);
        b.forget_value(&user);
        c.set_value(12.0, &user)
            .expect("a forgotten connector accepts a fresh value");
        a.set_value(4.0, &user)
            .expect("a forgotten connector accepts a fresh value");
        let derived_b = b
            .value()
            .expect("the constraint derives the missing addend");

        (average, derived_b)
    }
}

#[test]
fn ex_3_33() {
    assert_eq!(ex_3_33::ex_3_33(), (10.0, 20.0));
}

/// The same network is used to compute either direction within one
/// run, which is the constraint-propagation point the whole 3.3.5
/// subsection makes: nothing about `averager` names a preferred
/// input.
#[test]
fn one_network_computes_both_directions() {
    let network = Network::new();
    let a = network.connector();
    let b = network.connector();
    let c = network.connector();
    averager(&network, &a, &b, &c);

    let user = Informant::user();
    c.set_value(9.0, &user).expect("no contradiction");
    b.set_value(11.0, &user).expect("no contradiction");
    assert_eq!(a.value(), Some(7.0));
}
