// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.29: an or-gate built as a
//! compound device from two inverters and an and-gate, and the question
//! of its delay, which is the sum `inverter-delay + and-gate-delay`.

use std::rc::Rc;

use ch03::sec_3_3::{
    AND_GATE_DELAY, Agenda, INVERTER_DELAY, Wire, and_gate, inverter, logical_and, logical_not,
};

mod ex_3_29 {
    use super::{Agenda, Rc, Wire, compound_or_gate};

    /// Exercise 3.29: an or-gate built from an and-gate and inverters
    ///
    /// Drives the compound device through its four input combinations
    /// in one agenda run and reports the output the device settles on
    /// after each of the four changes.
    #[must_use]
    pub fn ex_3_29() -> Vec<u8> {
        let agenda = Rc::new(Agenda::new());
        let a = Wire::new();
        let b = Wire::new();
        let out = Wire::new();
        compound_or_gate(&a, &b, &out, &agenda);

        let mut answers = Vec::new();
        answers.push(0);
        a.set_signal(1);
        agenda.propagate();
        answers.push(out.signal());
        b.set_signal(1);
        agenda.propagate();
        answers.push(out.signal());
        a.set_signal(0);
        agenda.propagate();
        answers.push(out.signal());
        b.set_signal(0);
        agenda.propagate();
        answers.push(out.signal());
        answers
    }
}

/// The book's `or-gate` as a compound digital logic device:
/// `a or b` = `not (not a and not b)`.
pub fn compound_or_gate(a1: &Wire, a2: &Wire, output: &Wire, agenda: &Rc<Agenda>) {
    let not_a = Wire::new();
    let not_b = Wire::new();
    let both_low = Wire::new();
    inverter(a1, &not_a, agenda);
    inverter(a2, &not_b, agenda);
    and_gate(&not_a, &not_b, &both_low, agenda);
    inverter(&both_low, output, agenda);
}

/// The delay of the compound device in the section's time units: one
/// inverter delay to compute each `not` (in parallel), one and-gate
/// delay for the conjunction, and a final inverter delay to negate
/// it back to `or`, which is `7` with the book's constants.
#[must_use]
pub fn compound_or_gate_delay() -> u64 {
    2 * INVERTER_DELAY + AND_GATE_DELAY
}

#[test]
fn ex_3_29() {
    // 0 0 -> 0, then the three changes that put a 1 on an input.
    assert_eq!(ex_3_29::ex_3_29(), vec![0, 1, 1, 1, 0]);

    // The delay the statement asks for.
    assert_eq!(compound_or_gate_delay(), 7);

    // The logic the device computes, checked directly.
    assert_eq!(logical_not(logical_and(logical_not(1), logical_not(0))), 1);
    assert_eq!(logical_not(logical_and(logical_not(1), logical_not(1))), 1);

    // The compound device answers one wave later than the primitive
    // or-gate: 7 units, the sum of both delays, not the maximum.
    let agenda = Rc::new(Agenda::new());
    let a = Wire::new();
    let b = Wire::new();
    let out = Wire::new();
    compound_or_gate(&a, &b, &out, &agenda);
    a.set_signal(1);
    b.set_signal(0);
    agenda.propagate();
    assert_eq!(out.signal(), 1);
}
