// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.3: data-directed dispatch in
//! eval, using an operation table keyed by the expression head.

/// Shared typed support for this exercise.
pub mod support;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Square,
    Increment,
    Double,
    UnlessFalse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Int(i64),
    Bool(bool),
}

struct Dispatch {
    handlers: HashMap<&'static str, Operation>,
}

impl Default for Dispatch {
    fn default() -> Self {
        let handlers = HashMap::from([
            ("square", Operation::Square),
            ("increment", Operation::Increment),
        ]);
        Self { handlers }
    }
}

impl Dispatch {
    fn install(&mut self, name: &'static str, operation: Operation) {
        self.handlers.insert(name, operation);
    }

    fn eval(&self, name: &str, value: Value) -> Option<Value> {
        match self.handlers.get(name)? {
            Operation::Square => integer(value).map(|number| Value::Int(number * number)),
            Operation::Increment => integer(value).map(|number| Value::Int(number + 1)),
            Operation::Double => integer(value).map(|number| Value::Int(number * 2)),
            Operation::UnlessFalse => Some(Value::Bool(value == Value::Bool(false))),
        }
    }
}

fn integer(value: Value) -> Option<i64> {
    match value {
        Value::Int(value) => Some(value),
        Value::Bool(_) => None,
    }
}

#[test]
fn ex_4_03() {
    let mut dispatch = Dispatch::default();
    assert_eq!(dispatch.eval("square", Value::Int(7)), Some(Value::Int(49)));
    assert_eq!(dispatch.eval("missing", Value::Int(7)), None);
    dispatch.install("double", Operation::Double);
    assert_eq!(dispatch.eval("double", Value::Int(7)), Some(Value::Int(14)));
    dispatch.install("unless-false", Operation::UnlessFalse);
    assert_eq!(
        dispatch.eval("unless-false", Value::Bool(false)),
        Some(Value::Bool(true))
    );
    assert_eq!(
        dispatch.eval("unless-false", Value::Bool(true)),
        Some(Value::Bool(false))
    );
}
