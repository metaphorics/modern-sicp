// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.21: Ben's `print-queue`. The
//! book's default printer shows the queue's front/rear pointer pair,
//! not the items, so Eva Lu Ator sees `((b d) d)` where the queue holds
//! `b` and `d`. The fix prints the queue's own sequence of items.

use ch03::sec_3_3::Queue;
use sicp_runtime::Value;

mod ex_3_21 {
    use super::{Queue, Value, pointer_pair_view, print_queue};

    /// Exercise 3.21: print the queue's own sequence of items
    ///
    /// Runs Ben's q1 sequence — insert a, insert b, delete, insert d —
    /// and reports two views of the result: the queue's items, and the
    /// pointer-pair reading that the default printer of the book's
    /// representation produces.
    #[must_use]
    pub fn ex_3_21() -> (String, String) {
        let q1 = Queue::new();
        q1.insert(Value::sym("a"));
        q1.insert(Value::sym("b"));
        let _ = q1.delete();
        q1.insert(Value::sym("d"));

        (print_queue(&q1), pointer_pair_view(&q1))
    }
}

/// The fixed printer: the items in queue order.
#[must_use]
pub fn print_queue(queue: &Queue) -> String {
    let items = queue.items();
    let shown = items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    format!("({shown})")
}

/// What the book's default printer shows for the same queue: the pair
/// of the front pointer's list and the rear pointer's node. Reading the
/// raw pointers is what reproduces Ben's fourth line `(() b)`: after
/// the last deletion the front is empty while the rear still names the
/// final node.
#[must_use]
pub fn pointer_pair_view(queue: &Queue) -> String {
    let (front, rear) = queue.pointer_pair();
    let front_value = front.map_or(Value::Nil, Value::Pair);
    let rear_value = rear.map_or(Value::Nil, Value::Pair);
    Value::Pair(cons_of(front_value, rear_value)).to_string()
}

fn cons_of(car: Value, cdr: Value) -> sicp_runtime::Pair {
    sicp_runtime::cons_cell(car, cdr)
}

#[test]
fn ex_3_21() {
    // The queue holds b then d; the pointer-pair reading repeats the
    // rear node, which is exactly the confusion Eva Lu Ator names.
    assert_eq!(
        ex_3_21::ex_3_21(),
        ("(b d)".to_string(), "((b d) d)".to_string())
    );

    // Ben's premise, line for line: the raw pointer pair after each
    // operation, including the stale rear after the last deletion.
    let q1 = Queue::new();
    q1.insert(Value::sym("a"));
    assert_eq!(pointer_pair_view(&q1), "((a) a)");
    q1.insert(Value::sym("b"));
    assert_eq!(pointer_pair_view(&q1), "((a b) b)");
    let _ = q1.delete();
    assert_eq!(pointer_pair_view(&q1), "((b) b)");
    let _ = q1.delete();
    assert_eq!(pointer_pair_view(&q1), "(() b)");

    // The fixed printer never changes the queue it prints.
    let q = Queue::new();
    q.insert(Value::int(1));
    q.insert(Value::int(2));
    assert_eq!(print_queue(&q), "(1 2)");
    assert_eq!(print_queue(&q), "(1 2)");
}
