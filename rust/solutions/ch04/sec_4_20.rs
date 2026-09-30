// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.20: `letrec` as unassigned
//! bindings followed by explicit assignment.

/// Shared typed support for this exercise.
pub mod support;

use support::{BindingValue, Env};

fn letrec(
    bindings: &[(&str, BindingValue)],
    body: &str,
) -> Result<BindingValue, sicp_runtime::SicpError> {
    let env = Env::root();
    for (name, _) in bindings {
        env.define(*name, BindingValue::Unassigned);
    }
    for (name, value) in bindings {
        env.assign(name, value.clone())?;
    }
    env.lookup(body)
        .ok_or_else(|| sicp_runtime::SicpError::UnboundVariable(body.to_owned()))
}

#[test]
fn ex_4_20() {
    let even = BindingValue::Symbol("even? calls odd?".to_owned());
    let odd = BindingValue::Symbol("odd? calls even?".to_owned());
    assert_eq!(
        letrec(&[("even?", even.clone()), ("odd?", odd.clone())], "even?"),
        Ok(even)
    );
}
