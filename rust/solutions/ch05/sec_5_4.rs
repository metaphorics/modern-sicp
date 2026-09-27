// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.4: recursive and iterative
//! exponentiation machines, both run.

use ch05::sec_5_1::{Value, expt_iterative, expt_recursive};

mod ex_5_04 {
    //! Exercise 5.4: controller sequences for recursive and iterative
    //! exponentiation.

    use super::*;

    /// The recursive machine uses the save/restore discipline of the
    /// book's Figure 5.11: `continue` and `n` are saved before the
    /// recursive call and restored after it, then `val` receives
    /// `b * val`. The iterative machine is the product/counter loop
    /// with no stack. Both transcriptions live in
    /// [`ch05::sec_5_1::expt_recursive`] and
    /// [`ch05::sec_5_1::expt_iterative`].
    #[test]
    fn ex_5_04() {
        for (b, n, power) in [(2, 5, 32), (3, 4, 81)] {
            let init = [("b", Value::Int(b)), ("n", Value::Int(n))];
            let recursive = expt_recursive().run(&init).expect("run");
            let iterative = expt_iterative().run(&init).expect("run");
            assert_eq!(recursive.value_of("val"), Value::Int(power));
            assert_eq!(iterative.value_of("product"), Value::Int(power));
        }
        // The recursive machine pays two pushes per level, to depth
        // 2n; the iterative machine never touches the stack.
        let deep = expt_recursive()
            .run(&[("b", Value::Int(2)), ("n", Value::Int(5))])
            .expect("run");
        assert_eq!(deep.instructions, 60);
        assert_eq!(deep.pushes, 10);
        assert_eq!(deep.pops, 10);
        assert_eq!(deep.max_depth, 10);
        let flat = expt_iterative()
            .run(&[("b", Value::Int(2)), ("n", Value::Int(5))])
            .expect("run");
        assert_eq!(flat.instructions, 29);
        assert_eq!(flat.pushes, 0);
    }
}
