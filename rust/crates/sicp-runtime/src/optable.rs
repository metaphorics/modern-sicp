// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The book's operation-and-tag dispatch table (2.4.2): `put` installs a
//! handler under an `(operation, tag)` pair, `get` retrieves it, and a
//! later install overwrites an earlier one. Dispatch is data here, not
//! traits, because Rust traits close the type set at compile time while
//! 2.73 through 2.5.3 extend the table at runtime by "installing
//! packages" that may not exist when the caller compiles.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::key::Key;
use crate::value::Handler;

/// The `(operation, tag) -> handler` table behind `put` and `get`.
/// Cloning is not implemented on purpose: one program has one table, and
/// sections share it by `Rc` when they need aliasing.
#[derive(Default)]
pub struct OpTable(RefCell<HashMap<(Key, Key), Handler>>);

impl OpTable {
    /// An empty table.
    #[must_use]
    pub fn new() -> Self {
        Self(RefCell::new(HashMap::new()))
    }

    /// The book's `put`: installs `handler` under the `(op, tag)` pair,
    /// overwriting any earlier install of exactly that pair.
    pub fn put(&self, op: Key, tag: Key, handler: Handler) {
        self.0.borrow_mut().insert((op, tag), handler);
    }

    /// The book's `get`: the handler under `(op, tag)`, or the absent
    /// option on a miss — never a false-ish sentinel.
    #[must_use]
    pub fn get(&self, op: &Key, tag: &Key) -> Option<Handler> {
        self.0.borrow().get(&(op.clone(), tag.clone())).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::OpTable;
    use crate::error::SchemeError;
    use crate::key::Key;
    use crate::value::{Handler, Value};

    /// A handler that returns the fixed value `n`.
    fn constant(n: i128) -> Handler {
        Rc::new(move |_: &[Value]| Ok(Value::int(n)))
    }

    #[test]
    fn put_then_get_returns_the_handler() {
        let table = OpTable::new();
        table.put(Key::sym("real-part"), Key::sym("rectangular"), constant(1));
        let h = table
            .get(&Key::sym("real-part"), &Key::sym("rectangular"))
            .expect("installed");
        assert_eq!(h(&[]), Ok(Value::int(1)));
    }

    #[test]
    fn missing_key_returns_the_absent_option() {
        let table = OpTable::new();
        assert!(table.get(&Key::sym("nope"), &Key::sym("polar")).is_none());
    }

    #[test]
    fn later_install_overwrites_earlier() {
        let table = OpTable::new();
        let op = Key::sym("add");
        let tag = Key::sym("scheme-number");
        table.put(op.clone(), tag.clone(), constant(1));
        table.put(op.clone(), tag.clone(), constant(2));
        let h = table.get(&op, &tag).expect("installed twice");
        assert_eq!(h(&[]), Ok(Value::int(2)), "the second install won");
    }

    #[test]
    fn tags_and_operations_dispatch_independently() {
        let table = OpTable::new();
        table.put(Key::sym("real-part"), Key::sym("rectangular"), constant(1));
        table.put(Key::sym("real-part"), Key::sym("polar"), constant(2));
        table.put(Key::sym("imag-part"), Key::sym("rectangular"), constant(3));
        assert_eq!(
            table
                .get(&Key::sym("real-part"), &Key::sym("polar"))
                .expect("polar install")(&[]),
            Ok(Value::int(2))
        );
        assert!(
            table
                .get(&Key::sym("imag-part"), &Key::sym("polar"))
                .is_none()
        );
    }

    #[test]
    fn handlers_receive_the_argument_slice() {
        let table = OpTable::new();
        table.put(
            Key::sym("add"),
            Key::sym("scheme-number"),
            Rc::new(|args: &[Value]| {
                let a = args.first().cloned().unwrap_or(Value::Nil);
                let b = args.get(1).cloned().unwrap_or(Value::Nil);
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => x
                        .checked_add(y)
                        .map_or_else(|| Err(SchemeError::Overflow), |sum| Ok(Value::int(sum))),
                    _ => Err(SchemeError::TypeMismatch("add: numbers".to_owned())),
                }
            }),
        );
        let h = table
            .get(&Key::sym("add"), &Key::sym("scheme-number"))
            .expect("installed");
        assert_eq!(h(&[Value::int(2), Value::int(3)]), Ok(Value::int(5)));
        assert_eq!(
            h(&[Value::int(7), Value::sym("x")]),
            Err(SchemeError::TypeMismatch("add: numbers".to_owned()))
        );
    }
}
