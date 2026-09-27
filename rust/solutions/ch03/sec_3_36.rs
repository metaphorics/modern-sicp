// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.36, replaced for this
//! edition: the book asks for the environment diagram showing how
//! `(set-value! a 10 'user)` reaches a constraint's local state. This
//! edition traces the same call graph as a flat list of dispatch
//! labels instead: setting a connector runs [`Connector::set_value`],
//! which calls every attached constraint's `inform_about_value`,
//! which runs the constraint's own `process_new_value`. Forgetting
//! reaches the same constraint through `inform_about_no_value` and
//! `process_forget_value`.

use std::cell::RefCell;
use std::rc::Rc;

use ch03::sec_3_3::{Connector, Constraint, Informant};

/// A constraint that only logs the two dispatch labels
/// [`Constraint`] can receive, standing in for the book's `adder`
/// while the diagram-turned-trace runs.
struct Spy {
    me: Informant,
    calls: Rc<RefCell<Vec<String>>>,
}

impl Constraint for Spy {
    fn informant(&self) -> &Informant {
        &self.me
    }

    fn inform_about_value(&self) {
        self.calls.borrow_mut().push("inform_about_value".into());
        self.calls.borrow_mut().push("process_new_value".into());
    }

    fn inform_about_no_value(&self) {
        self.calls.borrow_mut().push("inform_about_no_value".into());
        self.calls.borrow_mut().push("process_forget_value".into());
    }
}

/// Attaches a fresh [`Spy`] to `connector` and returns the log it will
/// write dispatch labels into, plus the constraint handle itself: the
/// connector only ever holds a weak reference to a constraint (see
/// [`Connector::connect`]), so something outside must keep the spy
/// alive for as long as its calls still matter.
fn spy_on(connector: &Connector) -> (Rc<RefCell<Vec<String>>>, Rc<dyn Constraint>) {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let object: Rc<dyn Constraint> = Rc::new(Spy {
        me: Informant::new(),
        calls: Rc::clone(&calls),
    });
    connector.connect(&object);
    calls.borrow_mut().clear();
    (calls, object)
}

mod ex_3_36 {
    use super::{Connector, Informant, spy_on};

    /// Exercise 3.36: trace the connector's for-each-except calls
    ///
    /// The dispatch trace `(set-value! a 10 'user)` produces: every
    /// label the call graph runs, in the order it runs them, over the
    /// one spy constraint attached to `a`.
    #[must_use]
    pub fn ex_3_36() -> Vec<String> {
        let a = Connector::new();
        let (calls, _spy) = spy_on(&a);
        let user = Informant::user();
        a.set_value(10.0, &user)
            .expect("a fresh connector accepts its first value");
        calls.borrow().clone()
    }
}

#[test]
fn ex_3_36() {
    assert_eq!(
        ex_3_36::ex_3_36(),
        vec![
            "inform_about_value".to_string(),
            "process_new_value".to_string()
        ]
    );
}

/// Forgetting reaches the same constraint through the other half of
/// the dispatch: `inform_about_no_value`, then `process_forget_value`.
#[test]
fn forgetting_traces_the_other_dispatch_half() {
    let a = Connector::new();
    let (calls, _spy) = spy_on(&a);
    let user = Informant::user();
    a.set_value(10.0, &user)
        .expect("a fresh connector accepts its first value");
    calls.borrow_mut().clear();
    a.forget_value(&user);
    assert_eq!(
        calls.borrow().clone(),
        vec![
            "inform_about_no_value".to_string(),
            "process_forget_value".to_string()
        ]
    );
}
