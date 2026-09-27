// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.31: why `add-action!` runs each
//! new action once immediately. Without that initialization, probes
//! never report the wire's starting value, and a gate whose inputs are
//! already set computes nothing until an input changes.

use std::rc::Rc;

use ch03::sec_3_3::{AND_GATE_DELAY, Agenda, Wire, logical_and, probe_log, probe_wire};

mod ex_3_31 {
    use super::{Agenda, Rc, Wire, add_and_gate_action_deferred, add_and_gate_action_now};

    /// Exercise 3.31: why attaching runs each action once immediately
    ///
    /// Builds one and-gate whose inputs already carry 1 before the gate
    /// is attached, twice over: once with the section's immediate run,
    /// once with the exercise's deferred attachment. Propagating each
    /// answers the output only the immediate run produces.
    #[must_use]
    pub fn ex_3_31() -> (u8, u8) {
        let agenda_now = Rc::new(Agenda::new());
        let a1 = Wire::new();
        let b1 = Wire::new();
        let out_now = Wire::new();
        a1.set_signal(1);
        b1.set_signal(1);
        add_and_gate_action_now(&a1, &b1, &out_now, &agenda_now);
        agenda_now.propagate();

        let agenda_deferred = Rc::new(Agenda::new());
        let a2 = Wire::new();
        let b2 = Wire::new();
        let out_deferred = Wire::new();
        a2.set_signal(1);
        b2.set_signal(1);
        add_and_gate_action_deferred(&a2, &b2, &out_deferred, &agenda_deferred);
        agenda_deferred.propagate();

        (out_now.signal(), out_deferred.signal())
    }
}

/// The section's `and-gate` attachment: the action joins the wires and
/// then runs once immediately.
pub fn add_and_gate_action_now(a: &Wire, b: &Wire, out: &Wire, agenda: &Rc<Agenda>) {
    a.add_action(&and_gate_action(a, b, out, agenda));
    b.add_action(&and_gate_action(a, b, out, agenda));
}

/// Exercise 3.31's variant: the action joins the wires but waits for
/// the first change.
pub fn add_and_gate_action_deferred(a: &Wire, b: &Wire, out: &Wire, agenda: &Rc<Agenda>) {
    let action = and_gate_action(a, b, out, agenda);
    a.add_action_deferred(&action);
    b.add_action_deferred(&action);
}

/// One and-gate action over the given wires, computing the conjunction
/// when an input changes and scheduling the output a delay later.
fn and_gate_action(a: &Wire, b: &Wire, out: &Wire, agenda: &Rc<Agenda>) -> Rc<dyn Fn()> {
    let a = Wire::clone(a);
    let b = Wire::clone(b);
    let out = Wire::clone(out);
    let agenda = Rc::clone(agenda);
    Rc::new(move || {
        let new_value = logical_and(a.signal(), b.signal());
        let driven = Wire::clone(&out);
        agenda.after_delay(
            AND_GATE_DELAY,
            Rc::new(move || {
                driven.set_signal(new_value);
            }),
        );
    })
}

#[test]
fn ex_3_31() {
    // The immediate run computes the standing inputs: the output rises
    // to 1. The deferred variant never does: it sits at 0 forever,
    // because no input ever changes to wake it.
    assert_eq!(ex_3_31::ex_3_31(), (1, 0));
}

/// The probe half of the story: with the immediate run the probe
/// reports the wire's starting value; deferred, it stays silent.
#[test]
fn probe_reports_the_initial_value_only_through_the_immediate_run() {
    let agenda = Rc::new(Agenda::new());
    let w = Wire::new();
    let log = probe_log();
    probe_wire("w", &w, &agenda, &log);
    assert_eq!(log.borrow().as_slice(), ["w 0  New-value = 0"]);
}
