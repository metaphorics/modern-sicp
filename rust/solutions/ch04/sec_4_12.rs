// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.12: the traversals of 4.11's
//! representation abstracted, and the three operations redefined
//! through them.

use ch04::eval_support::*;

/// Builds one frame of `(name . value)` bindings, the 4.11 shape.
fn alist_frame(names: &[Value], values: &[Value]) -> Value {
    let mut frame = Value::Nil;
    for (name, value) in names.iter().zip(values) {
        let binding = Value::Pair(cons_cell(name.clone(), value.clone()));
        frame = Value::Pair(cons_cell(binding, frame));
    }
    frame
}

mod ex_4_12 {
    use super::*;

    /// The frame-level traversal: the binding of `var` in one frame.
    pub fn frame_scan(frame: &Value, var: &Value) -> Option<Value> {
        let mut cursor = frame.clone();
        while let Value::Pair(cell) = cursor {
            let binding = cell.car.borrow().clone();
            if sicp_runtime::car(&binding).ok().as_ref() == Some(var) {
                return Some(binding);
            }
            cursor = cell.cdr.borrow().clone();
        }
        None
    }

    /// The environment-level traversal: the binding of `var` in the
    /// first frame that has one, walking outwards.
    pub fn env_scan(env: &Value, var: &Value) -> Option<Value> {
        let mut cursor = env.clone();
        while let Value::Pair(cell) = cursor {
            let frame = cell.car.borrow().clone();
            if let Some(binding) = frame_scan(&frame, var) {
                return Some(binding);
            }
            cursor = cell.cdr.borrow().clone();
        }
        None
    }

    /// `lookup-variable-value` redefined through `env_scan`.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] when the scan finds nothing.
    pub fn lookup(env: &Value, var: &Value) -> EvalResult {
        let binding =
            env_scan(env, var).ok_or_else(|| SchemeError::UnboundVariable(var.to_string()))?;
        sicp_runtime::cdr(&binding)
    }

    /// `set-variable-value!` redefined through `env_scan`.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] when the scan finds nothing.
    pub fn set(env: &Value, var: &Value, val: Value) -> Result<(), SchemeError> {
        let binding =
            env_scan(env, var).ok_or_else(|| SchemeError::UnboundVariable(var.to_string()))?;
        let Value::Pair(binding_cell) = binding else {
            return Err(SchemeError::TypeMismatch("not a binding".to_owned()));
        };
        sicp_runtime::set_cdr(&binding_cell, val);
        Ok(())
    }

    /// `define-variable!` redefined through `frame_scan`: a hit mutates
    /// the binding, a miss adds one to the first frame.
    pub fn define(env: &Value, var: &Value, val: Value) {
        let Value::Pair(cell) = env else {
            return;
        };
        let frame = cell.car.borrow().clone();
        if let Some(binding) = frame_scan(&frame, var) {
            if let Value::Pair(binding_cell) = binding {
                sicp_runtime::set_cdr(&binding_cell, val);
            }
            return;
        }
        let binding = Value::Pair(cons_cell(var.clone(), val));
        sicp_runtime::set_car(cell, Value::Pair(cons_cell(binding, frame)));
    }

    /// Answers the same three lookups as exercise 4.11, now through the
    /// two abstractions.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let outer = Value::Pair(cons_cell(
            alist_frame(&[Value::sym("x")], &[Value::int(1)]),
            Value::Nil,
        ));
        let inner = Value::Pair(cons_cell(
            alist_frame(&[Value::sym("x")], &[Value::int(2)]),
            outer.clone(),
        ));
        let shadowed = lookup(&inner, &Value::sym("x"))?;
        set(&inner, &Value::sym("x"), Value::int(10))?;
        let rebound = lookup(&inner, &Value::sym("x"))?;
        define(&inner, &Value::sym("y"), Value::int(7));
        let added = lookup(&inner, &Value::sym("y"))?;
        let missing = lookup(&inner, &Value::sym("z")).expect_err("z is unbound");
        assert_eq!(added, Value::int(7));
        Ok(vec![
            print_value(&shadowed),
            print_value(&rebound),
            missing.to_string(),
        ])
    }
}

#[test]
fn ex_4_12() {
    let values = ex_4_12::answers().expect("runs");
    assert_eq!(values, vec!["2", "10", "unbound variable: z"]);
}
