// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3.5

//! Section 3.3.5: propagation of constraints — the Celsius-Fahrenheit
//! converter of `9C = 5(F - 32)`, driven from either end, with the
//! probe transcript asserted line for line.

use ch03::sec_3_3::{
    Connector, Informant, Network, celsius_fahrenheit_converter, probe_connector, probe_log,
};

fn main() {
    // Two named connectors, linked by the network of Figure 3.28.
    let network = Network::new();
    let c = Connector::new();
    let f = Connector::new();
    celsius_fahrenheit_converter(&network, &c, &f);

    // Probes on both ends, then the user sets C to 25.
    let log = probe_log();
    probe_connector(&network, "Celsius temp", &c, &log);
    probe_connector(&network, "Fahrenheit temp", &f, &log);

    let user = Informant::user();
    if c.set_value(25.0, &user).is_err() {
        return;
    }

    // Trying to set F to 212 now draws the book's contradiction: F is
    // already held at 77 by the network.
    let refused = match f.set_value(212.0, &user) {
        Ok(()) => return,
        Err(error) => error,
    };
    println!("{refused}");
    // => Contradiction (77 212)
    assert!(refused.to_string().starts_with("Contradiction"));

    // Forget C, watch both ends give up their values, then drive the
    // same network from the Fahrenheit side.
    c.forget_value(&user);
    if f.set_value(212.0, &user).is_err() {
        return;
    }

    let lines = log.borrow();
    for line in lines.iter() {
        println!("{line}");
    }
    let expected = [
        "Probe: Celsius temp = 25",
        "Probe: Fahrenheit temp = 77",
        "Probe: Celsius temp = ?",
        "Probe: Fahrenheit temp = ?",
        "Probe: Fahrenheit temp = 212",
        "Probe: Celsius temp = 100",
    ];
    assert_eq!(lines.as_slice(), expected);
}
