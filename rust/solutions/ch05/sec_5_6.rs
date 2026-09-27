// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.6: the Fibonacci machine's
//! redundant save and restore, removed and shown unnecessary.

use ch05::sec_5_1::{Value, fibonacci, fibonacci_without_redundant_pair};

mod ex_5_06 {
    //! Exercise 5.6: find the Fibonacci machine's redundant save and
    //! restore.

    use super::*;

    /// The answer: the `(restore continue)` at `afterfib-n-1` and the
    /// `(save continue)` in the second setup block. The restore
    /// brings back the caller's return label only for the next save
    /// to push it straight back; removing both leaves the stack
    /// balanced, because the final `(restore continue)` at
    /// `afterfib-n-2` then pops the copy saved at the first setup.
    /// The trace evidence is exercise 5.5a's steps 20/22 and 36/38.
    #[test]
    fn ex_5_06() {
        // Both machines compute Fibonacci for every n up to 10.
        for n in 0..=10 {
            let expected = Value::Int(fib_direct(n));
            let original = fibonacci().run(&[("n", Value::Int(n))]).expect("run");
            let pruned = fibonacci_without_redundant_pair()
                .run(&[("n", Value::Int(n))])
                .expect("run");
            assert_eq!(original.value_of("val"), expected);
            assert_eq!(pruned.value_of("val"), expected);
        }
        // At n = 6 the original machine executes 281 instructions
        // with 48 pushes; the pruned machine 257 and 36: one save and
        // one restore per internal call, twelve of them.
        let original = fibonacci().run(&[("n", Value::Int(6))]).expect("run");
        assert_eq!(original.instructions, 281);
        assert_eq!(original.pushes, 48);
        let pruned = fibonacci_without_redundant_pair()
            .run(&[("n", Value::Int(6))])
            .expect("run");
        assert_eq!(pruned.instructions, 257);
        assert_eq!(pruned.pushes, 36);
        assert_eq!(pruned.value_of("val"), Value::Int(8));
    }

    /// The host's own Fibonacci, for the direct comparison.
    fn fib_direct(n: i64) -> i64 {
        if n < 2 {
            n
        } else {
            fib_direct(n - 1) + fib_direct(n - 2)
        }
    }
}
