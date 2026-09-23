// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The hashable projection of `Value` behind dynamic tables: the keys the
//! book's `put` and `get` compare with `equal?`. `Value` holds `f64` and
//! closures and cannot derive `Eq`/`Hash`; `Key` is the hashable shape
//! tables key on, per the operation-table sketch of the edition plan.

use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use crate::error::SchemeError;
use crate::value::Value;

/// A table key: the shapes `equal?` compares in 2.4's dispatch table,
/// 3.3.3's pairs-based table, and `memo-fib`'s memoization.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// A symbol key: `rectangular` in the table of 2.4.3.
    Sym(Rc<str>),
    /// An exact-integer key: `memo-fib`'s argument in 3.3.3.
    Int(i128),
    /// A pair key: a list the pairs-based table compares by `equal?`.
    Pair(Box<Key>, Box<Key>),
    /// A string key.
    Str(Rc<str>),
    /// The empty-list terminator: proper lists are keys, so their `Nil`
    /// tail must be one too.
    Nil,
}

impl Key {
    /// Builds a symbol key.
    #[must_use]
    pub fn sym(s: &str) -> Self {
        Key::Sym(Rc::from(s))
    }

    /// Builds an exact-integer key.
    #[must_use]
    pub fn int(n: i128) -> Self {
        Key::Int(n)
    }

    /// Builds a string key.
    #[must_use]
    pub fn string(s: &str) -> Self {
        Key::Str(Rc::from(s))
    }

    /// Builds a pair key.
    #[must_use]
    pub fn pair(a: Key, b: Key) -> Self {
        Key::Pair(Box::new(a), Box::new(b))
    }
}

impl TryFrom<&Value> for Key {
    type Error = SchemeError;

    /// Projects a `Value` onto its hashable key shape.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when the value is not a symbol,
    /// integer, string, or (recursively) a pair of keyable values: reals,
    /// booleans, `Nil`, and procedure objects are not keys.
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(n) => Ok(Key::Int(*n)),
            Value::Sym(s) => Ok(Key::Sym(Rc::clone(s))),
            Value::Str(s) => Ok(Key::Str(Rc::clone(s))),
            Value::Nil => Ok(Key::Nil),
            Value::Pair(cell) => {
                let (car, cdr) = {
                    let car = cell.car.borrow().clone();
                    let cdr = cell.cdr.borrow().clone();
                    (car, cdr)
                };
                Ok(Key::Pair(
                    Box::new(Key::try_from(&car)?),
                    Box::new(Key::try_from(&cdr)?),
                ))
            }
            other => Err(SchemeError::TypeMismatch(format!(
                "not a table key: {other}"
            ))),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Key::Sym(s) => f.write_str(s),
            Key::Int(n) => write!(f, "{n}"),
            Key::Str(s) => write!(f, "{s:?}"),
            Key::Pair(a, b) => write!(f, "({a} . {b})"),
            Key::Nil => f.write_str("()"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Key;
    use crate::error::SchemeError;
    use crate::value::Value;

    #[test]
    fn round_trips_data_values() {
        assert_eq!(Key::try_from(&Value::int(42)), Ok(Key::int(42)));
        assert_eq!(Key::try_from(&Value::sym("x")), Ok(Key::sym("x")));
        assert_eq!(Key::try_from(&Value::string("s")), Ok(Key::string("s")));
    }

    #[test]
    fn pair_keys_recurse_into_cells() {
        let v = Value::list(vec![Value::sym("a"), Value::int(1), Value::sym("b")]);
        let k = Key::try_from(&v).expect("list of keyables");
        // The pair projection is the right-nested cons spine, ended by
        // the Nil key.
        assert_eq!(
            k,
            Key::pair(
                Key::sym("a"),
                Key::pair(Key::int(1), Key::pair(Key::sym("b"), Key::Nil))
            )
        );
    }

    #[test]
    fn non_keyable_values_are_rejected() {
        assert!(matches!(
            Key::try_from(&Value::real(1.5)),
            Err(SchemeError::TypeMismatch(_))
        ));
        assert!(matches!(
            Key::try_from(&Value::Bool(true)),
            Err(SchemeError::TypeMismatch(_))
        ));
    }

    #[test]
    fn equal_keys_hash_to_one_slot() {
        let mut map = HashMap::new();
        map.insert(Key::sym("add"), 1);
        // A freshly allocated, content-equal key must hit the same slot.
        assert_eq!(map.get(&Key::sym("add")), Some(&1));
        assert_eq!(map.get(&Key::sym("sub")), None);
    }

    #[test]
    fn display_is_scheme_syntax() {
        assert_eq!(Key::sym("rectangular").to_string(), "rectangular");
        assert_eq!(Key::int(-7).to_string(), "-7");
        assert_eq!(Key::pair(Key::sym("a"), Key::int(2)).to_string(), "(a . 2)");
    }
}
