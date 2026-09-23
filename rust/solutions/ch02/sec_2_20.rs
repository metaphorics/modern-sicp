// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.20, one module and one test.

mod ex_2_20 {
    use ch02::sec_2_2::List;

    /// Exercise 2.20: `same-parity`
    ///
    /// Scheme's dotted-tail notation becomes a Rust rest parameter: a
    /// fixed first parameter plus a slice holding all remaining
    /// arguments, which is exactly the shape `(define (f x . y) ...)`
    /// gives the book. The first argument leads the answer, and the
    /// filter keeps the arguments that share its parity.
    fn same_parity(first: i128, rest: &[i128]) -> List<i128> {
        let parity = first.rem_euclid(2);
        std::iter::once(first)
            .chain(rest.iter().copied())
            .filter(|x| x.rem_euclid(2) == parity)
            .collect()
    }

    /// Exercise 2.20: same-parity with a rest parameter
    ///
    /// Returns the rendered argument lists that share the first
    /// argument's parity for `same_parity(1, [2, 3, 4, 5, 6, 7])` and
    /// `same_parity(2, [3, 4, 5, 6, 7])`, in that order.
    pub fn ex_2_20() -> (String, String) {
        let odd_case = same_parity(1, &[2, 3, 4, 5, 6, 7]);
        let even_case = same_parity(2, &[3, 4, 5, 6, 7]);
        (odd_case.to_string(), even_case.to_string())
    }
}

#[test]
fn ex_2_20() {
    assert_eq!(
        ex_2_20::ex_2_20(),
        ("(1 3 5 7)".to_string(), "(2 4 6)".to_string())
    );
}
