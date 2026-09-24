// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 3.10 and its tailored addition
//! 3.10a, one module and one ignored test each.

mod ex_3_10 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.10`.
        pub exercise: &'static str,
    }

    /// Exercise 3.10: let desugaring adds a frame
    ///
    /// Returns the replies of the parameter-captured withdrawal
    /// processor to the book's interaction, the replies of the
    /// let-bound version, and the `size_of_val` measurements of one
    /// closure of each version, in that order.
    pub fn ex_3_10() -> Result<(Vec<i128>, Vec<i128>, usize, usize), Pending> {
        Err(Pending { exercise: "3.10" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_10() {
    let (parameter_replies, let_replies, parameter_size, let_size) =
        ex_3_10::ex_3_10().expect("solved");
    assert_eq!(parameter_replies, vec![50]);
    assert_eq!(let_replies, vec![50]);
    assert_eq!(parameter_size, size_of::<i128>());
    assert_eq!(let_size, size_of::<i128>());
}

mod ex_3_10a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.10a`.
        pub exercise: &'static str,
    }

    /// Exercise 3.10a: shared-cell capture, pointer versus binding
    ///
    /// Returns the replies of two withdrawal processors over clones of
    /// one shared cell, the cell's final value, the size of one such
    /// closure, and the replies of two processors built over copied
    /// integers, in that order.
    pub fn ex_3_10a() -> Result<(Vec<i128>, i128, usize, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.10a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_10a() {
    let (shared_replies, cell_value, closure_size, copied_replies) =
        ex_3_10a::ex_3_10a().expect("solved");
    assert_eq!(shared_replies, vec![70, 50]);
    assert_eq!(cell_value, 50);
    assert_eq!(
        closure_size,
        size_of::<std::rc::Rc<std::cell::Cell<i128>>>()
    );
    assert_eq!(copied_replies, vec![70, 80]);
}
