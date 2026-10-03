// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.25, one module and one test.

mod ex_2_25 {
    use ch02::sec_2_2::{Nest, leaf, sub};
    use sicp_runtime::SicpError;
    use std::rc::Rc;

    /// The `car` of a tree in the book's sense: the first element of a
    /// branch. A leaf has no `car`, and neither has the empty branch,
    /// and unlike Scheme the type system forces those failures to be
    /// values the chain must carry.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] on a leaf or on the empty branch.
    fn car(tree: &Nest<i128>) -> Result<Nest<i128>, SicpError> {
        match tree {
            Nest::Sub(items) => match items.car() {
                Some(first) => Ok(first.clone()),
                None => Err(SicpError::TypeMismatch("car of ()".into())),
            },
            Nest::Leaf(_) => Err(SicpError::TypeMismatch("car of a leaf".into())),
        }
    }

    /// The `cdr` of a tree: the rest of its branch, again as a branch.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] on a leaf or on the empty branch.
    fn cdr(tree: &Nest<i128>) -> Result<Nest<i128>, SicpError> {
        match tree {
            Nest::Sub(items) => match items.cdr() {
                Some(rest) => Ok(Nest::Sub(Rc::new(rest.clone()))),
                None => Err(SicpError::TypeMismatch("cdr of ()".into())),
            },
            Nest::Leaf(_) => Err(SicpError::TypeMismatch("cdr of a leaf".into())),
        }
    }

    /// The `i128` under a leaf, so a chain can end in a number.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] on a branch.
    fn as_leaf(tree: &Nest<i128>) -> Result<i128, SicpError> {
        match tree {
            Nest::Leaf(x) => Ok(*x),
            Nest::Sub(_) => Err(SicpError::TypeMismatch("expected a leaf".into())),
        }
    }

    /// Exercise 2.25: picking 7 with accessor chains
    ///
    /// Each chain is the edition's rendering of a `car`/`cdr`
    /// combination, with `?` standing in for Scheme's trust that the
    /// selectors land on pairs. Through `(1 3 (5 7) 9)` the route is
    /// cdr, cdr, car, cdr, car; through `((7))` it is car, car; through
    /// `(1 (2 (3 (4 (5 (6 7))))))` the chain alternates cdr and car,
    /// once per level, until `(6 7)` gives way to `(7)` and then the
    /// leaf. The alternation is the honest shape of the descent: cdr
    /// peels an element off, car steps into the sublist that element
    /// names.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when a chain walks off a leaf or
    /// an empty branch, which the three chains here never do.
    pub fn ex_2_25() -> Result<[i128; 3], SicpError> {
        let first = sub(&[leaf(1), leaf(3), sub(&[leaf(5), leaf(7)]), leaf(9)]);
        let second = sub(&[sub(&[leaf(7)])]);
        let third = sub(&[
            leaf(1),
            sub(&[
                leaf(2),
                sub(&[
                    leaf(3),
                    sub(&[leaf(4), sub(&[leaf(5), sub(&[leaf(6), leaf(7)])])]),
                ]),
            ]),
        ]);

        let a = cdr(&first)?;
        let b = cdr(&a)?;
        let c = car(&b)?;
        let d = cdr(&c)?;
        let seven_first = as_leaf(&car(&d)?)?;

        let seven_second = as_leaf(&car(&car(&second)?)?)?;

        let mut cursor = cdr(&third)?;
        for _ in 1..6 {
            cursor = car(&cursor)?;
            cursor = cdr(&cursor)?;
        }
        let seven_third = as_leaf(&car(&cursor)?)?;

        Ok([seven_first, seven_second, seven_third])
    }
}

#[test]
fn ex_2_25() {
    assert_eq!(ex_2_25::ex_2_25(), Ok([7, 7, 7]));
}
