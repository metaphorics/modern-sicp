// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution tests of section 0.3: one test per
//! exercise with its statement in a doc comment, and shared code in
//! the matching src module.

/// The reference solution of exercise 0.3: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_03 {
    /// Exercise 0.3: predict-then-compile
    ///
    /// Five cases, A through E, are documented on
    /// [`ch00::sec_0_3`](../../crates/ch00/src/sec_0_3.rs): case A moves
    /// then re-reads (rejected), B borrows then mutates after the borrow's
    /// last use (accepted), C borrows and mutates while the borrow is
    /// still live (rejected), D returns a reference with no lifetime to
    /// name (rejected), and E clones instead of moving (accepted).
    pub fn predictions() -> [bool; 5] {
        [false, true, false, false, true]
    }
}

#[test]
fn ex_0_03() {
    assert_eq!(ex_0_03::predictions(), ch00::sec_0_3::verdicts());
}
