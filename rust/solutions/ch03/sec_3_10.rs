// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercise 3.10 and its tailored addition
//! 3.10a: two versions of the withdrawal processor with the same
//! behavior, and what a `move` closure copies when the state lives
//! behind an `Rc` — the pointer, not the binding.

use ch03::sec_3_1::Reply;

mod ex_3_10 {
    use std::mem::size_of_val;

    use ch03::sec_3_1::{Reply, make_withdraw};

    /// The let-bound version from the exercise statement: an inner
    /// binding in the factory frame, and a `move` closure that names
    /// only `balance`.
    pub fn make_withdraw_let(initial_amount: i128) -> impl FnMut(i128) -> Reply {
        let mut balance = initial_amount;
        move |amount| {
            if balance >= amount {
                balance -= amount;
                Reply::Balance(balance)
            } else {
                Reply::Message("Insufficient funds")
            }
        }
    }

    /// The `size_of_val` measurement of one closure, in bytes: the
    /// width of the captured state is the width of the procedure
    /// object's environment part.
    #[must_use]
    pub fn closure_size<F: FnMut(i128) -> Reply>(closure: &F) -> usize {
        size_of_val(closure)
    }

    /// Exercise 3.10: let desugaring adds a frame
    ///
    /// Runs the book's interaction on both versions and measures one
    /// closure of each. Returns the replies of the parameter-captured
    /// version, the replies of the let-bound version, and the two
    /// closure sizes, in that order.
    #[must_use]
    pub fn ex_3_10() -> (Vec<Reply>, Vec<Reply>, usize, usize) {
        let mut w1 = make_withdraw(100);
        let parameter_replies = vec![w1(50)];

        let mut w2 = make_withdraw_let(100);
        let let_replies = vec![w2(50)];

        let parameter_size = closure_size(&w1);
        let let_size = closure_size(&w2);
        (parameter_replies, let_replies, parameter_size, let_size)
    }
}

#[test]
fn ex_3_10() {
    use std::mem::size_of;

    let (parameter_replies, let_replies, parameter_size, let_size) = ex_3_10::ex_3_10();

    // Same behavior: both versions answer the book's interaction alike.
    assert_eq!(parameter_replies, vec![Reply::Balance(50)]);
    assert_eq!(let_replies, vec![Reply::Balance(50)]);

    // Same capture structure: each closure is exactly as wide as the
    // one integer it owns, so the extra let of the second version built
    // no extra frame for the compiler to keep. The balance lives inside
    // the closure in both.
    assert_eq!(parameter_size, size_of::<i128>());
    assert_eq!(let_size, size_of::<i128>());

    // Independence holds in both versions, object against object.
    let mut first = ex_3_10::make_withdraw_let(100);
    let mut second = ex_3_10::make_withdraw_let(100);
    assert_eq!(first(60), Reply::Balance(40));
    assert_eq!(second(70), Reply::Balance(30));
    assert_eq!(first(50), Reply::Message("Insufficient funds"));
}

mod ex_3_10a {
    use std::cell::Cell;
    use std::mem::size_of_val;
    use std::rc::Rc;

    use ch03::sec_3_1::{Reply, make_withdraw};

    /// The shared-cell withdrawal processor from the exercise
    /// statement: the `move` closure captures the `Rc` handle, so each
    /// factory call moves in a clone of the pointer and both objects
    /// keep pointing at one cell.
    pub fn make_withdraw_cell(balance: Rc<Cell<i128>>) -> impl FnMut(i128) -> Reply {
        move |amount| {
            if balance.get() >= amount {
                balance.set(balance.get() - amount);
                Reply::Balance(balance.get())
            } else {
                Reply::Message("Insufficient funds")
            }
        }
    }

    /// Exercise 3.10a: shared-cell capture, pointer versus binding
    ///
    /// Builds two processors over clones of one cell and drains both,
    /// then two processors over copied integers and drains those.
    /// Returns the shared-cell replies, the cell's final value, the
    /// size of one shared-cell closure, and the copied-integer replies,
    /// in that order.
    #[must_use]
    pub fn ex_3_10a() -> (Vec<Reply>, i128, usize, Vec<Reply>) {
        let cell = Rc::new(Cell::new(100));
        let mut w1 = make_withdraw_cell(Rc::clone(&cell));
        let mut w2 = make_withdraw_cell(Rc::clone(&cell));
        let shared_replies = vec![w1(30), w2(20)];
        let cell_value = cell.get();
        let closure_size = size_of_val(&w1);

        let mut c1 = make_withdraw(100);
        let mut c2 = make_withdraw(100);
        let copied_replies = vec![c1(30), c2(20)];

        (shared_replies, cell_value, closure_size, copied_replies)
    }
}

#[test]
fn ex_3_10a() {
    use std::cell::Cell;
    use std::mem::size_of;
    use std::rc::Rc;

    let (shared_replies, cell_value, closure_size, copied_replies) = ex_3_10a::ex_3_10a();

    // One balance served both calls: the second withdrawal saw the
    // first, because each Rc::clone copied the pointer and the two
    // closures kept pointing at the same cell.
    assert_eq!(shared_replies, vec![Reply::Balance(70), Reply::Balance(50)]);
    assert_eq!(cell_value, 50);

    // With the integer copied, each capture took its own binding: the
    // same two calls now answer independently.
    assert_eq!(copied_replies, vec![Reply::Balance(70), Reply::Balance(80)]);

    // The closure is as wide as the handle it captured: a pointer and
    // its two counts, not the balance itself. The difference from
    // exercise 3.10's eight-byte capture measures exactly the
    // indirection that makes the state observable from outside.
    assert_eq!(closure_size, size_of::<Rc<Cell<i128>>>());

    // The outside handle watches the shared state: it reads the same 50
    // the two calls left behind.
    let cell = Rc::new(Cell::new(100));
    let mut watcher = ex_3_10a::make_withdraw_cell(Rc::clone(&cell));
    assert_eq!(watcher(40), Reply::Balance(60));
    assert_eq!(cell.get(), 60);
}
