// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The mutable pair of 3.3.1 under `Rc`, plus the `car`/`cdr` selectors
//! that operate on `Value` and never on bare cells. Sharing is visible in
//! the types: `Rc::ptr_eq` is the book's `eq?` on pairs.

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::SchemeError;
use crate::value::Value;

/// The mutable pair of 3.3.1: both fields under `RefCell`, so
/// `set-car!`/`set-cdr!` mutate through shared `Rc` handles and aliasing
/// questions become pointer questions.
#[derive(Clone, Debug, PartialEq)]
pub struct ConsCell {
    /// The car slot.
    pub car: RefCell<Value>,
    /// The cdr slot.
    pub cdr: RefCell<Value>,
}

/// A cons cell under `Rc`: the book's pair object.
pub type Pair = Rc<ConsCell>;

/// Builds the book's `(cons x y)`.
#[must_use]
pub fn cons_cell(car: Value, cdr: Value) -> Pair {
    Rc::new(ConsCell {
        car: RefCell::new(car),
        cdr: RefCell::new(cdr),
    })
}

/// The book's `set-car!`: mutates the shared cell.
pub fn set_car(p: &Pair, v: Value) {
    *p.car.borrow_mut() = v;
}

/// The book's `set-cdr!`: mutates the shared cell.
pub fn set_cdr(p: &Pair, v: Value) {
    *p.cdr.borrow_mut() = v;
}

/// The book's `eq?` on pairs: pointer identity of the shared cell.
#[must_use]
pub fn eq_pair(a: &Pair, b: &Pair) -> bool {
    Rc::ptr_eq(a, b)
}

/// The book's `car` over a dynamic value.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `v` is not a pair.
pub fn car(v: &Value) -> Result<Value, SchemeError> {
    match v {
        Value::Pair(cell) => Ok(cell.car.borrow().clone()),
        other => Err(SchemeError::TypeMismatch(format!(
            "car of a non-pair: {other}"
        ))),
    }
}

/// The book's `cdr` over a dynamic value.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `v` is not a pair.
pub fn cdr(v: &Value) -> Result<Value, SchemeError> {
    match v {
        Value::Pair(cell) => Ok(cell.cdr.borrow().clone()),
        other => Err(SchemeError::TypeMismatch(format!(
            "cdr of a non-pair: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::{car, cdr, cons_cell, eq_pair, set_car, set_cdr};
    use crate::error::SchemeError;
    use crate::value::Value;

    #[test]
    fn cons_builds_and_selectors_read() {
        let p = cons_cell(Value::int(1), Value::list(vec![Value::int(2)]));
        assert_eq!(car(&Value::Pair(p.clone())), Ok(Value::int(1)));
        assert_eq!(
            cdr(&Value::Pair(p.clone())),
            Ok(Value::list(vec![Value::int(2)]))
        );
    }

    #[test]
    fn mutation_is_visible_through_every_alias() {
        let p = cons_cell(Value::int(1), Value::int(2));
        let alias = p.clone();
        set_car(&p, Value::sym("new"));
        set_cdr(&p, Value::int(9));
        assert_eq!(alias.car.borrow().clone(), Value::sym("new"));
        assert_eq!(alias.cdr.borrow().clone(), Value::int(9));
    }

    #[test]
    fn eq_is_pointer_identity_not_structure() {
        let a = cons_cell(Value::int(1), Value::int(2));
        let b = cons_cell(Value::int(1), Value::int(2));
        assert!(!eq_pair(&a, &b));
        assert!(eq_pair(&a, &a.clone()));
    }

    #[test]
    fn selectors_reject_non_pairs() {
        assert!(matches!(
            car(&Value::int(3)),
            Err(SchemeError::TypeMismatch(_))
        ));
        assert!(matches!(
            cdr(&Value::sym("x")),
            Err(SchemeError::TypeMismatch(_))
        ));
    }

    #[test]
    fn cells_compare_deeply() {
        let a = cons_cell(Value::int(1), Value::int(2));
        let b = cons_cell(Value::int(1), Value::int(2));
        assert_eq!(Value::Pair(a), Value::Pair(b));
    }
}
