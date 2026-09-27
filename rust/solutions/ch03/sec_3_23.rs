// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.23: a deque, where items can be
//! inserted and deleted at either end in constant time. Each node
//! carries a previous pointer as well as a next pointer, and the deque
//! keeps both end handles, so no operation ever scans.

use std::cell::RefCell;
use std::rc::Rc;

use sicp_runtime::{SchemeError, Value};

/// One node of the deque: the item plus the two neighbor handles.
struct DequeNode {
    item: Value,
    previous: RefCell<Option<Rc<DequeNode>>>,
    next: RefCell<Option<Rc<DequeNode>>>,
}

/// The book's deque: front and rear handles over two-way linked nodes.
#[derive(Default)]
pub struct Deque {
    front: RefCell<Option<Rc<DequeNode>>>,
    rear: RefCell<Option<Rc<DequeNode>>>,
    length: std::cell::Cell<usize>,
}

impl Deque {
    /// The book's `make-deque`: an empty deque.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `empty-deque?`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length.get() == 0
    }

    /// How many items the deque holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.length.get()
    }

    /// The book's `front-insert-dequeue!`: one new node, constant time.
    pub fn front_insert(&self, item: Value) {
        let node = Rc::new(DequeNode {
            item,
            previous: RefCell::new(None),
            next: RefCell::new(self.front.borrow().clone()),
        });
        if let Some(old_front) = self.front.borrow().as_ref() {
            *old_front.previous.borrow_mut() = Some(Rc::clone(&node));
        }
        *self.front.borrow_mut() = Some(Rc::clone(&node));
        if self.rear.borrow().is_none() {
            *self.rear.borrow_mut() = Some(node);
        }
        self.length.set(self.length.get() + 1);
    }

    /// The book's `rear-insert-dequeue!`: the mirror image.
    pub fn rear_insert(&self, item: Value) {
        let node = Rc::new(DequeNode {
            item,
            previous: RefCell::new(self.rear.borrow().clone()),
            next: RefCell::new(None),
        });
        if let Some(old_rear) = self.rear.borrow().as_ref() {
            *old_rear.next.borrow_mut() = Some(Rc::clone(&node));
        }
        *self.rear.borrow_mut() = Some(Rc::clone(&node));
        if self.front.borrow().is_none() {
            *self.front.borrow_mut() = Some(node);
        }
        self.length.set(self.length.get() + 1);
    }

    /// The book's `front-delete-dequeue!`, answering the removed item.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] when the deque is empty.
    pub fn front_delete(&self) -> Result<Value, SchemeError> {
        let front = self.front.borrow().clone();
        let Some(node) = front else {
            return Err(SchemeError::UserRaised {
                message: "front-delete-dequeue! called with an empty deque".into(),
                irritants: vec![],
            });
        };
        self.front.borrow_mut().clone_from(&node.next.borrow());
        if self.front.borrow().is_none() {
            *self.rear.borrow_mut() = None;
        } else if let Some(new_front) = self.front.borrow().as_ref() {
            *new_front.previous.borrow_mut() = None;
        }
        self.length.set(self.length.get() - 1);
        Ok(node.item.clone())
    }

    /// The book's `rear-delete-dequeue!`, answering the removed item.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] when the deque is empty.
    pub fn rear_delete(&self) -> Result<Value, SchemeError> {
        let rear = self.rear.borrow().clone();
        let Some(node) = rear else {
            return Err(SchemeError::UserRaised {
                message: "rear-delete-dequeue! called with an empty deque".into(),
                irritants: vec![],
            });
        };
        self.rear.borrow_mut().clone_from(&node.previous.borrow());
        if self.rear.borrow().is_none() {
            *self.front.borrow_mut() = None;
        } else if let Some(new_rear) = self.rear.borrow().as_ref() {
            *new_rear.next.borrow_mut() = None;
        }
        self.length.set(self.length.get() - 1);
        Ok(node.item.clone())
    }

    /// The items from front to rear, for the tests and for printing.
    /// The walk stops at the first node whose next pointer is unset, so
    /// the structure's cycles can never trap it.
    #[must_use]
    #[allow(
        clippy::assigning_clones,
        reason = "cursor is moved out by the while-let pattern each iteration, so clone_from has no initialized place to reuse"
    )]
    pub fn items(&self) -> Vec<Value> {
        let mut out = Vec::new();
        let mut cursor = self.front.borrow().clone();
        while let Some(node) = cursor {
            out.push(node.item.clone());
            cursor = node.next.borrow().clone();
        }
        out
    }
}

mod ex_3_23 {
    use super::{Deque, Value};

    /// Exercise 3.23: a deque with constant-time operations at both ends
    ///
    /// Builds `(a b c)` by rear insertions, front-inserts `z`, then
    /// deletes from both ends once each, and reports the items left.
    #[must_use]
    pub fn ex_3_23() -> Vec<String> {
        let d = Deque::new();
        d.rear_insert(Value::sym("a"));
        d.rear_insert(Value::sym("b"));
        d.rear_insert(Value::sym("c"));
        d.front_insert(Value::sym("z"));
        let _ = d.front_delete();
        let _ = d.rear_delete();
        d.items().iter().map(ToString::to_string).collect()
    }
}

#[test]
fn ex_3_23() {
    assert_eq!(ex_3_23::ex_3_23(), vec!["a".to_string(), "b".to_string()]);

    // Every operation answers in constant time, which the book asks for
    // by construction: no insertion or deletion ever scans the chain.
    let d = Deque::new();
    assert!(d.is_empty());
    d.front_insert(Value::int(1));
    d.rear_insert(Value::int(2));
    d.front_insert(Value::int(3));
    assert_eq!(d.items(), vec![Value::int(3), Value::int(1), Value::int(2)]);
    assert_eq!(d.front_delete(), Ok(Value::int(3)));
    assert_eq!(d.rear_delete(), Ok(Value::int(2)));
    assert_eq!(d.front_delete(), Ok(Value::int(1)));
    assert!(d.is_empty());
    assert!(d.front_delete().is_err());
    assert!(d.rear_delete().is_err());
}
