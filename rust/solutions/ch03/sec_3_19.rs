// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.19 and its tailored addition
//! 3.19a: the tortoise-and-hare walk that detects a cycle in constant
//! space, and the shared-suffix question decided by the same kind of
//! pointer alignment.

use ch03::sec_3_3::{first_pair, has_cycle_constant_space, make_cycle, shares_suffix};
use sicp_runtime::{Pair, Value, cons_cell};

mod ex_3_19 {
    use super::{Value, first_pair, has_cycle_constant_space, make_cycle};

    /// Exercise 3.19: tortoise-hare cycle detection in constant space
    ///
    /// Answers the cycle question for a plain chain and for a ring,
    /// with the slow pointer stepping once and the fast pointer twice
    /// per round, holding only two cell handles for the whole walk.
    #[must_use]
    pub fn ex_3_19() -> (bool, bool) {
        let plain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        let plain_answer = has_cycle_constant_space(&first_pair(&plain));

        let ring = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        make_cycle(&first_pair(&ring));
        let ring_answer = has_cycle_constant_space(&first_pair(&ring));

        (plain_answer, ring_answer)
    }
}

mod ex_3_19a {
    use super::{Pair, Value, cons_cell, first_pair, shares_suffix};

    /// Exercise 3.19a: shared suffix detection
    ///
    /// Builds `x = (a b c d)` and `y = (e f c d)` so the pairs holding
    /// `(c d)` are one shared structure, and decides the suffix
    /// question for them and for two look-alike chains that share no
    /// cell at all.
    #[must_use]
    pub fn ex_3_19a() -> (bool, bool) {
        let c_d = Value::list(vec![Value::sym("c"), Value::sym("d")]);
        let shared_tail = first_pair(&c_d);
        let x = cons_cell(
            Value::sym("a"),
            Value::Pair(cons_cell(
                Value::sym("b"),
                Value::Pair(Pair::clone(&shared_tail)),
            )),
        );
        let y = cons_cell(
            Value::sym("e"),
            Value::Pair(cons_cell(
                Value::sym("f"),
                Value::Pair(Pair::clone(&shared_tail)),
            )),
        );

        let look_alike_left = Value::list(vec![Value::sym("c"), Value::sym("d")]);
        let look_alike_right = Value::list(vec![Value::sym("c"), Value::sym("d")]);

        (
            shares_suffix(&x, &y),
            shares_suffix(
                &first_pair(&look_alike_left),
                &first_pair(&look_alike_right),
            ),
        )
    }
}

#[test]
fn ex_3_19() {
    assert_eq!(ex_3_19::ex_3_19(), (false, true));
}

#[test]
fn ex_3_19a() {
    assert_eq!(ex_3_19a::ex_3_19a(), (true, false));
}

/// The suffix question is about identity, not content: two chains that
/// merely print alike share nothing.
#[test]
fn answers_identity_not_equality() {
    let real = Value::list(vec![Value::sym("m"), Value::sym("n")]);
    let copy = Value::list(vec![Value::sym("m"), Value::sym("n")]);
    assert!(shares_suffix(&first_pair(&real), &first_pair(&real)));
    assert!(!shares_suffix(&first_pair(&real), &first_pair(&copy)));
}

/// One chain of `length` pairs of filler symbols.
fn chain(prefix: u64, length: u64) -> Pair {
    let mut current = Value::list(vec![Value::sym("end")]);
    for step in 0..length {
        current = Value::Pair(cons_cell(
            Value::int(i128::from(prefix) + i128::from(step)),
            current,
        ));
    }
    first_pair(&current)
}

proptest::proptest! {
    /// For random chains, a suffix grafted from one chain into another
    /// is always found, and independent chains of equal content are
    /// always rejected.
    #[test]
    fn detects_shared_suffixes_exactly(
        length_left in 1u64..24,
        length_right in 1u64..24,
        graft_at in 0u64..8,
        seed in 0u64..1000,
    ) {
        let left = chain(seed, length_left);
        let right = chain(seed + 7, length_right);
        proptest::prop_assert!(!shares_suffix(&left, &right));

        let graft_point = graft_at % (length_left + 1);
        let shared = chain(seed, length_left.max(graft_point));
        let mut cursor = Pair::clone(&shared);
        for _ in 0..graft_point {
            let next = cursor.cdr.borrow().clone();
            let Value::Pair(cell) = next else {
                break;
            };
            cursor = cell;
        }
        let grafted = cons_cell(Value::int(999), Value::Pair(cursor));
        proptest::prop_assert!(shares_suffix(&grafted, &shared));
        proptest::prop_assert!(!shares_suffix(&left, &grafted));
    }
}
