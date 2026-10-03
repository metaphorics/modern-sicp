// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.2.1

//! Section 3.2.1: the rules for evaluation — the two spellings of a
//! procedure object, and the lookup paths through the frames of
//! Figure 3.1.

use std::rc::Rc;

use ch03::sec_3_2::{define, square};
use sicp_runtime::{Env, SicpError, Value};

/// Reads one binding for display: the value of a variable is the
/// binding in the first frame of the chain that has one.
fn value_of(env: &Rc<Env>, name: &str) -> Value {
    match env.lookup(name) {
        Ok(value) => value,
        Err(err) => Value::string(&err.to_string()),
    }
}

fn main() {
    // The `fn` item of the section: a procedure object whose code takes
    // one parameter and whose environment part is the crate's global
    // scope, because a plain fn captures nothing.
    let answer = square(5);
    println!("{answer}");
    // => 25
    assert_eq!(answer, 25);

    // The closure form: a function value bound to a name in the frame
    // that runs the binding. Same code, same answer.
    let square = |x: i128| x * x;
    let answer = square(7);
    println!("{answer}");
    // => 49
    assert_eq!(answer, 49);

    // Figure 3.1, with real pointers: frame I is the global frame, and
    // frames II and III both extend it. The names A, B, C, and D of the
    // figure are names for frames and become `Rc` handles here.
    let frame_i = Env::global();
    define(&frame_i, "x", Value::int(3));
    define(&frame_i, "y", Value::int(5));
    let frame_ii = Env::child(&frame_i);
    define(&frame_ii, "z", Value::int(6));
    define(&frame_ii, "x", Value::int(7));
    let frame_iii = Env::child(&frame_i);
    define(&frame_iii, "m", Value::int(1));
    define(&frame_iii, "y", Value::int(2));

    // Environment C and environment D of the figure point at frames III
    // and I; pointer identity says which frames the names share.
    let environment_c = Rc::clone(&frame_iii);
    assert!(Rc::ptr_eq(&environment_c, &frame_iii));
    assert!(!Rc::ptr_eq(&frame_ii, &frame_iii));

    // The value of x with respect to environment D (frame I) is 3.
    let answer = value_of(&frame_i, "x");
    println!("{answer}");
    // => 3
    assert_eq!(answer, Value::int(3));

    // With respect to environment B (frame III): frame III has no x, so
    // the lookup walks the enclosing-environment pointer out to frame I.
    let answer = value_of(&frame_iii, "x");
    println!("{answer}");
    // => 3
    assert_eq!(answer, Value::int(3));

    // With respect to environment A (frame II): frame II binds x to 7,
    // which shadows the binding of x to 3 in frame I.
    let answer = value_of(&frame_ii, "x");
    println!("{answer}");
    // => 7
    assert_eq!(answer, Value::int(7));

    // y resolves the other way round: frame III's 2 shadows frame I's 5,
    // and frame II has no y of its own.
    let answer = value_of(&frame_ii, "y");
    println!("{answer}");
    // => 5
    assert_eq!(answer, Value::int(5));

    // No frame on the chain binds w: the variable is unbound there, and
    // the model reports it as the edition's typed error.
    assert_eq!(
        frame_ii.lookup("w"),
        Err(SicpError::UnboundVariable("w".into()))
    );
    let answer = value_of(&frame_ii, "w");
    println!("{answer}");
    // => "unbound variable: w"
    assert_eq!(answer, Value::string("unbound variable: w"));

    // A single frame holds at most one binding per variable: define in
    // the same frame changes the binding instead of adding a second.
    define(&frame_ii, "z", Value::int(60));
    let answer = value_of(&frame_ii, "z");
    println!("{answer}");
    // => 60
    assert_eq!(answer, Value::int(60));
}
