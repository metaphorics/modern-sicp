// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.18, one module and one test.

mod ex_2_18 {
    use ch02::sec_2_2::List;

    /// Exercise 2.18: `reverse`
    ///
    /// Cdrs down the list, consing each element onto the front of the
    /// answer built so far: the book's "cons up an answer while cdring
    /// down" pattern with the roles of the two ends exchanged. Because
    /// [`List::cons`] puts its element in the head cell, visiting front
    /// to back lands them back to front.
    fn reverse<T: Clone>(items: &List<T>) -> List<T> {
        let mut answer = List::Nil;
        for x in items {
            answer = List::cons(x.clone(), &answer);
        }
        answer
    }

    /// Exercise 2.18: `reverse`
    ///
    /// Returns the rendered reverse of `(1 4 9 16 25)`.
    pub fn ex_2_18() -> String {
        let items = List::from_iter([1, 4, 9, 16, 25]);
        reverse(&items).to_string()
    }
}

#[test]
fn ex_2_18() {
    assert_eq!(ex_2_18::ex_2_18(), "(25 16 9 4 1)".to_string());
}
