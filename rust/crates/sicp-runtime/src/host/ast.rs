// SPDX-License-Identifier: GPL-3.0-only

//! The located surface syntax of the Rust host subset (grammar §2 and
//! §4): items, types, statements, expressions, and patterns, each node
//! carrying its [`Span`]. This tree is exactly the accepted grammar —
//! every construct the contract excludes is rejected here or by
//! [`check`](crate::host::check), never represented.

use crate::host::diag::Span;
use crate::host::hir::{BinOp, UnOp};
use crate::host::lexer::IntSuffix;

/// An identifier with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    /// The spelling: `[A-Za-z_][A-Za-z0-9_]*`.
    pub name: String,
    /// The identifier's location.
    pub span: Span,
}

/// A whole guest source: zero or more items in one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    /// The items, in source order.
    pub items: Vec<Item>,
}

/// The derive names the grammar admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeriveName {
    /// `#[derive(Clone)]`.
    Clone,
    /// `#[derive(Debug)]`.
    Debug,
    /// `#[derive(PartialEq)]`.
    PartialEq,
    /// `#[derive(Eq)]`.
    Eq,
    /// `#[derive(Hash)]`.
    Hash,
}

/// One item of the single source module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// `use std::collections::HashMap` possibly renamed.
    Use(UseItem),
    /// A closed, nongeneric struct.
    Struct(StructItem),
    /// A closed, nongeneric enum.
    Enum(EnumItem),
    /// A local type alias.
    TypeAlias(TypeAliasItem),
    /// A function item.
    Fn(FnItem),
}

/// The one admitted import: `std::collections::HashMap` with an
/// optional rename.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseItem {
    /// The local name: `HashMap` or the `as` rename.
    pub local: Ident,
}

/// A struct declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructItem {
    /// The admitted derives.
    pub derives: Vec<DeriveName>,
    /// The struct's name.
    pub name: Ident,
    /// The named fields, or empty for a tuple struct.
    pub fields: Vec<FieldDecl>,
    /// The tuple-struct payload types, or empty for a named struct.
    pub tuple: Vec<Ty>,
}

/// One named field declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDecl {
    /// The field's name.
    pub name: Ident,
    /// The field's type.
    pub ty: Ty,
}

/// An enum declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumItem {
    /// The admitted derives.
    pub derives: Vec<DeriveName>,
    /// The enum's name.
    pub name: Ident,
    /// The variants, in declaration order.
    pub variants: Vec<Variant>,
}

/// One enum variant: unit, tuple, or named-fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    /// The variant's name.
    pub name: Ident,
    /// The tuple payload types, empty when absent.
    pub tuple: Vec<Ty>,
    /// The named fields, empty when absent.
    pub fields: Vec<FieldDecl>,
}

/// `type NAME = TY;`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAliasItem {
    /// The alias's name.
    pub name: Ident,
    /// The aliased type.
    pub ty: Ty,
}

/// A function item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnItem {
    /// The function's name.
    pub name: Ident,
    /// The parameters, in order.
    pub params: Vec<Param>,
    /// The declared return type; `None` means `()`.
    pub ret: Option<Ty>,
    /// The body.
    pub body: Block,
}

/// One function parameter: `mut? NAME : TY`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    /// Whether the parameter is a mutable binding.
    pub mutable: bool,
    /// The parameter's name.
    pub name: Ident,
    /// The parameter's explicit type.
    pub ty: Ty,
}

/// A type with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    /// The type's shape.
    pub kind: TyKind,
    /// The type's location.
    pub span: Span,
}

/// The admitted type grammar of grammar §3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    /// `()`.
    Unit,
    /// `bool`.
    Bool,
    /// `i64`.
    I64,
    /// `usize`.
    Usize,
    /// `&str`.
    Str,
    /// `String`.
    String,
    /// `&T` or `&mut T`.
    Ref(bool, Box<Ty>),
    /// `Box<T>`.
    Box(Box<Ty>),
    /// `Vec<T>`.
    Vec(Box<Ty>),
    /// `Option<T>`.
    Option(Box<Ty>),
    /// `Result<T, E>`.
    Result(Box<Ty>, Box<Ty>),
    /// `HashMap<String, T>`.
    HashMap(Box<Ty>),
    /// `[T; N]` with an integer-literal length.
    Array(Box<Ty>, String),
    /// `(T, U)`.
    Tuple(Box<Ty>, Box<Ty>),
    /// A local struct, enum, or alias name.
    Named(String),
    /// `fn(T, ...) -> R`.
    FnPtr(Vec<Ty>, Box<Ty>),
    /// `Box<dyn Fn|FnMut|FnOnce(T, ...) -> R + 'static>`.
    DynClosure(ClosureTrait, Vec<Ty>, Box<Ty>),
}

