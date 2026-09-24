// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.34: Louis Reasoner's squarer.
//! `multiplier a a b` wires the *same* connector into both factor
//! terminals, and the flaw is structural: `Multiplier::process_new_value`
//! only ever fires a branch once two of its three connectors are
//! known, and here the two factor terminals are one and the same
//! unset connector, so "one factor known" never actually holds.

use ch03::sec_3_3::{Connector, Informant, Network, multiplier};

/// Louis's proposed squarer: a bare `multiplier a a b`, wiring `a`
/// into both factor terminals of a single multiplier constraint.
pub fn louis_squarer(network: &Network, a: &Connector, b: &Connector) {
    multiplier(network, a, a, b);
}

mod ex_3_34 {
    use super::{Informant, Network, louis_squarer};

    /// Exercise 3.34: Louis's squarer from a bare multiplier is flawed
    ///
    /// Sets `b` alone and reads `a` back: the multiplier never
    /// computes it, because both its factor terminals name the one
    /// connector that is still unset.
    #[must_use]
    pub fn ex_3_34() -> Option<f64> {
        let network = Network::new();
        let a = network.connector();
        let b = network.connector();
        louis_squarer(&network, &a, &b);

        let user = Informant::user();
        b.set_value(25.0, &user)
            .expect("a fresh connector accepts its first value");
        a.value()
    }
}

#[test]
fn ex_3_34() {
    assert_eq!(ex_3_34::ex_3_34(), None);
}

/// The forward direction still works, because naming `a` gives the
/// multiplier two known connectors (both factor terminals, at once):
/// the flaw is one-directional, not total.
#[test]
fn forward_direction_still_works() {
    let network = Network::new();
    let a = network.connector();
    let b = network.connector();
    louis_squarer(&network, &a, &b);

    let user = Informant::user();
    a.set_value(5.0, &user)
        .expect("a fresh connector accepts its first value");
    assert_eq!(a.value(), Some(5.0));
    assert_eq!(b.value(), Some(25.0));
}
