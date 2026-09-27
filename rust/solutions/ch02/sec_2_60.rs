// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.60, one module and one test.

mod ex_2_60 {
    /// Duplicates-allowed sets: membership is unchanged from the
    /// no-duplicate representation, still `Θ(n)`.
    fn element_of_set_dup(x: i128, set: &[i128]) -> bool {
        set.contains(&x)
    }

    /// Duplicates-allowed sets: adjoining never checks membership, so
    /// it is `Θ(1)` instead of the no-duplicate representation's
    /// `Θ(n)` — the whole point of allowing duplicates.
    fn adjoin_set_dup(x: i128, set: &[i128]) -> Vec<i128> {
        let mut out = vec![x];
        out.extend_from_slice(set);
        out
    }

    /// Duplicates-allowed sets: union is plain concatenation, `Θ(n)`
    /// instead of the no-duplicate representation's `Θ(n)` membership
    /// scan per element (same order, but no per-element check, and
    /// the result can be twice the size).
    fn union_set_dup(set1: &[i128], set2: &[i128]) -> Vec<i128> {
        let mut out = set1.to_vec();
        out.extend_from_slice(set2);
        out
    }

    /// Duplicates-allowed sets: intersection still scans `set2` for
    /// every element of `set1`, `Θ(n²)`, unchanged from the
    /// no-duplicate representation, and duplicates in `set1` produce
    /// duplicates in the result.
    fn intersection_set_dup(set1: &[i128], set2: &[i128]) -> Vec<i128> {
        set1.iter()
            .copied()
            .filter(|&x| element_of_set_dup(x, set2))
            .collect()
    }

    /// Exercise 2.60: sets allowing duplicates
    ///
    /// Returns, on the duplicate set `[2, 3, 2, 1, 3, 2, 2]`
    /// representing `{1, 2, 3}`: `adjoin_set_dup(1, set)`;
    /// `union_set_dup(set, &[1, 4])`; `intersection_set_dup(set, &[1, 3])`;
    /// and whether `1` is a member of `set`, in that order. Adjoining
    /// and union trade the smaller result size of the no-duplicate
    /// representation for a constant-time insert and a linear-time
    /// union, which favors applications that adjoin and union far more
    /// often than they scan for membership or intersect.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's four-part statement"
    )]
    pub fn ex_2_60() -> (Vec<i128>, Vec<i128>, Vec<i128>, bool) {
        let set = [2_i128, 3, 2, 1, 3, 2, 2];
        (
            adjoin_set_dup(1, &set),
            union_set_dup(&set, &[1, 4]),
            intersection_set_dup(&set, &[1, 3]),
            element_of_set_dup(1, &set),
        )
    }
}

#[test]
fn ex_2_60() {
    assert_eq!(
        ex_2_60::ex_2_60(),
        (
            vec![1, 2, 3, 2, 1, 3, 2, 2],
            vec![2, 3, 2, 1, 3, 2, 2, 1, 4],
            vec![3, 1, 3],
            true,
        )
    );
}
