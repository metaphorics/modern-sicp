// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.37, one module and one test.

mod ex_2_37 {
    use ch02::sec_2_2::accumulate;

    type Vector = Vec<i64>;
    type Matrix = Vec<Vector>;

    /// The book's `dot-product`, given in the statement: an
    /// `accumulate` of `*` over the elementwise products. The extended
    /// multi-sequence `map` of the book's footnote 78 becomes `zip`.
    fn dot_product(v: &Vector, w: &Vector) -> i64 {
        let products: Vec<i64> = v.iter().zip(w).map(|(a, b)| a * b).collect();
        accumulate(|p, acc: i64| acc + p, 0, &products)
    }

    /// Exercise 2.37: `matrix-*-vector`: each row of the matrix dots
    /// the vector.
    fn matrix_star_vector(m: &Matrix, v: &Vector) -> Vector {
        m.iter().map(|row| dot_product(row, v)).collect()
    }

    /// Exercise 2.37: `transpose`, built on `accumulate-n`: the rows of
    /// the answer are the columns of the input, assembled one element
    /// per sequence at a time. `accumulate` is a right fold, so the
    /// column element enters the row accumulator from the far end and
    /// the insert keeps the left-to-right order.
    fn transpose(mat: &Matrix) -> Matrix {
        let columns = accumulate_n(
            |x, acc: Vector| {
                let mut row = acc;
                row.insert(0, *x);
                row
            },
            Vec::new(),
            mat,
        );
        columns.unwrap_or_default()
    }

    /// Exercise 2.37: `matrix-*-matrix`: row `i` of the answer dots
    /// every column of `n`.
    fn matrix_star_matrix(m: &Matrix, n: &Matrix) -> Matrix {
        let cols = transpose(n);
        m.iter()
            .map(|row| cols.iter().map(|col| dot_product(row, col)).collect())
            .collect()
    }

    /// The book's example matrix, rows `(1 2 3 4)`, `(4 5 6 6)`, and
    /// `(6 7 8 9)`.
    fn book_matrix() -> Matrix {
        vec![vec![1, 2, 3, 4], vec![4, 5, 6, 6], vec![6, 7, 8, 9]]
    }

    /// Exercise 2.36's `accumulate-n` shape, kept as this file's private
    /// copy so the transpose stands on the exercise's own definition.
    fn accumulate_n<T: Clone, A: Clone>(
        op: impl Fn(&T, A) -> A,
        initial: A,
        seqs: &[Vec<T>],
    ) -> Option<Vec<A>> {
        fn go<T: Clone, A: Clone>(
            op: &impl Fn(&T, A) -> A,
            initial: A,
            seqs: &[Vec<T>],
        ) -> Option<Vec<A>> {
            let first = seqs.first()?;
            if first.is_empty() {
                return Some(Vec::new());
            }
            let firsts: Vec<T> = seqs.iter().map(|seq| seq[0].clone()).collect();
            let rests: Vec<Vec<T>> = seqs.iter().map(|seq| seq[1..].to_vec()).collect();
            let mut heads = vec![accumulate(op, initial.clone(), &firsts)];
            let tails = go(op, initial, &rests)?;
            heads.extend(tails);
            Some(heads)
        }
        go(&op, initial, seqs)
    }

    /// Exercise 2.37: matrix operations
    ///
    /// Returns the dot product of `(1 2 3 4)` with itself, the product
    /// of the book's matrix with the vector `(1 2 3 4)`, the matrix
    /// transpose, and the matrix's square, in that order.
    pub fn ex_2_37() -> (i64, Vector, Matrix, Matrix) {
        let m = book_matrix();
        let v = vec![1, 2, 3, 4];
        (
            dot_product(&v, &v),
            matrix_star_vector(&m, &v),
            transpose(&m),
            matrix_star_matrix(&m, &m),
        )
    }
}

#[test]
fn ex_2_37() {
    assert_eq!(
        ex_2_37::ex_2_37(),
        (
            30,
            vec![30, 56, 80],
            vec![vec![1, 4, 6], vec![2, 5, 7], vec![3, 6, 8], vec![4, 6, 9]],
            vec![
                vec![27, 33, 39, 43],
                vec![60, 75, 90, 100],
                vec![82, 103, 124, 138]
            ]
        )
    );
}
