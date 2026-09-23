// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.36, one module and one test.

mod ex_2_36 {
    use ch02::sec_2_2::accumulate;

    /// Exercise 2.36: `accumulate-n`
    ///
    /// The book's fill: combine all the first elements with
    /// `accumulate`, then recurse on the sequence of all the `cdr`s.
    /// Where the first sequence has run out of elements the answer is
    /// `nil`, which this edition states as the empty vector.
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

    /// Exercise 2.36: accumulate-n
    ///
    /// Returns the column sums of `((1 2 3) (4 5 6) (7 8 9) (10 11 12))`,
    /// which are `(22 26 30)`.
    pub fn ex_2_36() -> Vec<i64> {
        let seqs = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ];
        accumulate_n(|x, acc: i64| acc + x, 0, &seqs)
            .expect("all four sequences have the same length")
    }
}

#[test]
fn ex_2_36() {
    assert_eq!(ex_2_36::ex_2_36(), vec![22, 26, 30]);
}
