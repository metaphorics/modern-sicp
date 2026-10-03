// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The environment chain of the 3.2 re-cut: frames are `Rc<Env>` nodes, a
//! closure captures a clone of the `Rc`, and `Rc::clone` is the book's
//! "pointer to the environment" made explicit in syntax. `move` closures
//! copy the pointer, not the binding.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::SicpError;
use crate::value::Value;

/// One frame plus the frame it extends. Names bind `Value`s; the frame
/// itself is interior-mutable, which is the book's `define` and `set!`.
#[derive(Debug)]
pub struct Env {
    /// This frame's bindings.
    pub frame: RefCell<HashMap<Rc<str>, Value>>,
    /// The frame this one extends, if any.
    pub outer: Option<Rc<Env>>,
}

impl Env {
    /// A fresh frame with no outer frame: the global environment of 4.1.
    #[must_use]
    pub fn global() -> Rc<Env> {
        Env::empty_frame(None)
    }

    /// A fresh frame extending `outer`: the book's
    /// `extend-environment`. The caller holds the returned `Rc`, which is
    /// the procedure's captured environment pointer.
    #[must_use]
    pub fn child(outer: &Rc<Env>) -> Rc<Env> {
        Env::empty_frame(Some(Rc::clone(outer)))
    }

    fn empty_frame(outer: Option<Rc<Env>>) -> Rc<Env> {
        Rc::new(Env {
            frame: RefCell::new(HashMap::new()),
            outer,
        })
    }

    /// The book's `define-variable!`: binds (or rebinds) `name` in this
    /// frame, shadowing any outer binding without touching it.
    pub fn define(&self, name: Rc<str>, value: Value) {
        self.frame.borrow_mut().insert(name, value);
    }

    /// The book's `lookup-variable-value`: walks the chain outwards.
    ///
    /// # Errors
    /// [`SicpError::UnboundVariable`] when no frame on the chain binds
    /// `name`.
    pub fn lookup(&self, name: &str) -> Result<Value, SicpError> {
        let mut cursor = Some(self);
        while let Some(env) = cursor {
            if let Some(v) = env.frame.borrow().get(name) {
                return Ok(v.clone());
            }
            cursor = env.outer.as_deref();
        }
        Err(SicpError::UnboundVariable(name.to_owned()))
    }

    /// The book's `set-variable-value!`: rebinds the nearest existing
    /// binding of `name` on the chain, without creating one.
    ///
    /// # Errors
    /// [`SicpError::UnboundVariable`] when no frame on the chain binds
    /// `name` (the book's "Unbound variable -- SET!").
    pub fn set(&self, name: &str, value: Value) -> Result<(), SicpError> {
        let mut cursor = Some(self);
        while let Some(env) = cursor {
            let mut frame = env.frame.borrow_mut();
            if let Some(slot) = frame.get_mut(name) {
                *slot = value;
                return Ok(());
            }
            cursor = env.outer.as_deref();
        }
        Err(SicpError::UnboundVariable(name.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::Env;
    use crate::error::SicpError;
    use crate::value::Value;

    #[test]
    fn define_and_lookup_in_one_frame() {
        let env = Env::global();
        env.define("x".into(), Value::int(10));
        assert_eq!(env.lookup("x"), Ok(Value::int(10)));
    }

    #[test]
    fn lookup_walks_the_chain_outwards() {
        let global = Env::global();
        global.define("a".into(), Value::int(1));
        let inner = Env::child(&global);
        inner.define("b".into(), Value::int(2));
        assert_eq!(inner.lookup("a"), Ok(Value::int(1)));
        assert_eq!(inner.lookup("b"), Ok(Value::int(2)));
    }

    #[test]
    fn inner_define_shadows_without_touching_outer() {
        let global = Env::global();
        global.define("x".into(), Value::int(1));
        let inner = Env::child(&global);
        inner.define("x".into(), Value::int(99));
        assert_eq!(inner.lookup("x"), Ok(Value::int(99)));
        assert_eq!(global.lookup("x"), Ok(Value::int(1)));
    }

    #[test]
    fn set_mutates_the_nearest_binding_through_every_capture() {
        let global = Env::global();
        global.define("balance".into(), Value::int(100));
        let frame = Env::child(&global);
        frame.set("balance", Value::int(60)).expect("bound above");
        assert_eq!(global.lookup("balance"), Ok(Value::int(60)));
        assert_eq!(frame.lookup("balance"), Ok(Value::int(60)));
    }

    #[test]
    fn set_does_not_create_bindings() {
        let global = Env::global();
        let frame = Env::child(&global);
        let e = frame.set("nope", Value::int(1)).expect_err("unbound");
        assert!(matches!(e, SicpError::UnboundVariable(_)));
        assert!(matches!(
            frame.lookup("nope"),
            Err(SicpError::UnboundVariable(_))
        ));
    }

    #[test]
    fn two_captures_share_one_frame() {
        use std::rc::Rc;
        let shared = Env::global();
        let w1 = Env::child(&shared);
        let w2 = Env::child(&shared);
        // Both procedure frames extend the same outer frame: the book's
        // "W1 and W2 share code but not state" with the state in `shared`.
        shared.define("n".into(), Value::int(5));
        assert!(
            w1.outer
                .as_ref()
                .is_some_and(|outer| Rc::ptr_eq(outer, &shared)),
            "w1 captures the shared frame"
        );
        assert_eq!(w2.lookup("n"), Ok(Value::int(5)));
        assert_eq!(w1.lookup("n"), Ok(Value::int(5)));
    }

    #[test]
    fn unbound_error_names_the_symbol() {
        let env = Env::global();
        let e = env.lookup("ghost").expect_err("unbound");
        assert_eq!(e.to_string(), "unbound variable: ghost");
    }
}
