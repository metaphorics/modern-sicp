// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.30: the ripple-carry adder,
//! built by stringing together `n` full-adders so the carry of one
//! stage feeds the next, and checked against integer addition.

use std::rc::Rc;

use ch03::sec_3_3::{Agenda, Wire, full_adder};

mod ex_3_30 {
    use super::{Agenda, Rc, Wire, read_bits, ripple_carry_adder, set_bits};
    use ch03::sec_3_3::probe_log;

    /// Exercise 3.30: a ripple-carry adder over n full-adders
    ///
    /// Builds a 4-bit adder, sets A to 11 and B to 6, propagates, and
    /// reports the sum bits and the final carry the probes log: the 4
    /// bits of 17 plus the overflow bit.
    #[must_use]
    pub fn ex_3_30() -> (u8, u8) {
        let agenda = Rc::new(Agenda::new());
        let bits = 4;
        let a: Vec<Wire> = (0..bits).map(|_| Wire::new()).collect();
        let b: Vec<Wire> = (0..bits).map(|_| Wire::new()).collect();
        let s: Vec<Wire> = (0..bits).map(|_| Wire::new()).collect();
        let carry_out = Wire::new();
        let c_in = Wire::new();
        let _log = probe_log();

        ripple_carry_adder(&a, &b, &s, &carry_out, &c_in, &agenda);
        set_bits(&a, 11);
        set_bits(&b, 6);
        agenda.propagate();

        (read_bits(&s), carry_out.signal())
    }
}

/// Sets the bits of one operand, least significant first.
fn set_bits(wires: &[Wire], value: u8) {
    for (index, wire) in wires.iter().enumerate() {
        let bit = u8::from((value >> index) & 1 == 1);
        wire.set_signal(bit);
    }
}

/// Reads the bits of one operand, least significant first.
#[must_use]
pub fn read_bits(wires: &[Wire]) -> u8 {
    let mut value = 0u8;
    for (index, wire) in wires.iter().enumerate() {
        value |= wire.signal() << index;
    }
    value
}

/// The book's `ripple-carry-adder`: `n` full-adders chained through
/// their carries, starting from `c_in` and ending at the carry-out
/// wire the caller names. `c_in` is the caller's own wire, held for
/// as long as the circuit needs to answer future signal changes: a
/// wire created and dropped inside this function would leave stage
/// zero's gates holding a dead weak reference, silently deaf to any
/// later change on `b`'s low bit (the regression test below is what
/// caught it: 255 + 1 never carried because stage zero could no
/// longer see the constant-zero carry-in that both its gates read).
pub fn ripple_carry_adder(
    a: &[Wire],
    b: &[Wire],
    s: &[Wire],
    c_out: &Wire,
    c_in: &Wire,
    agenda: &Rc<Agenda>,
) {
    let mut carry_in = Wire::clone(c_in);
    for stage in 0..a.len() {
        let carry_to = if stage + 1 == a.len() {
            Wire::clone(c_out)
        } else {
            Wire::new()
        };
        full_adder(
            &a[stage], &b[stage], &carry_in, &s[stage], &carry_to, agenda,
        );
        carry_in = carry_to;
    }
}

#[test]
fn ex_3_30() {
    // 11 + 6 = 17: 4 sum bits 0001 and a carry of 1.
    assert_eq!(ex_3_30::ex_3_30(), (0b0001, 1));
}

#[test]
fn ripple_adder_matches_integer_addition() {
    for (x, y) in [(0u8, 0u8), (13, 6), (100, 100), (255, 1), (42, 42)] {
        let agenda = Rc::new(Agenda::new());
        let a: Vec<Wire> = (0..8).map(|_| Wire::new()).collect();
        let b: Vec<Wire> = (0..8).map(|_| Wire::new()).collect();
        let s: Vec<Wire> = (0..8).map(|_| Wire::new()).collect();
        let c_out = Wire::new();
        let c_in = Wire::new();
        ripple_carry_adder(&a, &b, &s, &c_out, &c_in, &agenda);
        set_bits(&a, x);
        set_bits(&b, y);
        agenda.propagate();
        let total = u16::from(read_bits(&s)) + (u16::from(c_out.signal()) << 8);
        assert_eq!(total, u16::from(x) + u16::from(y), "{x} + {y}");
    }
}
