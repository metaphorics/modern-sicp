// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3.2

//! Section 3.3.2: queues — the front/rear pointer representation and the
//! constant-time insertion and deletion the book's figures show.

use ch03::sec_3_3::Queue;
use sicp_runtime::Value;

fn main() {
    // Ben's q1 of the book's interaction: two insertions and a deletion.
    // The default printer of the book's representation shows the
    // front/rear pointer pair; this edition's `items` shows the queue's
    // own sequence, which is what exercise 3.21's `print-queue` prints.
    let q1 = Queue::new();
    assert!(q1.is_empty());

    q1.insert(Value::sym("a"));
    let items = q1.items();
    let shown = items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("({shown})");
    // => (a)
    assert_eq!(shown, "a");

    q1.insert(Value::sym("b"));
    let items = q1.items();
    let shown = items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("({shown})");
    // => (a b)
    assert_eq!(shown, "a b");

    // front-queue reads the first item without touching it.
    let Ok(front) = q1.front() else {
        return;
    };
    println!("{front}");
    // => a
    assert_eq!(front, Value::sym("a"));

    // delete-queue! moves only the front pointer; the rear pointer stays
    // where it is, which is harmless because emptiness looks at the
    // front alone.
    if q1.delete().is_err() {
        return;
    }
    let items = q1.items();
    let shown = items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("({shown})");
    // => (b)
    assert_eq!(shown, "b");

    if q1.delete().is_err() {
        return;
    }
    println!("{}", q1.is_empty());
    // => #t
    assert!(q1.is_empty());

    // Deleting from an empty queue raises the book's error.
    let refused = q1.delete().expect_err("empty queue must refuse");
    println!("{refused}");
    // => delete! called with an empty queue
    assert!(refused.to_string().contains("empty queue"));

    // Insertion after the queue drained works: the rear pointer picks
    // the new pair up even though it never moved before.
    q1.insert(Value::sym("c"));
    let Ok(front) = q1.front() else {
        return;
    };
    println!("{front}");
    // => c
    assert_eq!(front, Value::sym("c"));
}
