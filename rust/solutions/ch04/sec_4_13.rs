// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.13: unbinding removes a name
//! only from the frame that owns it.

/// Shared typed support for this exercise.
pub mod support;

use support::{BindingValue, Env};

#[test]
fn ex_4_13() {
    let outer = Env::root();
    outer.define("x", BindingValue::Int(1));
    let inner = outer.child();
    inner.define("x", BindingValue::Int(2));
    inner.unbind("x").expect("inner x is in the current frame");
    assert_eq!(inner.lookup("x"), Some(BindingValue::Int(1)));
    assert!(outer.lookup("y").is_none());
    assert!(inner.unbind("y").is_err());
}
