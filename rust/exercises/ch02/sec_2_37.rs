// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.37, one module and one ignored
//! test.

mod ex_2_37 {
    use sicp_runtime::Pending;

    /// Exercise 2.37: matrix operations
    ///
    type Matrix = Vec<Vec<i64>>;

    /// Returns the dot product of `(1 2 3 4)` with itself, the product of the
    /// book's matrix with the vector `(1 2 3 4)`, the matrix transpose, and
    /// the matrix's square, in that order.
    pub fn ex_2_37() -> Result<(i64, Vec<i64>, Matrix, Matrix), Pending> {
        Err(Pending::new("2.37"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_37() {
    assert_eq!(
        ex_2_37::ex_2_37(),
        Ok((
            30,
            vec![30, 56, 80],
            vec![vec![1, 4, 6], vec![2, 5, 7], vec![3, 6, 8], vec![4, 6, 9]],
            vec![
                vec![27, 33, 39, 43],
                vec![60, 75, 90, 100],
                vec![82, 103, 124, 138]
            ]
        ))
    );
}
