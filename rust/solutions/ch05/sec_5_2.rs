// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.2: the iterative factorial
//! controller sequence in the register-machine language.

use ch05::sec_5_1::{Value, factorial_iterative};

mod ex_5_02 {
    //! Exercise 5.2: describe the iterative factorial machine of
    //! exercise 5.1 in the register-machine language.

    use super::*;

    /// The controller sequence the exercise asks for, in the book's
    /// notation. The machine's transcription is the same sequence as
    /// data:
    ///
    /// ```text
    /// (controller
    ///    (assign product (const 1))
    ///    (assign counter (const 1))
    ///  test-counter
    ///    (test (op >) (reg counter) (reg n))
    ///    (branch (label factorial-done))
    ///    (assign product (op *) (reg counter) (reg product))
    ///    (assign counter (op +) (reg counter) (const 1))
    ///    (goto (label test-counter))
    ///  factorial-done)
    /// ```
    #[test]
    #[expect(
        clippy::unreadable_literal,
        reason = "the pin reproduces the model's printed value verbatim"
    )]
    fn ex_5_02() {
        let machine = factorial_iterative();
        let four = machine.run(&[("n", Value::Int(4))]).expect("run");
        assert_eq!(four.value_of("product"), Value::Int(24));
        let ten = machine.run(&[("n", Value::Int(10))]).expect("run");
        assert_eq!(ten.value_of("product"), Value::Int(3628800));
        assert_eq!(four.pushes, 0);
        assert_eq!(ten.pushes, 0);
    }
}
