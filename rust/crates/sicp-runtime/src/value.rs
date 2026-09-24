// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The dynamic datum of chapters 2 through 4: the `Value` enum everything
//! dynamic flows through, the compound procedure of 3.2 and 4.1, the
//! memoized thunk state of 4.2, the compiled-procedure value of 5.5.7, and
//! the `Symbol` binding name.

use std::cell::RefCell;
use std::fmt::{self, Display, Formatter, Write as _};
use std::rc::Rc;

use crate::env::Env;
use crate::error::SchemeError;
use crate::pair::{ConsCell, cons_cell};

/// A binding name: the book's symbols, quoted nowhere because quotation is
/// a constructor. `Rc<str>` makes `eq?` a content compare and sharing a
/// pointer bump.
pub type Symbol = Rc<str>;

/// The procedure shape the table of 2.4 and the 4.1 primitive list
/// install: `Rc<dyn Fn>` over the argument slice.
pub type Handler = Rc<dyn Fn(&[Value]) -> Result<Value, SchemeError>>;

/// The dynamic datum: everything chapters 2 through 4 pass around. `Value`
/// cannot derive `Eq` or `Hash` (it holds `f64` and closures), so dynamic
/// tables key on [`Key`](crate::Key) instead; `PartialEq` compares data
/// structurally and procedure objects by pointer identity.
#[derive(Clone)]
#[allow(
    clippy::exhaustive_enums,
    reason = "the book's datum is closed: new shapes enter through explicit sections"
)]
pub enum Value {
    /// An exact integer; checked arithmetic past the `i128` width raises
    /// [`SchemeError::Overflow`].
    Int(i128),
    /// An inexact real.
    Real(f64),
    /// The book's `#t` and `#f`.
    Bool(bool),
    /// A symbol.
    Sym(Symbol),
    /// A double-quoted string.
    Str(Rc<str>),
    /// The empty list.
    Nil,
    /// A cons cell under `Rc`.
    Pair(Rc<ConsCell>),
    /// A tag datum plus its payload (2.4.2): the dispatch tag the operation
    /// table looks up.
    Tagged {
        /// The tag, e.g. `rectangular`.
        tag: Rc<str>,
        /// The tagged payload.
        data: Box<Value>,
    },
    /// A primitive procedure installed under a name (4.1).
    Primitive {
        /// The printed name, e.g. `+`.
        name: Rc<str>,
        /// The body.
        f: Handler,
    },
    /// A compound procedure: parameters, optional rest parameter, body
    /// forms, and the captured environment (the 3.2 procedure object).
    Closure(Rc<Closure>),
    /// A memoized thunk of the lazy evaluator (4.2).
    Thunk(Rc<RefCell<ThunkState>>),
    /// A compiled procedure (5.5.7).
    CompiledProc(Rc<CompiledProc>),
}

/// A compound procedure: the parameter names, the optional `.`-style rest
/// parameter, the body forms evaluated in order, and the environment the
/// `lambda` captured.
#[derive(Clone, Debug)]
pub struct Closure {
    /// The required parameter names, in order.
    pub params: Vec<Symbol>,
    /// The rest parameter after `.`, if the form has one.
    pub rest: Option<Symbol>,
    /// The body forms; the last one's value is the answer.
    pub body: Vec<Value>,
    /// The environment captured at `lambda` time.
    pub env: Rc<Env>,
}

/// The memoized thunk state of 4.2: a delayed expression and its
/// environment, or the value that forcing produced.
#[derive(Clone, Debug)]
pub enum ThunkState {
    /// Delayed, not yet forced.
    Delayed {
        /// The expression to evaluate when forced.
        expr: Value,
        /// The environment to evaluate it in.
        env: Rc<Env>,
    },
    /// The memoized result: forcing ran once, and its side effects ran
    /// once, which is what exercise 4.27 probes.
    Forced(Value),
}

