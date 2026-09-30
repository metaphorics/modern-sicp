// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.12: environment operations
//! abstracted over one frame-level scan.

/// Shared typed support for this exercise.
pub mod support;

use support::BindingValue;

type Binding = (String, BindingValue);

fn frame_scan(frame: &[Binding], name: &str) -> Option<usize> {
    frame.iter().rposition(|(bound, _)| bound == name)
}

fn lookup(env: &[Vec<Binding>], name: &str) -> Option<BindingValue> {
    // Innermost first: a shadowing binding in an inner frame wins.
    env.iter()
        .rev()
        .find_map(|frame| frame_scan(frame, name).map(|index| frame[index].1.clone()))
}

fn define(env: &mut [Vec<Binding>], name: &str, value: BindingValue) {
    env[0].push((name.to_owned(), value));
}

fn assign(env: &mut [Vec<Binding>], name: &str, value: BindingValue) -> bool {
    let Some(frame) = env
        .iter_mut()
        .rev()
        .find(|frame| frame_scan(frame, name).is_some())
    else {
        return false;
    };
    let Some(index) = frame_scan(frame, name) else {
        return false;
    };
    frame[index].1 = value;
    true
}

#[test]
fn ex_4_12() {
    let mut env = vec![Vec::new(), Vec::new()];
    define(&mut env, "x", BindingValue::Int(1));
    env[1].push(("y".to_owned(), BindingValue::Int(2)));
    env[1].push(("x".to_owned(), BindingValue::Int(10)));
    assert_eq!(lookup(&env, "x"), Some(BindingValue::Int(10)));
    assert!(assign(&mut env, "y", BindingValue::Int(20)));
    assert_eq!(lookup(&env, "y"), Some(BindingValue::Int(20)));
    assert_eq!(lookup(&env, "z"), None);
}
