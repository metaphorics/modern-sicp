// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.2.3

//! Section 3.2.3: frames as the repository of local state — the
//! make-withdraw walkthrough of Figures 3.6 to 3.10, with real closures
//! and with the frames built as real `Rc<Env>` nodes.

use std::rc::Rc;

use ch03::sec_3_1::{Reply, make_withdraw};
use ch03::sec_3_2::{define, int_of, make_withdraw_procedure};
use sicp_runtime::{Env, SchemeError, Value};

/// Renders one answer of a procedure object the way the interactions
/// print: the value, or the book's error text.
fn show(answer: &Result<Value, SchemeError>) -> String {
    match answer {
        Ok(value) => value.to_string(),
        Err(err) => format!("Error: {err}"),
    }
}

fn main() {
    // The section's factory: the closure it returns owns the balance
    // the call moved in, so each call yields an independent object.
    let mut w1 = make_withdraw(100);
    let answer = w1(50);
    println!("{answer}");
    // => 50
    assert_eq!(answer, Reply::Balance(50));

    // A second object from the same factory: same code, its own state.
    // Draining w2 leaves w1's balance untouched.
    let mut w2 = make_withdraw(100);
    let answer = w2(25);
    println!("{answer}");
    // => 75
    assert_eq!(answer, Reply::Balance(75));

    let answer = w1(40);
    println!("{answer}");
    // => 10
    assert_eq!(answer, Reply::Balance(10));

    // The procedure object of Figure 3.7, built from real parts: E1 is
    // the call frame of make_withdraw(100), the move closure captures a
    // clone of it, and that frame is the object's environment part.
    let global = Env::global();
    let e1 = Env::child(&global);
    define(&e1, "balance", Value::int(100));
    let mut w1_object = make_withdraw_procedure(Rc::clone(&e1));

    let answer = w1_object(50);
    println!("{}", show(&answer));
    // => 50
    assert_eq!(answer, Ok(Value::int(50)));

    // E2 is the frame of a second call: a second object over the same
    // generated code, holding its own balance.
    let e2 = Env::child(&global);
    define(&e2, "balance", Value::int(100));
    let mut w2_object = make_withdraw_procedure(Rc::clone(&e2));

    let answer = w2_object(70);
    println!("{}", show(&answer));
    // => 30
    assert_eq!(answer, Ok(Value::int(30)));

    // An overdraw answers with the book's message, and the balance is
    // unchanged by the refused call.
    let answer = w2_object(40);
    println!("{}", show(&answer));
    // => "Insufficient funds"
    assert_eq!(answer, Ok(Value::string("Insufficient funds")));
    assert_eq!(int_of(&e2, "balance"), Ok(30));

    // Calls to the first object reference the balance stored in E1 and
    // nothing else: E1 and E2 are distinct frames that merely extend
    // the same global environment, which is the code sharing of the
    // section made visible.
    assert!(!Rc::ptr_eq(&e1, &e2));
    assert_eq!(int_of(&e1, "balance"), Ok(50));
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
}