impl ThunkState {
    /// Builds the book's `(delay expr)` under `env` as a `Value::Thunk`.
    #[must_use]
    pub fn delay(expr: Value, env: &Rc<Env>) -> Value {
        Value::Thunk(Rc::new(RefCell::new(ThunkState::Delayed {
            expr,
            env: Rc::clone(env),
        })))
    }

    /// Forces a `Value::Thunk`: on the first call runs `eval` over the
    /// delayed expression and memoizes the result; afterwards returns the
    /// memoized value. `eval` runs outside the cell borrow, so a thunk may
    /// legally touch other thunks while forcing; the state is memoized only
    /// when `eval` succeeds, so a failed force leaves the thunk delayed.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when `thunk` is not a `Value::Thunk`;
    /// whatever `eval` returns on the first force.
    pub fn force(
        thunk: &Value,
        eval: impl FnOnce(&Value, &Rc<Env>) -> Result<Value, SchemeError>,
    ) -> Result<Value, SchemeError> {
        let cell = match thunk {
            Value::Thunk(cell) => Rc::clone(cell),
            other => {
                return Err(SchemeError::TypeMismatch(format!(
                    "force on a non-thunk: {other}"
                )));
            }
        };
        let pending = match &*cell.borrow() {
            ThunkState::Forced(v) => return Ok(v.clone()),
            ThunkState::Delayed { expr, env } => (expr.clone(), Rc::clone(env)),
        };
        let (expr, env) = pending;
        let value = eval(&expr, &env)?;
        *cell.borrow_mut() = ThunkState::Forced(value.clone());
        Ok(value)
    }
}

/// The compiled-procedure value of 5.5.7: an entry label resolved against
/// the assembled machine's label table, the parameter names, and the
/// environment installed as the frame chain. The chapter 5 compiler keeps
/// its richer compile-time instruction sequences in its own crate; this is
/// the runtime shape the machine's `compiled-apply` entry consumes.
#[derive(Clone, Debug)]
pub struct CompiledProc {
    /// The entry label name in the machine's label table.
    pub entry: Symbol,
    /// The parameter names, bound to the argument values in order.
    pub params: Vec<Symbol>,
    /// The environment the closure's frame extends.
    pub env: Rc<Env>,
}

impl Value {
    /// Builds an exact integer.
    #[must_use]
    pub fn int(n: i128) -> Self {
        Value::Int(n)
    }

    /// Builds an inexact real.
    #[must_use]
    pub fn real(x: f64) -> Self {
        Value::Real(x)
    }

    /// Builds the book's `#t` or `#f`.
    #[must_use]
    pub fn boolean(b: bool) -> Self {
        Value::Bool(b)
    }

    /// Builds a symbol from `name`.
    #[must_use]
    pub fn sym(name: &str) -> Self {
        Value::Sym(Rc::from(name))
    }

    /// Builds a string.
    #[must_use]
    pub fn string(s: &str) -> Self {
        Value::Str(Rc::from(s))
    }

    /// Builds a tagged datum: the book's `(make-from-real-imag 3 4)` under
    /// the `rectangular` tag and friends.
    #[must_use]
    pub fn tagged(tag: &str, data: Value) -> Self {
        Value::Tagged {
            tag: Rc::from(tag),
            data: Box::new(data),
        }
    }

    /// Builds a proper list out of `items`, `Nil`-terminated.
    #[must_use]
    pub fn list(mut items: Vec<Value>) -> Self {
        let mut out = Value::Nil;
        while let Some(v) = items.pop() {
            out = Value::Pair(cons_cell(v, out));
        }
        out
    }

    /// Returns true when `self` is the empty list.
    #[must_use]
    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    /// Returns true when `self` is a cons cell.
    #[must_use]
    pub fn is_pair(&self) -> bool {
        matches!(self, Value::Pair(_))
    }

    /// Returns the items of a proper list, in order.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when a tail is neither a pair nor
    /// `Nil`: the list is dotted.
    pub fn list_items(&self) -> Result<Vec<Value>, SchemeError> {
        let mut items = Vec::new();
        let mut cursor = self.clone();
        while let Value::Pair(cell) = cursor {
            items.push(cell.car.borrow().clone());
            cursor = cell.cdr.borrow().clone();
        }
        if cursor.is_nil() {
            Ok(items)
        } else {
            Err(SchemeError::TypeMismatch(format!(
                "not a proper list: {self}"
            )))
        }
    }

