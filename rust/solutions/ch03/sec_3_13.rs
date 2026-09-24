// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.13: `make-cycle` closes the last
//! pair of `(a b c)` back onto the first, so no walk down `cdr`s ever
//! reaches the empty list. The classified reason this is an A: the ring
//! lives in shared mutable cells under `Rc`, and the exercise asks what
//! `last-pair` does on it.

use ch03::sec_3_3::{first_pair, last_pair, make_cycle};
use sicp_runtime::{Pair, Value, eq_pair};

mod ex_3_13 {
    use super::{Value, eq_pair, first_pair};
    use ch03::sec_3_3::make_cycle;

    /// Exercise 3.13: make-cycle closes the chain back on itself
    ///
    /// Builds the book's `z`, walks it the bounded way, and reports
    /// whether the third step lands on the first pair again — which is
    /// why `last-pair` on `z` never answers.
    #[must_use]
    pub fn ex_3_13() -> bool {
        let z = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        make_cycle(&first_pair(&z));
        let mut cursor = first_pair(&z);
        for _ in 0..3 {
            let next = cursor.cdr.borrow().clone();
            cursor = match next {
                Value::Pair(cell) => cell,
                _ => return false,
            };
        }
        eq_pair(&cursor, &first_pair(&z))
    }
}

#[test]
fn ex_3_13() {
    assert!(ex_3_13::ex_3_13());

    // The structure of z: three steps land on the first pair, and the
    // ring is closed, so (last-pair z) would run forever. The same walk
    // on an unclosed chain of three pairs ends on the third pair.
    let z = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
    make_cycle(&first_pair(&z));
    let first = first_pair(&z);
    let mut cursor = Pair::clone(&first);
    for _ in 0..3 {
        let next = cursor.cdr.borrow().clone();
        cursor = match next {
            Value::Pair(cell) => cell,
            _ => break,
        };
    }
    assert!(eq_pair(&cursor, &first));

    let plain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
    let end = last_pair(&first_pair(&plain));
    let tail = end.cdr.borrow().clone();
    assert!(tail.is_nil());
}
