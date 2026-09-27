// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The persistent cons list of 2.2: structural sharing where the book
//! draws sharing diagrams. Typed flat sequences stay `Vec<T>`; `List<T>`
//! exists so `count-leaves` and `fringe` translate onto the shape the
//! section is about.

use std::fmt;
use std::rc::Rc;

/// The cons list: `Nil` or a head over the shared rest. Cloning a `List`
/// clones the head cell and shares the whole spine, which is the sharing
/// the section's diagrams draw; use [`List::cons_shared`] to share a
/// spine by handle.
#[derive(Debug, PartialEq, Eq)]
pub enum List<T> {
    /// The empty list.
    Nil,
    /// A head element over the rest of the list.
    Cons(T, Rc<List<T>>),
}

impl<T: Clone> Clone for List<T> {
    /// Copies the head cell and shares the spine: the value semantics of a
    /// Scheme list, where copying the list shares its structure.
    fn clone(&self) -> Self {
        match self {
            List::Nil => List::Nil,
            List::Cons(x, rest) => List::Cons(x.clone(), Rc::clone(rest)),
        }
    }
}

impl<T> List<T> {
    /// Builds `(cons x rest)`: one new cell whose tail is a value copy of
    /// `rest`'s spine, per the section sketch. Where the book draws
    /// sharing, build the shared spine once and use [`List::cons_shared`].
    #[must_use]
    pub fn cons(x: T, rest: &Self) -> Self
    where
        T: Clone,
    {
        List::Cons(x, Rc::new(rest.clone()))
    }

    /// Builds `(cons x rest)` sharing `rest` by one `Rc` handle: `O(1)`,
    /// and later mutations of neither list are visible in the other. This
    /// is the primitive the sharing diagrams are about.
    #[must_use]
    pub fn cons_shared(x: T, rest: &Rc<Self>) -> Self {
        List::Cons(x, Rc::clone(rest))
    }

    /// The head, or `None` on the empty list.
    #[must_use]
    pub fn car(&self) -> Option<&T> {
        match self {
            List::Cons(x, _) => Some(x),
            List::Nil => None,
        }
    }

    /// The rest, or `None` on the empty list.
    #[must_use]
    pub fn cdr(&self) -> Option<&Self> {
        match self {
            List::Cons(_, r) => Some(r),
            List::Nil => None,
        }
    }

    /// True on the empty list.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// The number of cons cells.
    #[must_use]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Borrows the elements front to back.
    #[must_use]
    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter { cursor: Some(self) }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        let mut out = List::Nil;
        for x in items.into_iter().rev() {
            out = List::Cons(x, Rc::new(out));
        }
        out
    }
}

/// A borrowing iterator over [`List`].
#[derive(Clone, Debug)]
pub struct ListIter<'a, T> {
    cursor: Option<&'a List<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = self.cursor?;
        match node {
            List::Cons(x, rest) => {
                self.cursor = Some(rest);
                Some(x)
            }
            List::Nil => {
                self.cursor = None;
                None
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    /// Prints the book's list form: the elements separated by single
    /// spaces, enclosed in parentheses, so `(list 1 2 3 4)` displays as
    /// `(1 2 3 4)` and a list of lists nests the same way the book's
    /// box-and-pointer figures print.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{item}")?;
        }
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::List;
    use proptest::prelude::*;

    #[test]
    fn cons_car_cdr_round_trip() {
        let rest = List::from_iter([2, 3]);
        let l = List::cons(1, &rest);
        assert_eq!(l.car(), Some(&1));
        assert_eq!(l.cdr(), Some(&rest));
        assert_eq!(l.cdr().expect("cons cell").car(), Some(&2));
        assert!(List::<u8>::Nil.is_empty());
        assert_eq!(List::<u8>::Nil.car(), None);
        assert_eq!(List::<u8>::Nil.cdr(), None);
    }

    #[test]
    fn cons_shared_makes_one_spine_two_fronts() {
        let shared = Rc::new(List::from_iter([2, 3]));
        let a = List::cons_shared(1, &shared);
        let b = List::cons_shared(0, &shared);
        let (List::Cons(_, ra), List::Cons(_, rb)) = (&a, &b) else {
            panic!("both lists are cons cells");
        };
        assert!(Rc::ptr_eq(ra, rb), "the rest spine is shared, not copied");
    }

    #[test]
    fn clone_copies_the_head_and_shares_the_spine() {
        let l = List::from_iter([1, 2, 3]);
        let List::Cons(_, spine) = &l else {
            panic!("three items build two cells");
        };
        let copied = l.clone();
        let List::Cons(_, copied_spine) = &copied else {
            panic!("clone preserves the cons shape");
        };
        assert!(Rc::ptr_eq(spine, copied_spine), "the spine is shared");
        assert_eq!(copied, l);
    }

    #[test]
    fn iter_walks_front_to_back() {
        let l = List::from_iter([10, 20, 30]);
        let collected: Vec<i32> = l.iter().copied().collect();
        assert_eq!(collected, [10, 20, 30]);
        assert_eq!(l.len(), 3);
    }

    proptest! {
        #[test]
        fn from_iterator_then_iter_round_trips(xs in proptest::collection::vec(0..1000i64, 0..64)) {
            let l: List<i64> = xs.iter().copied().collect();
            let collected: Vec<i64> = l.iter().copied().collect();
            prop_assert_eq!(l.len(), xs.len());
            prop_assert_eq!(collected, xs);
        }
    }
}