/// The closure traits a boxed closure object may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureTrait {
    /// `Fn`.
    Fn,
    /// `FnMut`.
    FnMut,
    /// `FnOnce`.
    FnOnce,
}

/// A braced block: statements and an optional final expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The statements, in order.
    pub stmts: Vec<Stmt>,
    /// The trailing expression, when the block has one.
    pub tail: Option<Box<Expr>>,
    /// The block's location.
    pub span: Span,
}

/// One statement of a block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    /// `let mut? PAT [: TY] = EXPR ;`
    Let(LetStmt),
    /// An expression statement; `semi` records the trailing `;`,
    /// which block-like statements omit.
    Expr {
        /// The expression.
        expr: Expr,
        /// Whether a semicolon followed.
        semi: bool,
    },
}

/// A `let` binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    /// Whether the binding is mutable.
    pub mutable: bool,
    /// The binding pattern: an identifier or a two-element tuple.
    pub pat: Pat,
    /// The optional explicit type.
    pub annotation: Option<Ty>,
    /// The initializer.
    pub value: Expr,
    /// The statement's location.
    pub span: Span,
}

/// An expression with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    /// The expression's shape.
    pub kind: ExprKind,
    /// The expression's location.
    pub span: Span,
}

/// The admitted expression forms of grammar §4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    /// An integer literal with its text and optional suffix.
    IntLit {
        /// The digit text, separators included.
        text: String,
        /// The written suffix, when any.
        suffix: Option<IntSuffix>,
    },
    /// A string literal's decoded value.
    StrLit(String),
    /// `true` or `false`.
    BoolLit(bool),
    /// `()`.
    UnitLit,
    /// A path: a local, a function, an enum variant, or a
    /// constructor such as `String::from`.
    Path(Vec<Ident>),
    /// `(A, B)`.
    Tuple(Box<Expr>, Box<Expr>),
    /// `[A, B, ...]`.
    Array(Vec<Expr>),
    /// `vec![A, ...]`.
    VecList(Vec<Expr>),
    /// `vec![VALUE; COUNT]`.
    VecRepeat(Box<Expr>, Box<Expr>),
    /// `format!`, `print!`, or `println!` with its format text and
    /// arguments.
    Format {
        /// The macro's kind.
        kind: FormatKind,
        /// The format string's raw text.
        fmt: String,
        /// The arguments, in order.
        args: Vec<Expr>,
    },
    /// `PATH { field: EXPR, field, ... }`.
    StructLit {
        /// The struct's or variant's path.
        path: Vec<Ident>,
        /// The initialized fields; `None` value means shorthand.
        fields: Vec<(Ident, Option<Expr>)>,
    },
    /// `BASE.field`.
    Field {
        /// The base expression.
        base: Box<Expr>,
        /// The field's name.
        name: Ident,
    },
    /// `BASE[INDEX]`.
    Index {
        /// The indexed place.
        base: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
    },
    /// `CALLEE(ARGS)`.
    Call {
        /// The callee expression.
        callee: Box<Expr>,
        /// The arguments.
        args: Vec<Expr>,
    },
    /// `RECEIVER.name(ARGS)`.
    MethodCall {
        /// The receiver.
        receiver: Box<Expr>,
        /// The method's name.
        name: Ident,
        /// The arguments.
        args: Vec<Expr>,
    },
    /// A unary operator.
    Unary {
        /// The operator.
        op: UnOp,
        /// The operand.
        operand: Box<Expr>,
    },
    /// A binary operator.
    Binary {
        /// The operator.
        op: BinOp,
        /// The left operand.
        left: Box<Expr>,
        /// The right operand.
        right: Box<Expr>,
    },
    /// `TARGET = VALUE`, `TARGET += VALUE`, or `TARGET -= VALUE`.
    Assign {
        /// The compound operator, when written.
        op: Option<BinOp>,
        /// The assigned place.
        target: Box<Expr>,
        /// The assigned value.
        value: Box<Expr>,
    },
    /// `if TEST THEN else ELSE`.
    If {
        /// The `bool` test.
        test: Box<Expr>,
        /// The then-block.
        then: Block,
        /// The else branch, when written.
        else_branch: Option<Box<Expr>>,
    },
    /// `if let PAT = VALUE THEN else ELSE`.
    IfLet {
        /// The matched pattern.
        pat: Pat,
        /// The scrutinee.
        value: Box<Expr>,
        /// The then-block.
        then: Block,
        /// The else branch, when written.
        else_branch: Option<Box<Expr>>,
    },
    /// `match SCRUTINEE { ARMS }`.
    Match {
        /// The scrutinee.
        scrutinee: Box<Expr>,
        /// The arms, in order.
        arms: Vec<MatchArm>,
    },
    /// A block in expression position.
    Block(Block),
    /// `loop BODY`.
    Loop(Block),
    /// `while TEST BODY`.
    While {
        /// The `bool` test.
        test: Box<Expr>,
        /// The loop body.
        body: Block,
    },
    /// `while let PAT = VALUE BODY`.
    WhileLet {
        /// The matched pattern.
        pat: Pat,
        /// The scrutinee.
        value: Box<Expr>,
        /// The loop body.
        body: Block,
    },
    /// `for PAT in ITERABLE BODY`.
    For {
        /// The binding pattern.
        pat: Pat,
        /// The iterable expression.
        iterable: Box<Expr>,
        /// The loop body.
        body: Block,
    },
    /// A closure expression.
    Closure {
        /// Whether `move` was written.
        mov: bool,
        /// The parameters, with optional explicit types.
        params: Vec<(Ident, Option<Ty>)>,
        /// The closure body.
        body: ClosureBody,
    },
    /// `return EXPR?`.
    Return(Option<Box<Expr>>),
    /// `break EXPR?`.
    Break(Option<Box<Expr>>),
    /// `continue`.
    Continue,
    /// The postfix `?` operator.
    Try(Box<Expr>),
    /// `START..END`, admitted only as an iterable range.
    Range(Box<Expr>, Box<Expr>),
}

