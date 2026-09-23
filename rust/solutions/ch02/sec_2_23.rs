// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.23, one module and one test.

mod ex_2_23 {
    use ch02::sec_2_2::List;

    /// Exercise 2.23: `for-each`
    ///
    /// Applies an action to each element in turn, left to right, and
    /// returns `()` like the book's arbitrary return value. On `List`
    /// the left-to-right traversal is the borrowing iterator, so the
    /// implementation is the loop the iterator yields; the action takes
    /// `&T` because it only observes.
    fn for_each<T, F: FnMut(&T)>(items: &List<T>, mut action: F) {
        for x in items {
            action(x);
        }
    }

    /// Exercise 2.23: for-each
    ///
    /// Returns the items `for_each` applied its action to, in order, for
    /// the list `(57 321 88)`, together with a flag standing in for its
    /// `()` return value.
    pub fn ex_2_23() -> (Vec<i128>, bool) {
        let items = List::from_iter([57, 321, 88]);
        let mut applied = Vec::new();
        for_each(&items, |x| applied.push(*x));
        (applied, true)
    }
}

#[test]
fn ex_2_23() {
    assert_eq!(ex_2_23::ex_2_23(), (vec![57, 321, 88], true));
}
