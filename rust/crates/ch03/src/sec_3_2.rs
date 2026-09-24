// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.2

//! Section 3.2: The environment model of evaluation.
//!
//! The re-cut lesson: frames are [`Rc<Env>`](sicp_runtime::Env) nodes, a
//! closure captures a clone of the `Rc`, and `Rc::clone` is the book's
//! "pointer to the environment" made explicit in syntax. `move` closures
//! copy the pointer, not the binding; the examples in
//! `crates/ch03/examples/` walk the book's figures with both real Rust
//! closures and these real frames.

use std::rc::Rc;

use sicp_runtime::{Env, SchemeError, Value};

/// The book's `square`, the first procedure object of the section: a
/// plain `fn` item, which captures nothing, so its environment part is
/// the crate's global scope.
#[must_use]
pub fn square(x: i128) -> i128 {
    x * x
}

/// The book's `sum-of-squares`: two calls to [`square`], each building
/// its own frame with its own `x`.
#[must_use]
pub fn sum_of_squares(x: i128, y: i128) -> i128 {
    square(x) + square(y)
}

/// The book's `f`: applies the subexpressions `a + 1` and `a * 2`
/// before the call, so `f(5)` evaluates to 136.
#[must_use]
pub fn f(a: i128) -> i128 {
    sum_of_squares(a + 1, a * 2)
}

/// The book's `sqrt` with its internal definitions kept internal: the
/// two helper closures see the enclosing frame's `x`, and the loop
/// plays the part of the tail-recursive `sqrt-iter`, which no Rust call
/// is guaranteed to optimize away.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    let good_enough = |guess: f64| (guess * guess - x).abs() < 0.001;
    let improve = |guess: f64| f64::midpoint(guess, x / guess);
    let mut guess = 1.0;
    while !good_enough(guess) {
        guess = improve(guess);
    }
    guess
}

/// The book's `define`: binds (or rebinds) `name` in the frame itself,
/// without walking the chain. A `define` in the body of a call is what
/// fills the call's frame; the model's first frame is this frame.
pub fn define(env: &Rc<Env>, name: &str, value: Value) {
    env.define(Rc::from(name), value);
}

/// The value of a variable with respect to a frame: the binding in the
/// first frame of the chain that has one.
///
/// # Errors
/// Whatever [`Env::lookup`] returns; the model's unbound-variable error
/// is [`SchemeError::UnboundVariable`].
pub fn lookup(env: &Env, name: &str) -> Result<Value, SchemeError> {
    env.lookup(name)
}

/// Reads one integer binding out of a frame, so the examples can do the
/// book's arithmetic on the model's values.
///
/// # Errors
/// [`SchemeError::UnboundVariable`] when no frame on the chain binds
/// `name`; [`SchemeError::TypeMismatch`] when the binding is not an
/// exact integer.
pub fn int_of(env: &Env, name: &str) -> Result<i128, SchemeError> {
    match env.lookup(name)? {
        Value::Int(n) => Ok(n),
        other => Err(SchemeError::TypeMismatch(format!(
            "not an integer: {other}"
        ))),
    }
}

/// The procedure object of the book's Figure 3.7, built from real
/// parts: the code is the generated body of a `move` closure, and the
/// environment part is the [`Env`] frame whose clone the closure
/// captures. Calls read and write `balance` in exactly that frame, so
/// two objects built over two frames share the code and none of the
/// state.
pub fn make_withdraw_procedure(env: Rc<Env>) -> impl FnMut(i128) -> Result<Value, SchemeError> {
    move |amount| {
        let Value::Int(balance) = env.lookup("balance")? else {
            return Err(SchemeError::TypeMismatch(
                "balance is not an integer".to_owned(),
            ));
        };
        if balance >= amount {
            let remaining = balance - amount;
            env.set("balance", Value::int(remaining))?;
            Ok(Value::int(remaining))
        } else {
            Ok(Value::string("Insufficient funds"))
        }
    }
}
