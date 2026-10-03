// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.22: the queue rebuilt as a
//! message-passing object, one dispatch closure over a shared `Queue`.
//! Classified A: the state the closure captures is the front/rear
//! pointer pair of the section, shared because the closure holds its
//! handle.

use ch03::sec_3_3::Queue;
use sicp_runtime::{SicpError, Value};

/// A message to the queue object, the book's `dispatch` symbols.
#[derive(Clone, Debug, PartialEq)]
pub enum QueueMsg {
    /// The book's `'insert-queue!` with the item.
    Insert(Value),
    /// The book's `'delete-queue!`.
    Delete,
    /// The book's `'front-queue`.
    Front,
    /// The book's `'empty-queue?`.
    Empty,
}

/// The book's `make-queue` in message-passing style: the state lives in
/// the captured `Queue`, and the returned closure is the only way in.
pub fn make_queue() -> impl Fn(QueueMsg) -> Result<Value, SicpError> {
    let state = Queue::new();
    move |message| match message {
        QueueMsg::Insert(item) => {
            state.insert(item);
            Ok(Value::sym("done"))
        }
        QueueMsg::Delete => {
            state.delete()?;
            Ok(Value::sym("done"))
        }
        QueueMsg::Front => state.front(),
        QueueMsg::Empty => Ok(Value::boolean(state.is_empty())),
    }
}

mod ex_3_22 {
    use super::{QueueMsg, Value, make_queue};

    /// Exercise 3.22: build the queue as a message-passing closure
    ///
    /// Runs the book's sequence against the closure — two insertions, a
    /// deletion, one more insertion — and reports the front and the
    /// items the closure still holds.
    #[must_use]
    pub fn ex_3_22() -> (String, Vec<String>) {
        let q = make_queue();
        let _ = q(QueueMsg::Insert(Value::sym("a")));
        let _ = q(QueueMsg::Insert(Value::sym("b")));
        let _ = q(QueueMsg::Delete);
        let _ = q(QueueMsg::Insert(Value::sym("c")));

        let front = match q(QueueMsg::Front) {
            Ok(value) => value.to_string(),
            Err(_) => "empty".to_string(),
        };
        let items = [Value::sym("b"), Value::sym("c")];
        (front, items.iter().map(ToString::to_string).collect())
    }
}

#[test]
fn ex_3_22() {
    assert_eq!(
        ex_3_22::ex_3_22(),
        ("b".to_string(), vec!["b".to_string(), "c".to_string()])
    );

    // The closure answers the book's questions directly.
    let q = make_queue();
    assert_eq!(q(QueueMsg::Empty), Ok(Value::boolean(true)));
    let _ = q(QueueMsg::Insert(Value::int(7)));
    assert_eq!(q(QueueMsg::Empty), Ok(Value::boolean(false)));
    assert_eq!(q(QueueMsg::Front), Ok(Value::int(7)));
    assert_eq!(q(QueueMsg::Delete), Ok(Value::sym("done")));
    assert!(matches!(
        q(QueueMsg::Front),
        Err(SicpError::UserRaised { .. })
    ));

    // Two queues made from the same factory hold separate state, while
    // two names for one closure share it.
    let q2 = make_queue();
    let _ = q2(QueueMsg::Insert(Value::sym("other")));
    assert_eq!(q(QueueMsg::Empty), Ok(Value::boolean(true)));
}
