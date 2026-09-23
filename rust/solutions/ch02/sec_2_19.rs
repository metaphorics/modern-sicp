// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.19, one module and one test.

mod ex_2_19 {
    use ch02::sec_2_2::List;

    /// The book's `first-denomination`: the value of the first coin in
    /// the list. The caller guarantees a nonempty list, and the guard in
    /// [`cc`] keeps that true on every recursive call.
    fn first_denomination(coin_values: &List<i128>) -> i128 {
        match coin_values.car() {
            Some(value) => *value,
            None => 0,
        }
    }

    /// The book's `except-first-denomination`: the rest of the coin list.
    fn except_first_denomination(coin_values: &List<i128>) -> List<i128> {
        match coin_values.cdr() {
            Some(rest) => rest.clone(),
            None => List::Nil,
        }
    }

    /// The book's `no-more?`.
    fn no_more(coin_values: &List<i128>) -> bool {
        coin_values.is_empty()
    }

    /// Exercise 2.19: `cc` over a list of coin values, the 1.2.2 tree
    /// recursion with the coin kinds carried as data. Amounts are
    /// integer pence, so the book's half-pound coin would be the value
    /// 50 and duplicate the 50-pence coin; this edition's `uk-coins`
    /// therefore lists `(100 50 20 10 5 2 1)`.
    fn cc(amount: i128, coin_values: &List<i128>) -> i128 {
        if amount == 0 {
            return 1;
        }
        if amount < 0 || no_more(coin_values) {
            return 0;
        }
        cc(amount, &except_first_denomination(coin_values))
            + cc(amount - first_denomination(coin_values), coin_values)
    }

    /// Exercise 2.19: change-counting with a coin list
    ///
    /// Returns the number of ways to change 100 with `us-coins`
    /// `(50 25 10 5 1)` and with `uk-coins` `(100 50 20 10 5 2 1)`, in
    /// that order.
    pub fn ex_2_19() -> (i128, i128) {
        let american = List::from_iter([50, 25, 10, 5, 1]);
        let british = List::from_iter([100, 50, 20, 10, 5, 2, 1]);
        (cc(100, &american), cc(100, &british))
    }
}

#[test]
fn ex_2_19() {
    assert_eq!(ex_2_19::ex_2_19(), (292, 4_563));
}
