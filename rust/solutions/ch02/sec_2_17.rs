// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.17, one module and one test.

mod ex_2_17 {
    use ch02::sec_2_2::List;
    use sicp_runtime::SicpError;

    /// Exercise 2.17: `last-pair`
    ///
    /// Cdrs down the list until the rest is the empty list, then returns
    /// the one-element list holding that last cell's `car`, exactly the
    /// book's recursive plan.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] on the empty list, which has no
    /// last pair.
    fn last_pair(items: &List<i128>) -> Result<List<i128>, SicpError> {
        let List::Cons(x, rest) = items else {
            return Err(SicpError::TypeMismatch(
                "last-pair of the empty list".into(),
            ));
        };
        match rest.as_ref() {
            List::Nil => Ok(List::cons(*x, &List::Nil)),
            List::Cons(..) => last_pair(rest),
        }
    }

    /// Exercise 2.17: `last-pair`
    ///
    /// Returns the rendered list holding only the last element of
    /// `(23 72 149 34)`.
    pub fn ex_2_17() -> String {
        let items = List::from_iter([23, 72, 149, 34]);
        let last = last_pair(&items).expect("the sample list is nonempty");
        last.to_string()
    }
}

#[test]
fn ex_2_17() {
    assert_eq!(ex_2_17::ex_2_17(), "(34)".to_string());
}
