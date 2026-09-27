// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The edition's one typed error: every `error` in the book raises one of
//! these variants, and every checked operation returns it.

use std::fmt::Write as _;

use crate::value::Value;

/// The edition's typed error: every `error` in the book raises one of these
/// variants.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum SchemeError {
    /// A name lookup walked the whole environment chain without a hit.
    #[error("unbound variable: {0}")]
    UnboundVariable(String),

    /// The operator position of an application did not evaluate to a
    /// procedure.
    #[error("not a procedure: {0}")]
    NotProcedure(Value),

    /// A procedure received the wrong number of arguments; `expected` is
    /// the book's phrasing, `"2"` for a fixed list, `"at least 1"` when a
    /// rest parameter is present.
    #[error("{procedure}: wrong number of arguments, expected {expected}, got {got}")]
    WrongArity {
        /// The procedure that rejected the call.
        procedure: String,
        /// The expected count in the book's phrasing.
        expected: String,
        /// The count actually passed.
        got: usize,
    },

    /// A value reached an operation that only accepts another shape.
    #[error("type mismatch: {0}")]
    TypeMismatch(String),

    /// Division or modulo by zero.
    #[error("division by zero")]
    DivisionByZero,

    /// Checked `i128` arithmetic ran past the fixed width (the number
    /// convention of the edition: the overflow error is the lesson).
    #[error("integer overflow")]
    Overflow,

    /// A reader rejected malformed surface syntax.
    #[error("parse error: {0}")]
    Parse(String),

    /// The `amb` engine has no alternative left: `try-again` past the last
    /// solution unwinds to this (4.3 control flow, not a failure).
    #[error("no more alternatives")]
    Backtrack,

    /// The book's `(error message irritant ...)` reached top level.
    #[error("{message}{irritants}", irritants = irritants_suffix(.irritants))]
    UserRaised {
        /// The message argument of `error`.
        message: String,
        /// The irritant objects, printed after the message.
        irritants: Vec<Value>,
    },

    /// A message-passing object received a message it does not answer
    /// (2.1.3's procedural cons, 3.1.1's account).
    #[error("message not understood: {0}")]
    UnknownMessage(u8),

    /// `make-account` refused a withdrawal larger than the balance.
    #[error("insufficient funds")]
    InsufficientFunds,

    /// `Random::new` rejects a zero seed because `xorshift64*` never leaves
    /// the zero state.
    #[error("random: the seed must be nonzero")]
    ZeroSeed,
}

/// Formats the irritant list of `error` as a leading space plus each
/// object, so `(error "ap" 1 2)` displays as `ap 1 2`.
fn irritants_suffix(irritants: &[Value]) -> String {
    // `String`'s fmt::Write never fails, so the ignored result is safe.
    irritants.iter().fold(String::new(), |mut acc, v| {
        let _ = write!(acc, " {v}");
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::SchemeError;
    use crate::value::Value;

    #[test]
    fn unbound_names_the_variable() {
        let e = SchemeError::UnboundVariable("x".to_owned());
        assert_eq!(e.to_string(), "unbound variable: x");
    }

    #[test]
    fn user_raised_prints_message_then_irritants() {
        let e = SchemeError::UserRaised {
            message: "ap".to_owned(),
            irritants: vec![Value::Int(1), Value::Int(2)],
        };
        assert_eq!(e.to_string(), "ap 1 2");
    }

    #[test]
    fn user_raised_with_no_irritants_prints_bare_message() {
        let e = SchemeError::UserRaised {
            message: "boom".to_owned(),
            irritants: vec![],
        };
        assert_eq!(e.to_string(), "boom");
    }

    #[test]
    fn control_flow_and_domain_errors_print_fixed_text() {
        assert_eq!(SchemeError::Backtrack.to_string(), "no more alternatives");
        assert_eq!(SchemeError::DivisionByZero.to_string(), "division by zero");
        assert_eq!(SchemeError::Overflow.to_string(), "integer overflow");
        assert_eq!(
            SchemeError::InsufficientFunds.to_string(),
            "insufficient funds"
        );
        assert_eq!(
            SchemeError::UnknownMessage(3).to_string(),
            "message not understood: 3"
        );
    }

    #[test]
    fn not_procedure_prints_the_offending_object() {
        let e = SchemeError::NotProcedure(Value::Int(5));
        assert_eq!(e.to_string(), "not a procedure: 5");
    }

    #[test]
    fn wrong_arity_names_procedure_and_counts() {
        let e = SchemeError::WrongArity {
            procedure: "square".to_owned(),
            expected: "1".to_owned(),
            got: 2,
        };
        assert_eq!(
            e.to_string(),
            "square: wrong number of arguments, expected 1, got 2"
        );
    }

    #[test]
    fn errors_clone_and_compare_by_value() {
        let e = SchemeError::UnknownMessage(7);
        let f = e.clone();
        assert_eq!(e, f);
        let g = SchemeError::NotProcedure(Value::Pair(crate::cons_cell(Value::Nil, Value::Nil)));
        assert_ne!(e, g);
    }
}
