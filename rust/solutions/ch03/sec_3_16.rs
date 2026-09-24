// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.16: Ben's `count-pairs`, which
//! counts a shared pair once per route through it, so the structures
//! the statement draws answer 3, 4, 7, and never.

use ch03::sec_3_3::{first_pair, has_cycle, make_cycle};
use sicp_runtime::{Pair, Value, cons_cell};

mod ex_3_16 {
    use super::{Value, cons_cell, count_pairs, first_pair, has_cycle, make_cycle};

    /// Exercise 3.16: count-pairs counts shared pairs more than once
    ///
    /// Builds the book's four structures and reports what Ben's
    /// procedure answers for the first three, plus the cycle test that
    /// stands in for the fourth, whose count never returns.
    #[must_use]
    pub fn ex_3_16() -> (u64, u64, u64, bool) {
        // (a b c): three pairs in a plain chain.
        let plain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);

        // ((a b) b): the car names the pair chain (a b), the cdr names
        // its second pair, so that pair is reached by two routes.
        let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let x_tail = first_pair(&x).cdr.borrow().clone();
        let once_shared = Value::Pair(cons_cell(x, x_tail));

        // ((a b c) a b c): both halves name the same three-pair chain,
        // so every pair in it is counted twice, plus the outer pair.
        let chain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        let twice_shared = Value::Pair(cons_cell(chain.clone(), chain));

        // The fourth structure closes a three-pair ring.
        let ring = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        make_cycle(&first_pair(&ring));

        (
            count_pairs(&plain),
            count_pairs(&once_shared),
            count_pairs(&twice_shared),
            has_cycle(&first_pair(&ring)),
        )
    }
}

/// Ben's `count-pairs`: a shared pair is counted once per route to it.
#[must_use]
pub fn count_pairs(x: &Value) -> u64 {
    let Value::Pair(cell) = x else {
        return 0;
    };
    1 + count_pairs(&cell.car.borrow()) + count_pairs(&cell.cdr.borrow())
}

#[test]
fn ex_3_16() {
    let (plain, once, twice, loops) = ex_3_16::ex_3_16();
    assert_eq!((plain, once, twice), (3, 4, 7));
    assert!(loops);

    // The same three structures all hold exactly three distinct pairs,
    // which is what exercise 3.17's counter answers.
    let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let x_tail = first_pair(&x).cdr.borrow().clone();
    let once_shared = Value::Pair(cons_cell(x, x_tail));
    assert_eq!(ch03::sec_3_3::count_pairs_distinct(&once_shared), 3);

    let ring = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
    make_cycle(&first_pair(&ring));
    let _keep: Pair = first_pair(&ring);
    assert_eq!(
        ch03::sec_3_3::count_pairs_distinct(&Value::Pair(first_pair(&ring))),
        3
    );
}
