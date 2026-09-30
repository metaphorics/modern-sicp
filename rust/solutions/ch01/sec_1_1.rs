// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.1: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_01 {
    /// A value printed by the exercise's sequence.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Printed {
        /// An integer result.
        Number(i64),
        /// A Boolean result.
        Boolean(bool),
    }

    /// Exercise 1.1: evaluate a sequence of expressions in order.
    ///
    /// The two `let` bindings are silent; the comparison contributes
    /// a Boolean between the sixth and eighth printed values.
    pub fn ex_1_01() -> Vec<Printed> {
        use self::Printed::{Boolean, Number};

        let mut printed = Vec::with_capacity(11);
        printed.extend([
            Number(10),
            Number(5 + 3 + 4),
            Number(9 - 1),
            Number(6 / 2),
            Number(2 * 4 + (4 - 6)),
        ]);
        let a: i64 = 3;
        let b: i64 = a + 1;
        printed.push(Number(a + b + a * b));
        printed.push(Boolean(a == b));
        printed.push(Number(if b > a && b < a * b { b } else { a }));
        printed.push(Number(match a {
            _ if a == 4 => 6,
            _ if b == 4 => 6 + 7 + a,
            _ => 25,
        }));
        printed.push(Number(2 + if b > a { b } else { a }));
        printed.push(Number(
            match a.cmp(&b) {
                std::cmp::Ordering::Greater => a,
                std::cmp::Ordering::Less => b,
                std::cmp::Ordering::Equal => -1,
            } * (a + 1),
        ));
        printed
    }
}

#[test]
fn ex_1_01() {
    use ex_1_01::Printed::{Boolean, Number};

    assert_eq!(
        ex_1_01::ex_1_01(),
        vec![
            Number(10),
            Number(12),
            Number(8),
            Number(3),
            Number(6),
            Number(19),
            Boolean(false),
            Number(4),
            Number(16),
            Number(6),
            Number(16),
        ]
    );
}
