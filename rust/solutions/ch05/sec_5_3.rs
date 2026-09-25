// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.3: the square-root machine in
//! two stages, both run.

use ch05::sec_5_1::{Value, sqrt_expanded, sqrt_primitive};

mod ex_5_03 {
    //! Exercise 5.3: design a square-root machine on Newton's method,
    //! first treating `good-enough?` and `improve` as primitives, then
    //! expanding them into the arithmetic operations.

    use super::*;

    /// The data-path descriptions the exercise asks to draw, in the
    /// book's notation; the two controller definitions are the
    /// transcriptions in [`ch05::sec_5_1::sqrt_primitive`] and
    /// [`ch05::sec_5_1::sqrt_expanded`].
    ///
    /// ```text
    /// First stage, the two compound operations as primitive boxes:
    ///
    ///     guess, x --> (good-enough?) --> to the controller (test)
    ///     guess, x --> (improve) -------> guess
    ///
    /// Second stage, arithmetic only, with t holding each
    /// intermediate value:
    ///
    ///     guess, guess --> (*) ---> t
    ///     t, x          --> (-) ---> t
    ///     t             --> (abs) -> t
    ///     t, 0.001      --> (<) ---> to the controller (test)
    ///     x, guess      --> (/) ---> t
    ///     guess, t      --> (+) ---> t
    ///     t, 2          --> (/) ---> guess
    /// ```
    #[test]
    #[expect(
        clippy::unreadable_literal,
        reason = "the pins reproduce the model's printed values verbatim"
    )]
    fn ex_5_03() {
        // x = 9 converges to 3.00009155413138 in five improvements;
        // the next test's |guess^2 - x| is below 0.001.
        let nine_p = sqrt_primitive().run(&[("x", Value::Int(9))]).expect("run");
        let nine_e = sqrt_expanded().run(&[("x", Value::Int(9))]).expect("run");
        assert_eq!(nine_p.value_of("guess"), Value::Real(3.00009155413138));
        assert_eq!(nine_e.value_of("guess"), Value::Real(3.00009155413138));
        // x = 2 converges to the classic 1.4142156862745097.
        let two_p = sqrt_primitive().run(&[("x", Value::Int(2))]).expect("run");
        let two_e = sqrt_expanded().run(&[("x", Value::Int(2))]).expect("run");
        assert_eq!(two_p.value_of("guess"), Value::Real(1.4142156862745097));
        assert_eq!(two_e.value_of("guess"), Value::Real(1.4142156862745097));
        // Both stages agree, and each answer is good enough.
        for (primitive, expanded, x) in [(&nine_p, &nine_e, 9.0), (&two_p, &two_e, 2.0)] {
            assert_eq!(primitive.value_of("guess"), expanded.value_of("guess"));
            let Value::Real(guess) = primitive.value_of("guess") else {
                panic!("guess is not a real")
            };
            assert!((guess * guess - x).abs() < 0.001);
        }
    }
}
