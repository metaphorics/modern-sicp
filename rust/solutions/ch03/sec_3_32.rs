// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.32: why a time segment's
//! actions must run first in, first out. The and-gate scenario is the
//! book's own: settle the inputs to (0, 1), then change both within
//! one segment to (1, 0). Both changes schedule a set of the output
//! wire into the same later segment, and only FIFO applies them in
//! the order that matches the gate's final, settled inputs.

use std::cell::{Cell, RefCell};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::rc::Rc;

use ch03::sec_3_3::{AND_GATE_DELAY, Agenda, Wire, and_gate, logical_and};

/// One action scheduled on the comparison agenda: same `(time, seq)`
/// key the section's own [`Agenda`] uses, but ordered so that within
/// one time the *larger* seq — the action added later — comes first.
struct LifoScheduled {
    time: u64,
    seq: u64,
    action: Rc<dyn Fn()>,
}

impl PartialEq for LifoScheduled {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for LifoScheduled {}

impl PartialOrd for LifoScheduled {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LifoScheduled {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).then(other.seq.cmp(&self.seq))
    }
}

/// The exercise's comparison agenda: the same scheduling contract as
/// [`Agenda`], except that a time segment's actions run last in,
/// first out instead of first in, first out.
#[derive(Default)]
struct LifoAgenda {
    now: Cell<u64>,
    next_seq: Cell<u64>,
    heap: RefCell<BinaryHeap<Reverse<LifoScheduled>>>,
}

impl LifoAgenda {
    fn add_to(&self, time: u64, action: Rc<dyn Fn()>) {
        let seq = self.next_seq.get();
        self.next_seq.set(seq + 1);
        self.heap
            .borrow_mut()
            .push(Reverse(LifoScheduled { time, seq, action }));
    }

    fn after_delay(self: &Rc<Self>, delay: u64, action: Rc<dyn Fn()>) {
        self.add_to(self.now.get() + delay, action);
    }

    fn propagate(&self) {
        loop {
            let next = self.heap.borrow_mut().pop();
            let Some(item) = next else {
                break;
            };
            self.now.set(item.0.time);
            (item.0.action)();
        }
    }
}

/// An and-gate wired against the comparison agenda: identical rule to
/// the section's [`and_gate`] (read both inputs whenever either
/// changes, schedule the output [`AND_GATE_DELAY`] later), so the only
/// difference between the two runs is which agenda receives the
/// scheduled actions.
fn lifo_and_gate(a1: &Wire, a2: &Wire, output: &Wire, agenda: &Rc<LifoAgenda>) {
    let output = output.clone();
    let input_a = a1.clone();
    let input_b = a2.clone();
    let agenda = Rc::clone(agenda);
    let action: Rc<dyn Fn()> = Rc::new(move || {
        let new_value = logical_and(input_a.signal(), input_b.signal());
        let output = output.clone();
        agenda.after_delay(
            AND_GATE_DELAY,
            Rc::new(move || output.set_signal(new_value)),
        );
    });
    a1.add_action(&action);
    a2.add_action(&action);
}

/// The book's own queue-segmented agenda run over the scenario: two
/// same-segment updates, applied in the order they were scheduled.
#[must_use]
pub fn fifo_result() -> u8 {
    let agenda = Rc::new(Agenda::new());
    let a = Wire::new();
    let b = Wire::new();
    let out = Wire::new();
    and_gate(&a, &b, &out, &agenda);
    a.set_signal(0);
    b.set_signal(1);
    agenda.propagate();
    a.set_signal(1);
    b.set_signal(0);
    agenda.propagate();
    out.signal()
}

/// The comparison run: the same scenario over [`LifoAgenda`], where
/// the two same-segment updates run backwards.
#[must_use]
pub fn lifo_result() -> u8 {
    let agenda = Rc::new(LifoAgenda::default());
    let a = Wire::new();
    let b = Wire::new();
    let out = Wire::new();
    lifo_and_gate(&a, &b, &out, &agenda);
    a.set_signal(0);
    b.set_signal(1);
    agenda.propagate();
    a.set_signal(1);
    b.set_signal(0);
    agenda.propagate();
    out.signal()
}

mod ex_3_32 {
    use super::lifo_result;

    /// Exercise 3.32: agenda segments must run actions first in, first out
    ///
    /// Answers the LIFO comparison agenda's output for the and-gate
    /// scenario: the wrong value a stack-ordered segment produces,
    /// against the correct value [`super::fifo_result`] settles on.
    #[must_use]
    pub fn ex_3_32() -> u8 {
        lifo_result()
    }
}

#[test]
fn ex_3_32() {
    assert_eq!(ex_3_32::ex_3_32(), 1);
}

/// FIFO reads the and-gate's two same-segment updates in the order
/// they were scheduled and settles on the gate's actual final inputs
/// (1, 0), so the output is 0. LIFO runs the same two updates
/// backwards: the update scheduled *first*, from the stale reading
/// (1, 1), overwrites the update scheduled *second*, from the correct
/// reading (1, 0) — so the output is left at the gate's superseded
/// value, 1.
#[test]
fn fifo_and_lifo_disagree_on_the_same_segment_update() {
    assert_eq!(fifo_result(), 0);
    assert_eq!(lifo_result(), 1);
}
