// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.21, one module and one test.

mod ex_2_21 {
    use ch02::sec_2_2::{List, map_list};

    /// Exercise 2.21: `square-list`, the direct definition: cons the
    /// square of the `car` onto the square of the `cdr`, with the two
    /// base cases named by the `List` variants.
    fn square_list_direct(items: &List<i128>) -> List<i128> {
        match items {
            List::Nil => List::Nil,
            List::Cons(x, rest) => List::cons(x * x, &square_list_direct(rest)),
        }
    }

    /// Exercise 2.21: `square-list`, the `map` definition: the element
    /// operation and the traversal are separated, and the traversal
    /// becomes one call.
    fn square_list_map(items: &List<i128>) -> List<i128> {
        map_list(|x| x * x, items)
    }

    /// Exercise 2.21: square-list two ways
    ///
    /// Returns the rendered squares of `(1 2 3 4)` computed by the
    /// direct recursive definition and by the `map`-based one, in that
    /// order.
    pub fn ex_2_21() -> (String, String) {
        let items = List::from_iter([1, 2, 3, 4]);
        let direct = square_list_direct(&items);
        let mapped = square_list_map(&items);
        (direct.to_string(), mapped.to_string())
    }
}

#[test]
fn ex_2_21() {
    assert_eq!(
        ex_2_21::ex_2_21(),
        ("(1 4 9 16)".to_string(), "(1 4 9 16)".to_string())
    );
}