    /// Applies a primitive procedure to `args`.
    ///
    /// # Errors
    /// [`SchemeError::NotProcedure`] unless `self` is a primitive; whatever
    /// the primitive's body returns.
    pub fn call(&self, args: &[Value]) -> Result<Value, SchemeError> {
        match self {
            Value::Primitive { f, .. } => f(args),
            other => Err(SchemeError::NotProcedure(other.clone())),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Real(a), Value::Real(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            #[expect(
                clippy::match_same_arms,
                reason = "symbol/string content equality and primitive name equality are different judgements that happen to share the shape"
            )]
            (Value::Sym(a), Value::Sym(b)) | (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Pair(a), Value::Pair(b)) => a == b,
            (Value::Tagged { tag: a, data: b }, Value::Tagged { tag: c, data: d }) => {
                a == c && b == d
            }
            (Value::Primitive { name: a, .. }, Value::Primitive { name: b, .. }) => a == b,
            (Value::Closure(a), Value::Closure(b)) => Rc::ptr_eq(a, b),
            (Value::Thunk(a), Value::Thunk(b)) => Rc::ptr_eq(a, b),
            (Value::CompiledProc(a), Value::CompiledProc(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Real(x) => write!(f, "{x}"),
            Value::Bool(b) => f.write_str(if *b { "#t" } else { "#f" }),
            Value::Sym(s) => f.write_str(s),
            Value::Str(s) => write_quoted(f, s),
            Value::Nil => f.write_str("()"),
            Value::Pair(cell) => write_list(f, cell),
            Value::Tagged { tag, data } => {
                if data.is_nil() {
                    write!(f, "({tag})")
                } else {
                    write!(f, "({tag} {data})")
                }
            }
            Value::Primitive { name, .. } => write!(f, "#[primitive {name}]"),
            Value::Closure(_) => f.write_str("#[compound-procedure]"),
            Value::Thunk(_) => f.write_str("#[thunk]"),
            Value::CompiledProc(p) => write!(f, "#[compiled-procedure {}]", p.entry),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => f.debug_tuple("Int").field(n).finish(),
            Value::Real(x) => f.debug_tuple("Real").field(x).finish(),
            Value::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            Value::Sym(s) => f.debug_tuple("Sym").field(s).finish(),
            Value::Str(s) => f.debug_tuple("Str").field(s).finish(),
            Value::Nil => f.write_str("Nil"),
            Value::Pair(cell) => f.debug_tuple("Pair").field(cell).finish(),
            Value::Tagged { tag, data } => f
                .debug_struct("Tagged")
                .field("tag", tag)
                .field("data", data)
                .finish(),
            Value::Primitive { name, .. } => {
                f.debug_struct("Primitive").field("name", name).finish()
            }
            Value::Closure(c) => f.debug_tuple("Closure").field(c).finish(),
            Value::Thunk(state) => f.debug_tuple("Thunk").field(state).finish(),
            Value::CompiledProc(p) => f.debug_tuple("CompiledProc").field(p).finish(),
        }
    }
}

/// Writes `s` double-quoted, escaping quotes and backslashes only, the
/// escapes the book's strings ever carry.
fn write_quoted(f: &mut Formatter<'_>, s: &str) -> fmt::Result {
    f.write_char('"')?;
    for c in s.chars() {
        if c == '"' || c == '\\' {
            f.write_char('\\')?;
        }
        f.write_char(c)?;
    }
    f.write_char('"')
}

/// Writes the book's pair notation: `(a b c)` for a proper list,
/// `(a . b)` across a dotted tail.
fn write_list(f: &mut Formatter<'_>, start: &Rc<ConsCell>) -> fmt::Result {
    f.write_str("(")?;
    write!(f, "{}", start.car.borrow())?;
    let mut cursor = start.cdr.borrow().clone();
    loop {
        match cursor {
            Value::Nil => break,
            Value::Pair(cell) => {
                f.write_str(" ")?;
                write!(f, "{}", cell.car.borrow())?;
                cursor = cell.cdr.borrow().clone();
            }
            other => {
                write!(f, " . {other}")?;
                break;
            }
        }
    }
    f.write_str(")")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::Env;
    use crate::error::SchemeError;
    use crate::pair::cons_cell;

    #[test]
    fn list_items_round_trips_a_proper_list() {
        assert_eq!(Value::Nil.list_items(), Ok(Vec::new()));
        let list = Value::list(vec![Value::int(1), Value::int(2), Value::int(3)]);
        assert_eq!(
            list.list_items(),
            Ok(vec![Value::int(1), Value::int(2), Value::int(3)])
        );
    }

    #[test]
    fn list_items_rejects_a_dotted_list() {
        let dotted = Value::Pair(cons_cell(Value::int(1), Value::int(2)));
        assert_eq!(
            dotted.list_items(),
            Err(SchemeError::TypeMismatch(
                "not a proper list: (1 . 2)".to_string()
            ))
        );
    }

    #[test]
    fn call_runs_the_primitive_handler() {
        let sum: Handler = Rc::new(|args: &[Value]| {
            args.iter()
                .try_fold(0i128, |acc, v| match v {
                    Value::Int(n) => Ok(acc + n),
                    other => Err(SchemeError::TypeMismatch(format!("not a number: {other}"))),
                })
                .map(Value::int)
        });
        let plus = Value::Primitive {
            name: Rc::from("+"),
            f: sum,
        };
        assert_eq!(
            plus.call(&[Value::int(3), Value::int(4)]),
            Ok(Value::int(7))
        );
    }

    #[test]
    fn call_on_a_non_procedure_raises_not_procedure() {
        assert_eq!(
            Value::int(3).call(&[]),
            Err(SchemeError::NotProcedure(Value::int(3)))
        );
    }

    #[test]
    fn display_prints_tagged_data_both_ways() {
        let bare = Value::tagged("rectangular", Value::Nil);
        assert_eq!(bare.to_string(), "(rectangular)");
        let payload = Value::tagged(
            "rectangular",
            Value::list(vec![Value::real(3.0), Value::real(4.0)]),
        );
        assert_eq!(payload.to_string(), "(rectangular (3 4))");
    }

    #[test]
    fn display_prints_procedure_variants() {
        let identity: Handler = Rc::new(|args: &[Value]| Ok(args[0].clone()));
        let primitive = Value::Primitive {
            name: Rc::from("identity"),
            f: identity,
        };
        assert_eq!(primitive.to_string(), "#[primitive identity]");

        let closure = Value::Closure(Rc::new(Closure {
            params: vec![Rc::from("x")],
            rest: None,
            body: vec![Value::sym("x")],
            env: Env::global(),
        }));
        assert_eq!(closure.to_string(), "#[compound-procedure]");

        let thunk = ThunkState::delay(Value::int(1), &Env::global());
        assert_eq!(thunk.to_string(), "#[thunk]");

        let compiled = Value::CompiledProc(Rc::new(CompiledProc {
            entry: Rc::from("compiled-entry"),
            params: vec![Rc::from("x")],
            env: Env::global(),
        }));
        assert_eq!(compiled.to_string(), "#[compiled-procedure compiled-entry]");
    }

    #[test]
    fn display_escapes_quotes_and_backslashes_in_strings() {
        assert_eq!(Value::string("plain").to_string(), "\"plain\"");
        assert_eq!(Value::string("a\"b\\c").to_string(), "\"a\\\"b\\\\c\"");
    }

    #[test]
    fn display_prints_a_dotted_pair() {
        let dotted = Value::Pair(cons_cell(
            Value::int(1),
            Value::Pair(cons_cell(Value::int(2), Value::int(3))),
        ));
        assert_eq!(dotted.to_string(), "(1 2 . 3)");
    }
}
