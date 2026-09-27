// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.40, one module and one test.

mod ex_2_40 {
    use ch02::sec_2_2::is_prime;

    /// Exercise 2.40: `unique-pairs`: the nested mapping that produces
    /// `(i, j)` with `1 <= j < i <= n`, lifted out of
    /// `prime-sum-pairs` so the enumerating detail disappears from the
    /// procedure that uses it.
    fn unique_pairs(n: i64) -> Vec<(i64, i64)> {
        (1..=n).flat_map(|i| (1..i).map(move |j| (i, j))).collect()
    }

    /// `prime-sum-pairs`, simplified by `unique_pairs` to a filter and
    /// a map over the pairs.
    fn prime_sum_pairs(n: i64) -> Vec<(i64, i64, i64)> {
        unique_pairs(n)
            .into_iter()
            .filter(|(i, j)| is_prime(i + j))
            .map(|(i, j)| (i, j, i + j))
            .collect()
    }

    /// Renders the triples in the book's printed form, `((2 1 3) ...)`.
    fn render(triples: &[(i64, i64, i64)]) -> String {
        let inner: Vec<String> = triples
            .iter()
            .map(|(i, j, s)| format!("({i} {j} {s})"))
            .collect();
        format!("({})", inner.join(" "))
    }

    /// Exercise 2.40: unique-pairs
    ///
    /// Returns the rendered `prime_sum_pairs` for `n = 6`, simplified
    /// by `unique_pairs`.
    pub fn ex_2_40() -> String {
        render(&prime_sum_pairs(6))
    }
}

#[test]
fn ex_2_40() {
    assert_eq!(
        ex_2_40::ex_2_40(),
        "((2 1 3) (3 2 5) (4 1 5) (4 3 7) (5 2 7) (6 1 7) (6 5 11))".to_string()
    );
}
