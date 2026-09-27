// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.11: the frame as an association list. The environment is a
// list of frames again, each frame a list of `(name . value)` bindings,
// and `set!` mutates the binding cell it finds..

use ch04::eval_support::*;

mod ex_4_11 {
    use super::*;

    /// Builds one frame of `(name . value)` bindings, the book's
    /// `make-frame` of exercise 4.11.
    #[must_use]
    pub fn alist_frame(names: &[Value], values: &[Value]) -> Value {
        let mut frame = Value::Nil;
        for (var, val) in names.iter().zip(values) {
            let binding = Value::Pair(cons_cell(var.clone(), val.clone()));
            frame = Value::Pair(cons_cell(binding, frame));
        }
        frame
    }

    /// `extend-environment`: the new frame in front of `base`.
    #[must_use]
    pub fn alist_extend(frame: &Value, base: &Value) -> Value {
        Value::Pair(cons_cell(frame.clone(), base.clone()))
    }

    /// The book's `frame-variables`/`frame-values` scan as one walk: the
    /// binding whose name is `var`, if the frame holds one.
    fn scan_frame(frame: &Value, var: &Value) -> Option<Value> {
        let mut cursor = frame.clone();
        while let Value::Pair(cell) = cursor {
            let binding = cell.car.borrow().clone();
            let binding_name = sicp_runtime::car(&binding).ok()?;
            if binding_name == *var {
                return Some(binding);
            }
            cursor = cell.cdr.borrow().clone();
        }
        None
    }

    /// `lookup-variable-value`: frame by frame, outwards.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] when no frame binds `var`.
    pub fn alist_lookup(env: &Value, var: &Value) -> EvalResult {
        let mut cursor = env.clone();
        while let Value::Pair(cell) = cursor {
            let frame = cell.car.borrow().clone();
            if let Some(binding) = scan_frame(&frame, var) {
                return sicp_runtime::cdr(&binding);
            }
            cursor = cell.cdr.borrow().clone();
        }
        Err(SchemeError::UnboundVariable(var.to_string()))
    }

    /// `define-variable!`: the binding joins the first frame.
    pub fn alist_define(env: &Value, var: &Value, val: Value) {
        let Value::Pair(cell) = env else {
            return;
        };
        let frame = cell.car.borrow().clone();
        let binding = Value::Pair(cons_cell(var.clone(), val));
        sicp_runtime::set_car(cell, Value::Pair(cons_cell(binding, frame)));
    }

    /// `set-variable-value!`: the found binding's value cell mutates in
    /// place, so every holder of the frame sees the new value.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] when no frame binds `var`.
    pub fn alist_set(env: &Value, var: &Value, val: Value) -> Result<(), SchemeError> {
        let mut cursor = env.clone();
        while let Value::Pair(cell) = cursor {
            let frame = cell.car.borrow().clone();
            if let Some(binding) = scan_frame(&frame, var) {
                let Value::Pair(binding_cell) = binding else {
                    return Err(SchemeError::TypeMismatch("not a binding".to_owned()));
                };
                sicp_runtime::set_cdr(&binding_cell, val);
                return Ok(());
            }
            cursor = cell.cdr.borrow().clone();
        }
        Err(SchemeError::UnboundVariable(var.to_string()))
    }

    /// Answers the lookups of a shadowed name, a rebound name, and a
    /// missing name under the association-list representation.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let outer = alist_extend(
            &alist_frame(&[Value::sym("x")], &[Value::int(1)]),
            &Value::Nil,
        );
        let inner = alist_extend(&alist_frame(&[Value::sym("x")], &[Value::int(2)]), &outer);
        let shadowed = alist_lookup(&inner, &Value::sym("x"))?;
        alist_set(&inner, &Value::sym("x"), Value::int(10))?;
        let rebound = alist_lookup(&inner, &Value::sym("x"))?;
        alist_define(&inner, &Value::sym("y"), Value::int(7));
        assert_eq!(alist_lookup(&inner, &Value::sym("y"))?, Value::int(7));
        let missing = alist_lookup(&inner, &Value::sym("z")).expect_err("z is unbound");
        // The outer frame's own binding survived the inner set!.
        let outer_x = alist_lookup(&outer, &Value::sym("x"))?;
        assert_eq!(outer_x, Value::int(1));
        Ok(vec![
            print_value(&shadowed),
            print_value(&rebound),
            missing.to_string(),
        ])
    }
}

#[test]
fn ex_4_11() {
    let values = ex_4_11::answers().expect("runs");
    assert_eq!(values, vec!["2", "10", "unbound variable: z"]);
}
