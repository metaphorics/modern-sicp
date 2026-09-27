// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.2.2

//! Section 3.2.2: applying simple procedures — the call of `f(5)` and
//! the frames of Figure 3.5, each one a real child of the global frame.

use std::rc::Rc;

use ch03::sec_3_2::{define, f, int_of, square};
use sicp_runtime::{Env, Value};

fn main() {
    // The call from the section text: each frame the evaluation builds
    // is drawn in Figure 3.5, and the answer is 136.
    let answer = f(5);
    println!("{answer}");
    // => 136
    assert_eq!(answer, 136);

    // The same call, with the figure's frames built as real nodes. E1
    // is the call frame of f, holding a: 5.
    let global = Env::global();
    let e1 = Env::child(&global);
    define(&e1, "a", Value::int(5));
    assert_eq!(int_of(&e1, "a"), Ok(5));

    // The operands of f's body, evaluated in E1: 5 + 1 and 5 * 2.
    let a = int_of(&e1, "a").unwrap_or_default();
    let left_operand = a + 1;
    let right_operand = a * 2;
    assert_eq!((left_operand, right_operand), (6, 10));

    // E2 is the frame of the sum_of_squares call; the two parameters
    // are bound to the operands just computed.
    let e2 = Env::child(&global);
    define(&e2, "x", Value::int(left_operand));
    define(&e2, "y", Value::int(right_operand));
    assert_eq!(int_of(&e2, "x"), Ok(6));
    assert_eq!(int_of(&e2, "y"), Ok(10));

    // E3 and E4 are the frames of the two square calls, each with its
    // own x: the frames keep the two locals that share the name apart.
    let e3 = Env::child(&global);
    define(&e3, "x", Value::int(int_of(&e2, "x").unwrap_or_default()));
    let e4 = Env::child(&global);
    define(&e4, "x", Value::int(int_of(&e2, "y").unwrap_or_default()));
    assert_eq!(int_of(&e3, "x"), Ok(6));
    assert_eq!(int_of(&e4, "x"), Ok(10));

    // The two calls answer from their own frames by evaluating square's
    // body, x * x, and sum_of_squares adds the results.
    let first = square(int_of(&e3, "x").unwrap_or_default());
    let second = square(int_of(&e4, "x").unwrap_or_default());
    let answer = first + second;
    println!("{answer}");
    // => 136
    assert_eq!(answer, 136);

    // Every frame the call built extends the global environment,
    // because that is the environment part of these plain fns.
    assert!(
        e1.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, &global))
    );
    assert!(
        e2.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, &global))
    );
    assert!(
        e3.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, &global))
    );
    assert!(
        e4.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, &global))
    );

    // The two x frames are distinct frames, not one shared slot.
    assert!(!Rc::ptr_eq(&e3, &e4));
}
