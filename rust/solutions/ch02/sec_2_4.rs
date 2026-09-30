// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.4: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_04 {
    /// An alternative procedural pair: `cons(x, y)` returns a value whose
    /// only operation is `apply`, handing both parts to a selector
    /// closure at once, the message-passing shape the book's procedural
    /// pair has. Rust
    /// fixes a closure's return type at its own definition, so `apply` is
    /// a generic method rather than a second closure layer: that is what
    /// lets `car` and `cdr` below request two different result types from
    /// the same pair.
    struct Cons<A, B> {
        x: A,
        y: B,
    }

    impl<A: Clone, B: Clone> Cons<A, B> {
        fn new(x: A, y: B) -> Self {
            Cons { x, y }
        }

        fn apply<R>(&self, selector: impl FnOnce(&A, &B) -> R) -> R {
            selector(&self.x, &self.y)
        }
    }

    /// Exercise 2.4's `car`: selects the first part.
    fn car<A: Clone, B: Clone>(z: &Cons<A, B>) -> A {
        z.apply(|p, _q| p.clone())
    }

    /// Exercise 2.4's `cdr`: selects the second part, applying the
    /// selector that answers its second argument instead of `car`'s,
    /// which answers its first.
    fn cdr<A: Clone, B: Clone>(z: &Cons<A, B>) -> B {
        z.apply(|_p, q| q.clone())
    }

    /// Exercise 2.4: an alternative procedural representation of pairs
    ///
    /// Returns `car(cons(3, 4))` and `cdr(cons(3, 4))`, verifying that
    /// this representation's `car` and `cdr` recover both parts.
    pub fn ex_2_04() -> (i64, i64) {
        let z = Cons::new(3, 4);
        (car(&z), cdr(&z))
    }
}

#[test]
fn ex_2_04() {
    assert_eq!(ex_2_04::ex_2_04(), (3, 4));
}
