// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.17: counting the distinct pairs
//! of a structure once each, with an auxiliary history of the cells
//! already visited. In Rust the history is a `HashSet` of cell
//! addresses, which is exactly the identity test the book asks for.

use ch03::sec_3_3::{count_pairs_distinct, first_pair};
use sicp_runtime::{Value, cons_cell};

mod ex_3_17 {
    use super::{Value, cons_cell, count_pairs_distinct, first_pair};

    /// Exercise 3.17: count distinct pairs once each with a history
    ///
    /// Rebuilds the three structures of exercise 3.16 and reports the
    /// distinct count for each: sharing a pair between two spines only
    /// ever visits that pair once, so `once_shared` still counts 3, but
    /// `twice_shared`'s three-cell chain sits behind the outer pair's
    /// car and cdr alike, for 1 + 3 = 4 distinct cells in all.
    #[must_use]
    pub fn ex_3_17() -> (u64, u64, u64) {
        let plain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);

        let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let x_tail = first_pair(&x).cdr.borrow().clone();
        let once_shared = Value::Pair(cons_cell(x, x_tail));

        let chain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        let twice_shared = Value::Pair(cons_cell(chain.clone(), chain));

        (
            count_pairs_distinct(&plain),
            count_pairs_distinct(&once_shared),
            count_pairs_distinct(&twice_shared),
        )
    }
}

#[test]
fn ex_3_17() {
    assert_eq!(ex_3_17::ex_3_17(), (3, 3, 4));

    // The history runs on cell identity, not content: two structures
    // built with equal parts but no sharing count every cell, so the
    // twins are the outer pair plus four cells, five in all.
    let left = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let right = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let twins = Value::Pair(cons_cell(left, right));
    assert_eq!(count_pairs_distinct(&twins), 5);

    // A ring terminates: the history stops the walk at the first
    // revisit, where Ben's counter would spin forever.
    let ring = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
    ch03::sec_3_3::make_cycle(&first_pair(&ring));
    assert_eq!(count_pairs_distinct(&Value::Pair(first_pair(&ring))), 3);
}
