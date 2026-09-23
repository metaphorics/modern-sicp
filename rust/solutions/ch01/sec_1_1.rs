// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.1: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_01 {
    /// Exercise 1.1: evaluate a sequence of expressions in order
    ///
    /// Rust has no read-eval-print loop, so the sequence runs inside one
    /// function, evaluated in the order it is presented; each numeric
    /// result is what the program would print at that step. The two
    /// `let` bindings and the `a == b` comparison produce no number.
    pub fn ex_1_01() -> Vec<i64> {
        let mut printed: Vec<i64> = vec![10, 5 + 3 + 4, 9 - 1, 6 / 2, 2 * 4 + (4 - 6)];
        let a: i64 = 3;
        let b: i64 = a + 1;
        printed.push(a + b + a * b);
        let _ = a == b;
        printed.push(if b > a && b < a * b { b } else { a });
        printed.push(match a {
            _ if a == 4 => 6,
            _ if b == 4 => 6 + 7 + a,
            _ => 25,
        });
        printed.push(2 + if b > a { b } else { a });
        printed.push(
            match a.cmp(&b) {
                std::cmp::Ordering::Greater => a,
                std::cmp::Ordering::Less => b,
                std::cmp::Ordering::Equal => -1,
            } * (a + 1),
        );
        printed
    }
}

#[test]
fn ex_1_01() {
    assert_eq!(ex_1_01::ex_1_01(), vec![10, 12, 8, 3, 6, 19, 4, 16, 6, 16]);
}
