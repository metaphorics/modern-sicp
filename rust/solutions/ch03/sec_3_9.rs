// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.9: the call stacks of two
//! factorials, made measurable.

mod ex_3_09 {
    use std::cell::Cell;

    /// The book's recursive factorial (section 1.2.1), carrying the
    /// call depth so the frame stack the exercise asks to draw can be
    /// checked against the drawing: every call builds a stack frame
    /// holding its own `n`, and none of them goes away before the
    /// deepest one is reached.
    #[must_use]
    pub fn factorial_recursive(n: u64, depth: u64, deepest: &Cell<u64>) -> u128 {
        if depth > deepest.get() {
            deepest.set(depth);
        }
        if n <= 1 {
            1
        } else {
            u128::from(n) * factorial_recursive(n - 1, depth + 1, deepest)
        }
    }

    /// The book's iterative factorial as a loop: one frame for the
    /// whole computation, so the depth counter never leaves the depth
    /// it started at.
    #[must_use]
    pub fn factorial_loop(n: u64, deepest: &Cell<u64>) -> u128 {
        if 1 > deepest.get() {
            deepest.set(1);
        }
        let mut product = 1;
        for counter in 1..=n {
            product *= u128::from(counter);
        }
        product
    }

    /// Exercise 3.9: environment structures of two factorials
    ///
    /// Returns value and deepest frame depth for the recursive
    /// factorial at 6, then the same pair for the loop version.
    #[must_use]
    pub fn ex_3_09() -> (u128, u64, u128, u64) {
        let recursive_deepest = Cell::new(0);
        let recursive = factorial_recursive(6, 1, &recursive_deepest);
        let loop_deepest = Cell::new(0);
        let iterative = factorial_loop(6, &loop_deepest);
        (
            recursive,
            recursive_deepest.get(),
            iterative,
            loop_deepest.get(),
        )
    }
}

#[test]
fn ex_3_09() {
    use std::cell::Cell;

    // Both versions answer 720; only the stacks differ. The recursive
    // stack at its deepest holds one frame per call, for n = 6 down to
    // n = 1; the loop never leaves the one frame it started in.
    assert_eq!(ex_3_09::ex_3_09(), (720, 6, 720, 1));

    // The recursive depth is one frame per call, whatever the answer:
    // nine calls down for 9 factorial.
    let deepest = Cell::new(0);
    assert_eq!(ex_3_09::factorial_recursive(9, 1, &deepest), 362_880);
    assert_eq!(deepest.get(), 9);

    // The loop stays at depth 1 even for a value that dwarfs 720.
    let deepest = Cell::new(0);
    assert_eq!(
        ex_3_09::factorial_loop(20, &deepest),
        2_432_902_008_176_640_000
    );
    assert_eq!(deepest.get(), 1);
}
