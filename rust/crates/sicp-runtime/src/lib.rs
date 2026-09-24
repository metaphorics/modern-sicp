// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Shared runtime for the Rust edition: the seeded `random` every
//! probabilistic section runs on, the dynamic `Value` of chapters 2
//! through 4, and the substrate every chapter shares — mutable pairs, the
//! persistent `List`, the memoized `Lazy` and `Stream`, the environment
//! chain, the operation-and-tag table, the numeric tower, the typed
//! pending marker, and the edition's one typed error.

mod env;
mod error;
mod key;
mod lazy;
mod list;
mod number;
mod optable;
mod pair;
mod pending;
mod random;
mod stream;
mod value;

pub use env::Env;
pub use error::SchemeError;
pub use key::Key;
pub use lazy::Lazy;
pub use list::List;
pub use number::Number;
pub use optable::OpTable;
pub use pair::{ConsCell, Pair, car, cdr, cons_cell, eq_pair, set_car, set_cdr};
pub use pending::Pending;
pub use random::Random;
pub use stream::{Stream, StreamIter};
pub use value::{Closure, CompiledProc, Handler, Symbol, ThunkState, Value};
