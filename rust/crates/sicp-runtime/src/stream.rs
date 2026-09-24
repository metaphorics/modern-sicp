// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The stream of 3.5: the book's `cons-stream`/`delay` pair as a chain of
//! memoized nodes. `Stream::Cons` is a pair whose head is computed eagerly
//! and whose tail is a [`Lazy`] of the rest of the stream;
//! `Stream::Empty` is `the-empty-stream`, so `stream-null?` is
//! [`Stream::is_empty`].
//!
//! Cloning a stream clones a pointer to the same memoized spine: two
//! references to one stream force each tail at most once, which is what
//! the memoization probes of exercises 3.51, 3.52, 3.57, and 3.63 hinge
//! on. The tail thunk of a self-referential stream (`fibs` defined in
//! terms of itself, the 3.5.4 `solve` integral loop) captures the head
//! node, so this port necessarily forms an `Rc` cycle that leaks by
//! design. The leak is bounded to streams the program still holds and is
//! reclaimed at process exit; it is not a defect to remove.

use std::rc::Rc;

use crate::lazy::Lazy;

/// The book's stream: either `the-empty-stream` or a `cons-stream` pair
/// whose tail is delayed and memoized.
pub enum Stream<T> {
    /// The book's `the-empty-stream`; `stream-null?` is [`Stream::is_empty`].
    Empty,
    /// The book's `(cons-stream head tail-thunk)`: the head is eager, the
    /// tail thunk runs at most once and its result is memoized.
    Cons(T, Lazy<Stream<T>>),
}

impl<T: Clone> Clone for Stream<T> {
    fn clone(&self) -> Self {
        match self {
            Stream::Empty => Stream::Empty,
            Stream::Cons(head, tail) => Stream::Cons(head.clone(), tail.clone()),
        }
    }
}

impl<T: 'static> Stream<T> {
    /// Builds the book's `(cons-stream head tail-thunk)`: the head is
    /// eager, the tail thunk runs at most once.
    #[must_use]
    pub fn cons_stream(head: T, tail: impl FnOnce() -> Stream<T> + 'static) -> Stream<T> {
        Stream::Cons(head, Lazy::new(tail))
    }

    /// The book's `stream-null?`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Stream::Empty)
    }

    /// The book's `stream-car`.
    ///
    /// # Panics
    /// Panics on `the-empty-stream`: taking the head of the empty stream
    /// is an error in the book, too.
    #[must_use]
    pub fn head(&self) -> &T {
        match self {
            Stream::Empty => panic!("stream-car: the empty stream has no head"),
            Stream::Cons(head, _) => head,
        }
    }
}

impl<T: Clone + 'static> Stream<T> {
    /// The book's `stream-cdr`: forces and returns the memoized tail.
    ///
    /// # Panics
    /// Panics on `the-empty-stream`: taking the tail of the empty stream
    /// is an error in the book, too.
    #[must_use]
    pub fn tail(&self) -> Stream<T> {
        match self {
            Stream::Empty => panic!("stream-cdr: the empty stream has no tail"),
            Stream::Cons(_, tail) => Rc::unwrap_or_clone(tail.force()),
        }
    }

    /// Walks heads front to back, forcing memoized tails as it goes.
    /// Infinite streams are fine; callers `take(n)`. The cursor shares
    /// every memoized tail it walks with all other cursors.
    #[must_use]
    pub fn iter(&self) -> StreamIter<T> {
        StreamIter {
            cursor: Some(self.clone()),
        }
    }
}

/// A cursor over a [`Stream`], sharing memoized tails with every other
/// cursor on the same stream.
pub struct StreamIter<T> {
    cursor: Option<Stream<T>>,
}

impl<T: Clone + 'static> IntoIterator for &Stream<T> {
    type Item = T;
    type IntoIter = StreamIter<T>;

    fn into_iter(self) -> StreamIter<T> {
        self.iter()
    }
}

impl<T: Clone + 'static> Iterator for StreamIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let cursor = self.cursor.take()?;
        if cursor.is_empty() {
            return None;
        }
        let head = cursor.head().clone();
        self.cursor = Some(cursor.tail());
        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::Stream;

    fn cons(head: u32, tail: Stream<u32>) -> Stream<u32> {
        Stream::cons_stream(head, move || tail)
    }

    #[test]
    fn tail_generator_runs_once_across_many_forces() {
        let calls = Rc::new(Cell::new(0u32));
        let calls_in_tail = Rc::clone(&calls);
        let s: Stream<u64> = Stream::cons_stream(1, move || {
            calls_in_tail.set(calls_in_tail.get() + 1);
            Stream::cons_stream(2, || Stream::Empty)
        });
        let t1 = s.tail();
        let t2 = s.tail();
        assert_eq!(calls.get(), 1, "the tail generator ran exactly once");
        assert_eq!(t1.head(), &2);
        assert_eq!(t2.head(), &2, "the second force served the same memo");
    }

    #[test]
    fn clone_shares_the_memoized_spine() {
        let builds = Rc::new(Cell::new(0u32));
        let builds_in_tail = Rc::clone(&builds);
        let s: Stream<u32> = Stream::cons_stream(0, move || {
            builds_in_tail.set(builds_in_tail.get() + 1);
            Stream::cons_stream(1, || Stream::Empty)
        });
        let t1 = s.clone().tail();
        let t2 = s.tail();
        assert_eq!(
            builds.get(),
            1,
            "one build across both references: the clone walks the same spine"
        );
        assert_eq!((t1.head(), t2.head()), (&1, &1));
    }

    #[test]
    fn iter_takes_n_heads_of_a_finite_chain() {
        let s = cons(1, cons(2, cons(3, Stream::Empty)));
        let heads: Vec<u32> = s.iter().take(3).collect();
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
        let ones: Stream<u32> = Stream::cons_stream(1, move || {
            slot_in_tail
                .borrow()
                .as_ref()
                .expect("seeded before any force")
                .clone()
        });
        *slot.borrow_mut() = Some(ones.clone());
        let heads: Vec<u32> = ones.iter().take(4).collect();
        assert_eq!(heads, [1, 1, 1, 1]);
    }

    #[test]
    fn two_cursors_share_one_memoized_spine() {
        let builds = Rc::new(Cell::new(0u32));
        let builds_in_tail = Rc::clone(&builds);
        let s: Stream<u32> = Stream::cons_stream(0, move || {
            builds_in_tail.set(builds_in_tail.get() + 1);
            Stream::cons_stream(1, || Stream::cons_stream(2, || Stream::Empty))
        });
        let mut a = s.iter();
        let mut b = s.iter();
        // Two heads per cursor force the one memoized tail exactly once.
        assert_eq!((a.next(), a.next()), (Some(0), Some(1)));
        assert_eq!((b.next(), b.next()), (Some(0), Some(1)));
        assert_eq!(builds.get(), 1, "two cursors, one tail build");
    }

    #[test]
    fn head_and_tail_of_the_empty_stream_panic() {
        let empty: Stream<u32> = Stream::Empty;
        assert!(empty.is_empty());
        let head = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| empty.head()));
        assert!(head.is_err(), "stream-car of the empty stream panics");
        let tail = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| empty.tail()));
        assert!(tail.is_err(), "stream-cdr of the empty stream panics");
    }
}
