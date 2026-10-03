// SPDX-License-Identifier: GPL-3.0-only

//! The Rust host-subset source of chapters 4 and 5: the front end that
//! turns guest source into a checked program, the runtime values and
//! shared operations the teaching engines execute, and the explicit
//! data languages of the query and register-machine lessons.
//!
//! The contract is `spec/host-subsets/rust/grammar.md`. [`diag`] carries
//! the source locations and the rejection classes of the contract
//! (§6); [`lexer`] and [`parser`] produce the located syntax tree of
//! [`ast`]; [`check`] resolves, types, and ownership-checks that tree
//! into the typed [`hir`] program every engine consumes; [`value`] and
//! [`ops`] are the arena runtime and the shared leaf semantics those
//! engines execute; and [`query`] / [`machine`] are the closed data
//! languages of sections 4.4 and 5.1, which are values, not source
//! syntax.
//!
//! Nothing here parses, prints, or admits the edition's retired Lisp
//! surface: a guest program is Rust source or it is rejected before any
//! effect.

pub mod ast;
pub mod check;
pub mod diag;
pub mod hir;
pub mod lexer;
pub mod machine;
pub mod ops;
pub mod parser;
pub mod query;
pub mod value;

pub use ast::{Block, Expr, ExprKind, Ident, Item, Param, Pat, PatKind, Program, Stmt, Ty, TyKind};
pub use check::{CheckedProgram, check_program};
pub use diag::{Diag, DiagKind, Span};
pub use hir::{
    BinOp, BindId, CaptureMode, ClosureKind, FunId, HirExpr, HostTy, Place, PlaceRoot, Sema, UnOp,
};
pub use lexer::{Tok, TokKind, lex};
pub use ops::{Engine, Flow, RunOutcome, TrapReport};
pub use parser::{ParseError, parse_program};
pub use query::{Query, Substitution, Term};
pub use value::{HostValue, Trap};

/// Admits one guest source text: lexes, parses, and checks it, answering
/// the typed program or the first rejection.
///
/// # Errors
/// The first [`Diag`] the front end raises: [`DiagKind::Syntax`] from
/// the lexer or parser, then [`DiagKind::Type`], [`DiagKind::Ownership`],
/// or [`DiagKind::Unsupported`] from the checker. No effect can occur
/// while this returns an error.
pub fn admit(source: &str) -> Result<CheckedProgram, Diag> {
    let program = parse_program(source)?;
    check_program(&program)
}
