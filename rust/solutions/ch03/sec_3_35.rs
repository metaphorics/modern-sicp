// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.35: the squarer as a genuine
//! new primitive constraint, filling in Ben Bitdiddle's outline: the
//! missing alternative of `process-new-value`, the body of
//! `process-forget-value`, the body of the `me` dispatch, and the
//! `connect` calls. Built the same way the section builds `adder`,
//! `multiplier`, and `constant`: a `me` object implementing
//! [`Constraint`].

use std::cell::Cell;
use std::rc::Rc;

use ch03::sec_3_3::{Connector, Constraint, Informant};

/// Ben's squarer: `a.value * a.value = b.value`, in either direction.
/// `errored` records the book's "square less than 0" refusal, since
/// [`Constraint::inform_about_value`] cannot itself return a
/// `Result`.
struct Squarer {
    a: Connector,
    b: Connector,
    me: Informant,
    errored: Rc<Cell<bool>>,
}

impl Squarer {
    /// Ben's `process-new-value`: `b` known computes `a` as its square
    /// root (refusing a negative `b`); otherwise `a` known computes
    /// `b` as its square.
    fn process_new_value(&self) {
        if let Some(v) = self.b.value() {
            if v < 0.0 {
                self.errored.set(true);
                return;
            }
            let _ = self.a.set_value(v.sqrt(), &self.me);
        } else if let Some(v) = self.a.value() {
            let _ = self.b.set_value(v * v, &self.me);
        }
    }

    /// Ben's `process-forget-value`: both connectors let go, then the
    /// constraint re-derives whatever it still can.
    fn process_forget_value(&self) {
        self.a.forget_value(&self.me);
        self.b.forget_value(&self.me);
        self.process_new_value();
    }
}

impl Constraint for Squarer {
    fn informant(&self) -> &Informant {
        &self.me
    }

    fn inform_about_value(&self) {
        self.process_new_value();
    }

    fn inform_about_no_value(&self) {
        self.process_forget_value();
    }
}

/// Exercise 3.35's `squarer a b`: connects a fresh [`Squarer`] to both
/// terminals and returns it, alongside the flag that records a
/// negative-square refusal.
#[must_use]
pub fn squarer(a: &Connector, b: &Connector) -> (Rc<dyn Constraint>, Rc<Cell<bool>>) {
    let errored = Rc::new(Cell::new(false));
    let me = Rc::new(Squarer {
        a: a.clone(),
        b: b.clone(),
        me: Informant::new(),
        errored: Rc::clone(&errored),
    });
    let object: Rc<dyn Constraint> = me;
    a.connect(&object);
    b.connect(&object);
    (object, errored)
}

mod ex_3_35 {
    use super::{Connector, Informant, squarer};

    /// Exercise 3.35: a squarer as a new primitive constraint
    ///
    /// Sets `b`, reads `a` back as its square root; forgets `b`, sets
    /// `a`, reads `b` back as its square; and, on a second network,
    /// whether setting `b` negative is refused.
    #[must_use]
    pub fn ex_3_35() -> (Option<f64>, Option<f64>, bool) {
        let a = Connector::new();
        let b = Connector::new();
        let (_me, _errored) = squarer(&a, &b);
        let user = Informant::user();

        b.set_value(49.0, &user)
            .expect("a fresh connector accepts its first value");
        let a_from_b = a.value();
        b.forget_value(&user);
        a.set_value(6.0, &user)
            .expect("a forgotten connector accepts a fresh value");
        let b_from_a = b.value();

        let a2 = Connector::new();
        let b2 = Connector::new();
        let (_me2, errored) = squarer(&a2, &b2);
        b2.set_value(-4.0, &user)
            .expect("the connector itself accepts any first value");

        (a_from_b, b_from_a, errored.get())
    }
}

#[test]
fn ex_3_35() {
    assert_eq!(ex_3_35::ex_3_35(), (Some(7.0), Some(36.0), true));
}
