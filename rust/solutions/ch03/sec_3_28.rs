// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.28: an or-gate as a primitive
//! function box. The answer is the section's `or_gate`, which has the
//! same shape the and-gate has; the probe transcript shows it answering
//! every input combination.

use std::rc::Rc;

use ch03::sec_3_3::{Agenda, OR_GATE_DELAY, Wire, or_gate, probe_log, probe_wire};

mod ex_3_28 {
    use super::{Agenda, Rc, Wire, or_gate, probe_log, probe_wire};

    /// Exercise 3.28: an or-gate as a primitive function box
    ///
    /// Wires one or-gate, drives its inputs through every combination,
    /// and reports the output the probes log at the end of each run.
    #[must_use]
    pub fn ex_3_28() -> Vec<String> {
        let agenda = Rc::new(Agenda::new());
        let a = Wire::new();
        let b = Wire::new();
        let out = Wire::new();
        or_gate(&a, &b, &out, &agenda);
        let log = probe_log();
        probe_wire("out", &out, &agenda, &log);

        a.set_signal(1);
        agenda.propagate();
        b.set_signal(1);
        agenda.propagate();
        a.set_signal(0);
        agenda.propagate();
        b.set_signal(0);
        agenda.propagate();

        log.borrow().clone()
    }
}

#[test]
fn ex_3_28() {
    // The initial probe line, then only the changes the output truly
    // takes, one or-gate delay after the input change that caused
    // them: an or-gate output rises on the first high input and falls
    // only when both inputs fall, so the middle combinations leave it
    // at 1 and its probe silent.
    assert_eq!(
        ex_3_28::ex_3_28(),
        vec![
            "out 0  New-value = 0",
            "out 5  New-value = 1",
            "out 20  New-value = 0",
        ]
    );

    // The delay is the or-gate's, not the and-gate's.
    assert_eq!(OR_GATE_DELAY, 5);

    // The truth table in one gate: an output rises when either input
    // does, and falls only when both fall.
    let agenda = Rc::new(Agenda::new());
    let a = Wire::new();
    let b = Wire::new();
    let out = Wire::new();
    or_gate(&a, &b, &out, &agenda);
    a.set_signal(1);
    b.set_signal(1);
    agenda.propagate();
    assert_eq!(out.signal(), 1);
    a.set_signal(0);
    agenda.propagate();
    assert_eq!(out.signal(), 1);
    b.set_signal(0);
    agenda.propagate();
    assert_eq!(out.signal(), 0);
}
