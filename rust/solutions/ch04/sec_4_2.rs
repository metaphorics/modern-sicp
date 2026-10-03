// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.2: dispatch order and
//! call-prefixed applications over typed syntax data.

/// Shared typed support for this exercise.
pub mod support;

use sicp_runtime::SicpError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Form {
    Definition(i64),
    Assignment(i64),
    Application(i64, i64),
    PrefixedCall(i64, i64),
}

fn application_first(form: Form) -> Result<i64, SicpError> {
    match form {
        Form::Application(left, right) | Form::PrefixedCall(left, right) => Ok(left + right),
        Form::Definition(_) | Form::Assignment(_) => Err(SicpError::TypeMismatch(
            "special form reached the application clause".to_owned(),
        )),
    }
}

fn special_first(form: Form) -> i64 {
    match form {
        Form::Definition(value) | Form::Assignment(value) => value,
        Form::Application(left, right) | Form::PrefixedCall(left, right) => left + right,
    }
}

#[test]
fn ex_4_02() {
    assert!(application_first(Form::Definition(3)).is_err());
    assert_eq!(special_first(Form::Definition(3)), 3);
    assert_eq!(special_first(Form::PrefixedCall(20, 22)), 42);
    assert!(application_first(Form::Assignment(4)).is_err());
    assert_eq!(special_first(Form::Assignment(4)), 4);
    assert_eq!(special_first(Form::Application(20, 22)), 42);
}
