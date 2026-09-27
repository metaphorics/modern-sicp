// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.12: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_12 {
    /// An element of Pascal's triangle, rows and columns counted from
    /// zero: the edges are 1, and each interior element is the sum of
    /// the two above it.
    fn pascal(row: u32, col: u32) -> u64 {
        if col == 0 || col == row {
            1
        } else {
            pascal(row - 1, col - 1) + pascal(row - 1, col)
        }
    }

    /// Exercise 1.12: elements of Pascal's triangle
    ///
    /// Returns the elements at `(row 4, column 2)`, `(row 6, column 3)`,
    /// and `(row 0, column 0)` in that order, counting rows and columns
    /// from zero.
    pub fn ex_1_12() -> [u64; 3] {
        [pascal(4, 2), pascal(6, 3), pascal(0, 0)]
    }
}

#[test]
fn ex_1_12() {
    let values = ex_1_12::ex_1_12();
    assert_eq!(values, [6, 20, 1]);
}
