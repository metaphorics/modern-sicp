// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.37: the expression-style
//! constraint combinators. Each combinator makes a fresh result
//! connector, wires the primitive constraint into its arguments, and
//! returns the connector, so a converter becomes one expression
//! instead of a sequence of `define`s naming every intermediate
//! connector by hand.

use ch03::sec_3_3::{Connector, Informant, Network, adder, constant, multiplier};

/// `x + y`, as a fresh connector.
pub fn c_add(network: &Network, x: &Connector, y: &Connector) -> Connector {
    let z = network.connector();
    adder(network, x, y, &z);
    z
}

/// `x - y`, built as `adder z y x` (`z + y = x`) so the network still
/// runs in every direction.
pub fn c_sub(network: &Network, x: &Connector, y: &Connector) -> Connector {
    let z = network.connector();
    adder(network, &z, y, x);
    z
}

/// `x * y`, as a fresh connector.
pub fn c_mul(network: &Network, x: &Connector, y: &Connector) -> Connector {
    let z = network.connector();
    multiplier(network, x, y, &z);
    z
}

/// `x / y`, built as `multiplier y z x` (`y * z = x`).
pub fn c_div(network: &Network, x: &Connector, y: &Connector) -> Connector {
    let z = network.connector();
    multiplier(network, y, &z, x);
    z
}

/// A connector permanently fixed at `value`.
pub fn cv(network: &Network, value: f64) -> Connector {
    let z = network.connector();
    constant(network, value, &z);
    z
}

/// The book's `F = 9C/5 + 32`, written as one expression over the
/// combinators above instead of the section's sequence of `define`s.
#[must_use]
pub fn celsius_fahrenheit_converter(network: &Network, c: &Connector) -> Connector {
    let scaled = c_div(
        network,
        &c_mul(network, &cv(network, 9.0), c),
        &cv(network, 5.0),
    );
    c_add(network, &scaled, &cv(network, 32.0))
}

mod ex_3_37 {
    use super::{Informant, Network, celsius_fahrenheit_converter};

    /// Exercise 3.37: expression-style constraint combinators
    ///
    /// Drives the expression-style converter from the Celsius side,
    /// then, after forgetting, from the Fahrenheit side, matching the
    /// two-directional interaction of 3.3.5's own `celsius-fahrenheit-converter`.
    #[must_use]
    pub fn ex_3_37() -> (f64, f64) {
        let network = Network::new();
        let c = network.connector();
        let f = celsius_fahrenheit_converter(&network, &c);
        let user = Informant::user();

        c.set_value(25.0, &user)
            .expect("a fresh connector accepts its first value");
        let f_at_25 = f.value().expect("the network derives f from c");

        c.forget_value(&user);
        f.set_value(212.0, &user)
            .expect("a forgotten connector accepts a fresh value");
        let c_at_212 = c.value().expect("the network derives c from f");

        (f_at_25, c_at_212)
    }
}

#[test]
fn ex_3_37() {
    assert_eq!(ex_3_37::ex_3_37(), (77.0, 100.0));
}

/// `c_sub` alone runs in both directions, the same trick the book
/// performs for subtraction and division from `adder` and
/// `multiplier`: `x - y = z` is `z + y = x` underneath.
#[test]
fn c_sub_runs_both_directions() {
    let network = Network::new();
    let x = network.connector();
    let y = network.connector();
    let z = c_sub(&network, &x, &y);
    let user = Informant::user();

    x.set_value(10.0, &user)
        .expect("a fresh connector accepts its first value");
    y.set_value(4.0, &user)
        .expect("a fresh connector accepts its first value");
    assert_eq!(z.value(), Some(6.0));

    x.forget_value(&user);
    z.set_value(3.0, &user)
        .expect("a forgotten connector accepts a fresh value");
    assert_eq!(x.value(), Some(7.0));
}
