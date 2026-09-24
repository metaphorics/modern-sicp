// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.15: Ben's `set-to-wow!` on the
//! two structures `z1 = (cons x x)` and `z2 = (cons (list 'a 'b)
//! (list 'a 'b))`. Classified A: the sharing the book draws is aliasing
//! through `Rc` handles over mutable cells.

use std::rc::Rc;

use ch03::sec_3_3::first_pair;
use sicp_runtime::{Pair, Value, cons_cell, eq_pair, set_car};

mod ex_3_15 {
    use super::{Value, cons_cell, set_to_wow};

    /// Exercise 3.15: set-to-wow! shows which halves share one pair
    ///
    /// Builds `z1` and `z2`, applies `set-to-wow!` to each, and reports
    /// the two printed forms afterwards.
    #[must_use]
    #[expect(
        clippy::similar_names,
        reason = "fresh_car/fresh_cdr name the two independently built halves of z2, deliberately mirroring z1's car/cdr"
    )]
    pub fn ex_3_15() -> (String, String) {
        let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let z1 = cons_cell(x.clone(), x);

        let fresh_car = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let fresh_cdr = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let z2 = cons_cell(fresh_car, fresh_cdr);

        let z1_after = Value::Pair(set_to_wow(&z1)).to_string();
        let z2_after = Value::Pair(set_to_wow(&z2)).to_string();
        (z1_after, z2_after)
    }
}

/// The book's `set-to-wow!`: replaces the `car` of the pair that `x`'s
/// first cell names, and returns `x`.
#[must_use]
pub fn set_to_wow(x: &Pair) -> Pair {
    let inner = x.car.borrow().clone();
    if let Value::Pair(cell) = inner {
        set_car(&cell, Value::sym("wow"));
    }
    Pair::clone(x)
}

#[test]
#[expect(
    clippy::similar_names,
    reason = "z1_car/z1_cdr name the two halves of one pair; that similarity is the point of the pointer test"
)]
fn ex_3_15() {
    // z1's two halves are one pair, so both change; z2's halves are two
    // distinct pairs with equal content, so only the car half changes.
    assert_eq!(
        ex_3_15::ex_3_15(),
        ("((wow b) wow b)".to_string(), "((wow b) a b)".to_string())
    );

    // The pointer test explains both outcomes before any mutation: the
    // halves of z1 are the same object, the halves of z2 are not.
    let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
    let z1 = cons_cell(x.clone(), x);
    let z1_car = z1.car.borrow().clone();
    let z1_cdr = z1.cdr.borrow().clone();
    assert!(eq_pair(&first_pair(&z1_car), &first_pair(&z1_cdr)));

    let set_to_wow = set_to_wow(&z1);
    assert_eq!(Value::Pair(set_to_wow).to_string(), "((wow b) wow b)");
    let _ = Rc::new(0u8);
}
