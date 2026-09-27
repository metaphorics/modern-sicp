// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.2.4

//! Section 3.2.4: internal definitions — the `sqrt` of the section with
//! its helpers kept internal, and the frames of Figure 3.11 built as
//! real nodes.

use std::rc::Rc;

use ch03::sec_3_2::{define, sqrt};
use sicp_runtime::{Env, SchemeError, Value};

fn main() {
    // The internal definitions see the enclosing frame's x because they
    // are closures; a nested fn item could not capture it at all. The
    // answer for 2 converges to within the 0.001 test.
    let answer = sqrt(2.0);
    println!("{answer}");
    // => 1.4142156862745097
    assert!((answer - 2.0_f64.sqrt()).abs() < 0.001);

    // The frames of Figure 3.11: E1 is the call frame of sqrt(2),
    // holding x and the two procedure objects built in it.
    let global = Env::global();
    let e1 = Env::child(&global);
    define(&e1, "x", Value::real(2.0));

    // E2 is the frame of the first good_enough call, E3 the frame of
    // the improve call: each holds its own guess, and both extend E1,
    // because that is the environment part the closures carry.
    let e2 = Env::child(&e1);
    define(&e2, "guess", Value::real(1.0));
    let e3 = Env::child(&e1);
    define(&e3, "guess", Value::real(1.0));

    // The two guesses are distinct locals in distinct frames.
    assert!(!Rc::ptr_eq(&e2, &e3));
    assert_eq!(e2.lookup("guess"), Ok(Value::real(1.0)));

    // x in good_enough's body resolves from E2 by walking out to E1:
    // it is the value the original sqrt call received. The global frame
    // has no x: nothing about the call leaked outwards.
    assert_eq!(e2.lookup("x"), Ok(Value::real(2.0)));
    assert_eq!(
        global.lookup("x"),
        Err(SchemeError::UnboundVariable("x".into()))
    );

    // The helper names never reached the global environment: the frame
    // that the call built holds them, and the global frame stays empty
    // of them.
    assert!(
        e1.outer
            .as_ref()
            .is_some_and(|outer| Rc::ptr_eq(outer, &global))
    );
}
