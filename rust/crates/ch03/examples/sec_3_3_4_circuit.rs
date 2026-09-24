// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3.4

//! Section 3.3.4: the digital circuit simulator — the book's half-adder
//! sample simulation, with the probe transcript asserted line for line.

use std::rc::Rc;

use ch03::sec_3_3::{Agenda, Wire, half_adder, probe_log, probe_wire};

fn main() {
    // The book's setup: one agenda. The primitive delays are the
    // section's constants, inverter 2, and-gate 3, or-gate 5.
    let agenda = Rc::new(Agenda::new());

    // Four wires, probes on two of them. A probe runs once as it is
    // attached, which is why the initial zero already shows.
    let input_1 = Wire::new();
    let input_2 = Wire::new();
    let sum = Wire::new();
    let carry = Wire::new();
    let log = probe_log();

    probe_wire("sum", &sum, &agenda, &log);
    probe_wire("carry", &carry, &agenda, &log);

    // Connect the wires in a half-adder circuit and set input-1 to 1.
    half_adder(&input_1, &input_2, &sum, &carry, &agenda);
    input_1.set_signal(1);
    agenda.propagate();

    // The sum signal changes to 1 at time 8: five for the or-gate that
    // computed d, three more for the and-gate from d and e.
    input_2.set_signal(1);
    agenda.propagate();

    // The carry changes to 1 at time 11 and the sum falls to 0 at 16:
    // 11 for c (3 and-delay), 13 for e (2 inverter-delay), 16 for s.
    let lines = log.borrow();
    for line in lines.iter() {
        println!("{line}");
    }
    let expected = [
        "sum 0  New-value = 0",
        "carry 0  New-value = 0",
        "sum 8  New-value = 1",
        "carry 11  New-value = 1",
        "sum 16  New-value = 0",
    ];
    assert_eq!(lines.as_slice(), expected);
}