/// Which admitted format macro a [`ExprKind::Format`] names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatKind {
    /// `format!`.
    Format,
    /// `print!`.
    Print,
    /// `println!`.
    Println,
}

/// A closure's body: a block or a bare expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureBody {
    /// `|...| { ... }`.
    Block(Block),
    /// `|...| EXPR`.
    Expr(Box<Expr>),
}

/// One `match` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    /// The arm's pattern.
    pub pat: Pat,
    /// The arm's body expression.
    pub body: Expr,
}

/// A pattern with its location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pat {
    /// The pattern's shape.
    pub kind: PatKind,
    /// The pattern's location.
    pub span: Span,
}

/// The admitted pattern grammar of grammar §4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatKind {
    /// `_`.
    Wild,
    /// A fresh binding identifier.
    Bind(Ident),
    /// An integer literal.
    IntLit {
        /// The digit text, separators included.
        text: String,
        /// The written suffix, when any.
        suffix: Option<IntSuffix>,
    },
    /// `true` or `false`.
    BoolLit(bool),
    /// `(A, B)`.
    Tuple(Box<Pat>, Box<Pat>),
    /// A bare path: a unit struct or unit enum variant, or a binding
    /// when it names neither.
    Path(Vec<Ident>),
    /// `PATH(P, ...)`: a tuple-struct or tuple-variant pattern.
    TuplePath(Vec<Ident>, Vec<Pat>),
    /// `PATH { field: PAT, field, ... }`: a struct or variant pattern.
    Struct {
        /// The matched path.
        path: Vec<Ident>,
        /// The field patterns; `None` sub-pattern means shorthand.
        fields: Vec<(Ident, Option<Pat>)>,
    },
}
