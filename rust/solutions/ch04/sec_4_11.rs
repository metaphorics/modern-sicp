// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.11: frames represented as
//! association lists of typed bindings.

/// Shared typed support for this exercise.
pub mod support;

use support::BindingValue;

type Binding = (String, BindingValue);

fn lookup(frame: &[Binding], name: &str) -> Option<BindingValue> {
    frame
        .iter()
        .rev()
        .find(|(bound, _)| bound == name)
        .map(|(_, value)| value.clone())
}

fn define(frame: &mut Vec<Binding>, name: &str, value: BindingValue) {
    frame.push((name.to_owned(), value));
}

fn assign(frame: &mut [Binding], name: &str, value: BindingValue) -> bool {
    let Some(binding) = frame.iter_mut().rev().find(|(bound, _)| bound == name) else {
        return false;
    };
    binding.1 = value;
    true
}

#[test]
fn ex_4_11() {
    let mut frame = Vec::new();
    define(&mut frame, "x", BindingValue::Int(1));
    define(&mut frame, "y", BindingValue::Int(2));
    define(&mut frame, "x", BindingValue::Int(10));
    assert_eq!(lookup(&frame, "x"), Some(BindingValue::Int(10)));
    assert!(assign(&mut frame, "y", BindingValue::Int(20)));
    assert_eq!(lookup(&frame, "y"), Some(BindingValue::Int(20)));
    assert_eq!(lookup(&frame, "z"), None);
}
