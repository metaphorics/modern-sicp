// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The stream of 3.5: a memoized head node whose tail is a [`Lazy`] of the
//! next node, the book's `cons-stream`/`delay` pair.
//!
//! The tail thunk hands out the shared node pointer (`Stream<T>`, an
//! `Rc`), not an owned node: the book's self-referential streams (`fibs`
//! defined in terms of itself, the 3.5.4 `solve` integral loop) make the
//! tail thunk capture the head node, so this port necessarily forms an
//! `Rc` cycle that leaks by design. The leak is bounded to streams the
//! program still holds and is reclaimed at process exit; it is not a
//! defect to remove.

use std::rc::Rc;

use crate::lazy::Lazy;

/// One stream node: a computed head and a memoized tail.
pub struct SNode<T> {
    /// The head value, computed eagerly by `cons-stream`.
    pub head: T,
    /// The delayed tail, memoized by `Lazy` on the shared node pointer.
    pub tail: Lazy<Stream<T>>,
}

/// The book's stream: a shared, memoized chain of [`SNode`]s.
pub type Stream<T> = Rc<SNode<T>>;

impl<T: 'static> SNode<T> {
    /// Builds the book's `(cons-stream head tail-thunk)`: the head is
    /// eager, the tail thunk runs at most once.
    #[must_use]
    pub fn cons_stream(head: T, tail: impl FnOnce() -> Stream<T> + 'static) -> Stream<T> {
        Rc::new(SNode {
            head,
            tail: Lazy::new(tail),
        })
    }

    /// Forces and returns the memoized tail node.
    #[must_use]
    pub fn tail(&self) -> Stream<T> {
        Rc::unwrap_or_clone(self.tail.force())
    }
}

impl<T: Clone + 'static> SNode<T> {
    /// Walks heads front to back, forcing memoized tails as it goes.
    /// Infinite streams are fine; callers `take(n)`.
    #[must_use]
    pub fn iter(self: Rc<Self>) -> StreamIter<T> {
        StreamIter { cursor: Some(self) }
    }
}

/// A cursor over a [`Stream`], sharing memoized tails with every other
/// cursor on the same stream.
pub struct StreamIter<T> {
    cursor: Option<Stream<T>>,
}

impl<T: Clone + 'static> Iterator for StreamIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let node = self.cursor.take()?;
        let head = node.head.clone();
        self.cursor = Some(node.tail());
        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::{SNode, Stream};

    #[test]
    fn tail_generator_runs_once_across_many_forces() {
        let calls = Rc::new(Cell::new(0u32));
        let calls_in_tail = Rc::clone(&calls);
        let s: Stream<u64> = SNode::cons_stream(1, move || {
            calls_in_tail.set(calls_in_tail.get() + 1);
            SNode::cons_stream(2, || panic!("test never walks past the second node"))
        });
        let t1 = s.tail();
        let t2 = s.tail();
        assert_eq!(calls.get(), 1, "the tail generator ran exactly once");
        assert!(Rc::ptr_eq(&t1, &t2), "both forces share one memoized tail");
        assert_eq!(t1.head, 2);
    }

    #[test]
    fn iter_takes_n_heads_of_a_finite_chain() {
        // Iterator::take pulls one item past the last requested one, so
        // the chain carries four nodes for three heads.
        let s = SNode::cons_stream(1u8, || {
            SNode::cons_stream(2, || {
                SNode::cons_stream(3, || SNode::cons_stream(4, || panic!("chain ends here")))
            })
        });
        let heads: Vec<u8> = s.iter().take(3).collect();
        assert_eq!(heads, [1, 2, 3]);
    }

    #[test]
    fn self_referential_stream_cycles_by_design() {
        // ones = (cons-stream 1 ones): the tail thunk captures the head
        // node, forming the documented Rc cycle. Iteration still works;
        // the cycle leaks by design until process exit.
        let slot: Rc<std::cell::RefCell<Option<Stream<u32>>>> =
            Rc::new(std::cell::RefCell::new(None));
        let slot_in_tail = Rc::clone(&slot);
        let ones: Stream<u32> = SNode::cons_stream(1, move || {
            slot_in_tail.borrow().as_ref().expect("seeded").clone()
        });
        *slot.borrow_mut() = Some(Rc::clone(&ones));
        let heads: Vec<u32> = ones.iter().take(4).collect();
        assert_eq!(heads, [1, 1, 1, 1]);
    }

    #[test]
    fn two_cursors_share_one_memoized_spine() {
        let builds = Rc::new(Cell::new(0u32));
        let builds_in_tail = Rc::clone(&builds);
        let s: Stream<u32> = SNode::cons_stream(0, move || {
            builds_in_tail.set(builds_in_tail.get() + 1);
            SNode::cons_stream(1, || {
                SNode::cons_stream(2, || panic!("beyond the counted tail"))
            })
        });
        let mut a = Rc::clone(&s).iter();
        let mut b = s.iter();
        // Two heads per cursor force the one memoized tail exactly once.
        assert_eq!((a.next(), a.next()), (Some(0), Some(1)));
        assert_eq!((b.next(), b.next()), (Some(0), Some(1)));
        assert_eq!(builds.get(), 1, "two cursors, one tail build");
    }
}
