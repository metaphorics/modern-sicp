// SPDX-License-Identifier: GPL-3.0-only

//! The typed representation every teaching engine consumes (grammar §1
//! and §3): resolved names, concrete types, explicit places, and
//! explicit closure captures. Direct evaluation, analyzed evaluation,
//! explicit-control execution, and generated code all run this one
//! representation; nothing here can bypass the checks that produced it.

use std::collections::HashMap;

pub use crate::host::ast::FormatKind;
use crate::host::diag::Span;

/// A binding identity: one immutable or mutable local slot in one
/// function activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindId(pub u32);

/// A top-level function identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunId(pub u32);

/// A struct, enum, or alias item identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u32);

/// A syntax node identity, assigned in traversal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// The unary operators of grammar §4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    /// `-`: integer negation, checked at run time.
    Neg,
    /// `!`: `bool` inversion.
    Not,
    /// `*`: dereference of a reference place.
    Deref,
    /// `&`: a shared borrow.
    Ref,
    /// `&mut`: an exclusive borrow.
    RefMut,
}

/// The binary operators of grammar §3 and §4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// `+`: admitted integers.
    Add,
    /// `-`: admitted integers.
    Sub,
    /// `*`: `i64` only.
    Mul,
    /// `/`: `i64` only, truncating toward zero.
    Div,
    /// `%`: `i64` only, Rust's sign rule.
    Rem,
    /// `==`.
    Eq,
    /// `!=`.
    Ne,
    /// `<`.
    Lt,
    /// `<=`.
    Le,
    /// `>`.
    Gt,
    /// `>=`.
    Ge,
    /// `&&`: short-circuit `bool`.
    And,
    /// `||`: short-circuit `bool`.
    Or,
}

/// How a closure uses one captured binding (grammar §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    /// The capture reads through a shared reference; later writes to
    /// the captured binding are visible.
    Shared,
    /// The capture mutates through an exclusive reference.
    Mut,
    /// The capture owns the value; non-`Copy` values move at closure
    /// creation.
    Owned,
}

/// The closure kind Rust's capture rules determine (grammar §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClosureKind {
    /// `Fn`: callable through a shared borrow.
    Fn,
    /// `FnMut`: callable through an exclusive borrow.
    FnMut,
    /// `FnOnce`: consumed by a call.
    FnOnce,
}

/// A place: a binding or dereference root followed by projections.
#[derive(Debug, Clone)]
pub struct Place {
    /// The place's root.
    pub root: PlaceRoot,
    /// The projections applied left to right.
    pub proj: Vec<Proj>,
    /// The place expression's location.
    pub span: Span,
}

impl Place {
    /// The place's location, for diagnostics about its projections.
    #[must_use]
    pub fn root_span(&self) -> Span {
        self.span
    }
}

/// The root a place addresses.
#[derive(Debug, Clone)]
pub enum PlaceRoot {
    /// A local binding slot in the current activation.
    Local(BindId),
    /// The referent of a reference-valued expression.
    Deref(Box<HirExpr>),
}

/// One place projection.
#[derive(Debug, Clone)]
pub enum Proj {
    /// `.field` by resolved field index.
    Field(u32),
    /// `[index]` with its checked index expression.
    Index(Box<HirExpr>),
}

/// How an expression uses a place (grammar §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceUse {
    /// A `Copy` read: the place keeps its value.
    Read,
    /// A move: the place is uninitialized afterwards until reassigned.
    Move,
}

/// The concrete admitted types of grammar §3. Unification leaves no
/// variable in a checked program.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HostTy {
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
    Ref(bool, Box<HostTy>),
    /// `Box<T>`.
    Box(Box<HostTy>),
    /// `Vec<T>`.
    Vec(Box<HostTy>),
    /// `Option<T>`.
    Option(Box<HostTy>),
    /// `Result<T, E>`.
    Result(Box<HostTy>, Box<HostTy>),
    /// `HashMap<String, T>`.
    HashMap(Box<HostTy>),
    /// `[T; N]`.
    Array(Box<HostTy>, u64),
    /// `(T, U)`.
    Tuple(Box<HostTy>, Box<HostTy>),
    /// A user struct.
    Struct(ItemId),
    /// A user enum.
    Enum(ItemId),
    /// `fn(T, ...) -> R`.
    FnPtr(Vec<HostTy>, Box<HostTy>),
    /// `Box<dyn Fn|FnMut|FnOnce(T, ...) -> R + 'static>`.
    DynFn(ClosureKind, Vec<HostTy>, Box<HostTy>),
    /// `START..END` iterator items: one admitted integer type.
    Range(Box<HostTy>),
    /// `Vec::iter` / `&[T; N]` iteration: items are `&T`.
    Iter(Box<HostTy>),
    /// `Vec::iter_mut`: items are `&mut T`.
    IterMut(Box<HostTy>),
    /// `Vec::into_iter` and by-value `for`: items are `T`.
    IntoIter(Box<HostTy>),
    /// `Iterator::enumerate`: items are `(usize, ITEM)`.
    Enumerate(Box<HostTy>),
    /// `Iterator::zip`: items are `(LEFT, RIGHT)`.
    Zip(Box<HostTy>, Box<HostTy>),
    /// A checker-internal type variable. A checked program never
    /// contains one: [`check`](crate::host::check) resolves every cell
    /// before it answers [`CheckedProgram`](crate::host::check::CheckedProgram).
    Infer(usize),
}

impl HostTy {
    /// Whether values of this type are `Copy` (grammar §3): the
    /// scalars, shared references, and function pointers, and, as in
    /// Rust, a tuple, array, `Option`, or `Result` exactly when its
    /// components are. User data (no `Copy` derive is admitted),
    /// `String`, `Vec`, `HashMap`, `Box`, mutable references, boxed
    /// closures, and iterators are moved.
    #[must_use]
    pub fn is_copy(&self) -> bool {
        match self {
            Self::Unit | Self::Bool | Self::I64 | Self::Usize | Self::Str | Self::FnPtr(..) => true,
            Self::Ref(mutable, _) => !*mutable,
            Self::Option(inner) | Self::Array(inner, _) => inner.is_copy(),
            Self::Tuple(left, right) | Self::Result(left, right) => {
                left.is_copy() && right.is_copy()
            }
            Self::Infer(_)
            | Self::Range(_)
            | Self::Iter(_)
            | Self::IterMut(_)
            | Self::IntoIter(_)
            | Self::Enumerate(_)
            | Self::Zip(..)
            | Self::Box(_)
            | Self::Vec(_)
            | Self::HashMap(_)
            | Self::Struct(_)
            | Self::Enum(_)
            | Self::String
            | Self::DynFn(..) => false,
        }
    }

    /// The referent type behind one level of reference, with whether
    /// the reference is exclusive.
    #[must_use]
    pub fn referent(&self) -> Option<(bool, &HostTy)> {
        match self {
            Self::Ref(mutable, inner) => Some((*mutable, inner)),
            _ => None,
        }
    }
}

/// The static kind one path resolved to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolved {
    /// A local binding slot.
    Local(BindId),
    /// A top-level function.
    Fun(FunId),
    /// A unit struct constructor.
    UnitStruct(ItemId),
    /// A tuple-struct constructor.
    TupleStruct(ItemId),
    /// An enum variant constructor; the payload is built by a
    /// separate variant expression.
    Variant(ItemId, u32),
    /// An admitted associated constructor such as `String::from`,
    /// `Vec::new`, or `Box::new`.
    Ctor(CtorOp),
    /// The imported `HashMap` name or an admitted type name in
    /// expression position.
    TypeName(&'static str),
}

/// The closed set of admitted constructors and methods (grammar §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtorOp {
    /// `String::from`.
    StringFrom,
    /// `Vec::new`.
    VecNew,
    /// `Vec::with_capacity`.
    VecWithCapacity,
    /// `HashMap::new`.
    MapNew,
    /// `Box::new`.
    BoxNew,
    /// `Some(value)`.
    OptSome,
    /// `None`.
    OptNone,
    /// `Ok(value)`.
    ResOk,
    /// `Err(value)`.
    ResErr,
}

/// The closed method allowlist, resolved by receiver type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodOp {
    /// `String::push_str`.
    StrPushStr,
    /// `String::as_str`.
    AsStr,
    /// `String::len`, in bytes like the native method.
    StrLen,
    /// `Vec::push`.
    VecPush,
    /// `Vec::pop`.
    VecPop,
    /// `Vec::len`.
    VecLen,
    /// `Vec::is_empty`.
    VecIsEmpty,
    /// `Vec::get`.
    VecGet,
    /// `Vec::get_mut`.
    VecGetMut,
    /// `HashMap::insert`.
    MapInsert,
    /// `HashMap::get`.
    MapGet,
    /// `HashMap::get_mut`.
    MapGetMut,
    /// `HashMap::contains_key`.
    MapContainsKey,
    /// `HashMap::remove`.
    MapRemove,
    /// `HashMap::len`.
    MapLen,
    /// `HashMap::is_empty`.
    MapIsEmpty,
    /// `Box::as_ref`.
    BoxAsRef,
    /// `Box::as_mut`.
    BoxAsMut,
    /// `clone` on any admitted `Clone` receiver.
    Clone,
    /// Iterator production: `iter`, `iter_mut`, `into_iter`.
    Iter,
    /// Iterator production: `iter_mut`.
    IterMut,
    /// Iterator production: `into_iter`.
    IntoIter,
    /// Iterator stepping: `next`.
    Next,
    /// `enumerate`.
    Enumerate,
    /// `zip`.
    Zip,
}

/// A captured binding inside a closure value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capture {
    /// The captured parent binding.
    pub binding: BindId,
    /// How the closure uses it.
    pub mode: CaptureMode,
    /// Whether the closure body consumes the capture on each call,
    /// which makes the closure `FnOnce`.
    pub consumed: bool,
}

/// The typed function definition.
#[derive(Debug, Clone)]
pub struct FunDef {
    /// The function's identity.
    pub id: FunId,
    /// The function's name.
    pub name: String,
    /// The parameters as fresh bindings.
    pub params: Vec<(BindId, String)>,
    /// The return type.
    pub ret: HostTy,
    /// The first binding slot the activation allocates.
    pub bind_base: u32,
    /// The number of local slots the activation allocates.
    pub frame_slots: u32,
    /// The body.
    pub body: HirBlock,
}

/// A named item's static definition.
#[derive(Debug, Clone)]
pub struct ItemDef {
    /// The item's identity.
    pub id: ItemId,
    /// The item's name.
    pub name: String,
    /// The item's kind.
    pub kind: ItemKind,
}

/// What an [`ItemDef`] declares.
#[derive(Debug, Clone)]
pub enum ItemKind {
    /// A struct with its ordered field names and types; tuple structs
    /// name their fields `0`, `1`, and so on.
    Struct(Vec<(String, HostTy)>),
    /// An enum with its ordered variants, each carrying ordered field
    /// names and types.
    Enum(Vec<(String, Vec<(String, HostTy)>)>),
    /// A type alias's resolved type.
    Alias(HostTy),
}

/// The whole checked program: the item table, the function table, and
/// the entry point.
#[derive(Debug, Clone)]
pub struct Sema {
    /// The declared items, in source order.
    pub items: Vec<ItemDef>,
    /// The top-level functions, in source order.
    pub funs: Vec<FunDef>,
    /// The `main` function.
    pub main: FunId,
    /// The `HashMap` local rename, when the program imported one.
    pub map_alias: Option<String>,
    /// Binding metadata for diagnostics and runtime slots.
    pub bindings: Vec<BindingInfo>,
}

impl Sema {
    /// Looks a function up by name.
    #[must_use]
    pub fn fun_by_name(&self, name: &str) -> Option<&FunDef> {
        self.funs.iter().find(|fun| fun.name == name)
    }

    /// Looks an item up by name.
    #[must_use]
    pub fn item_by_name(&self, name: &str) -> Option<&ItemDef> {
        self.items.iter().find(|item| item.name == name)
    }
}

/// One binding's static metadata.
#[derive(Debug, Clone)]
pub struct BindingInfo {
    /// The binding's name.
    pub name: String,
    /// The binding's type.
    pub ty: HostTy,
    /// Whether the binding is mutable.
    pub mutable: bool,
}

/// A typed block.
#[derive(Debug, Clone)]
pub struct HirBlock {
    /// The statements, in order.
    pub stmts: Vec<HirStmt>,
    /// The trailing expression, when present.
    pub tail: Option<Box<HirExpr>>,
}

/// A typed statement.
#[derive(Debug, Clone)]
pub enum HirStmt {
    /// A binding: fresh slot, optional move source, and initializer.
    Let {
        /// The fresh binding slot.
        binding: BindId,
        /// The tuple destructuring sources, for two-element tuple
        /// patterns.
        destruct: Option<(BindId, BindId)>,
        /// The initializer.
        value: HirExpr,
    },
    /// An expression evaluated for its effects or value.
    Expr(HirExpr),
}

/// A typed expression.
#[derive(Debug, Clone)]
pub struct HirExpr {
    /// The expression's shape.
    pub kind: HirExprKind,
    /// The expression's static type.
    pub ty: HostTy,
    /// Whether the expression diverges (`return`, `break`,
    /// `continue`, or an always-exiting loop).
    pub diverges: bool,
    /// The expression's location.
    pub span: Span,
}

/// The typed expression forms.
#[derive(Debug, Clone)]
pub enum HirExprKind {
    /// An `i64` literal.
    I64(i64),
    /// A `usize` literal.
    Usize(u64),
    /// A `bool` literal.
    Bool(bool),
    /// `()`.
    Unit,
    /// A string literal's value.
    Str(String),
    /// A place read or move.
    Place {
        /// The addressed place.
        place: Place,
        /// The read mode.
        mode: PlaceUse,
    },
    /// A reference to a top-level function as a value.
    FunRef(FunId),
    /// A struct literal with fields in declaration order.
    StructLit(ItemId, Vec<HirExpr>),
    /// A tuple-struct literal.
    TupleStructLit(ItemId, Vec<HirExpr>),
    /// An enum variant construction.
    VariantLit(ItemId, u32, Vec<HirExpr>),
    /// `(A, B)`.
    Tuple(Box<HirExpr>, Box<HirExpr>),
    /// `[A, ...]`.
    Array(Vec<HirExpr>),
    /// `vec![A, ...]`.
    VecList(Vec<HirExpr>),
    /// `vec![VALUE; COUNT]`.
    VecRepeat(Box<HirExpr>, Box<HirExpr>),
    /// `format!`, `print!`, or `println!`.
    Format {
        /// The macro's kind.
        kind: FormatKind,
        /// The parsed format specification.
        spec: FormatSpec,
        /// The arguments, in order.
        args: Vec<HirExpr>,
    },
    /// A field read.
    Field {
        /// The base.
        base: Box<HirExpr>,
        /// The resolved field index.
        index: u32,
    },
    /// An index read on a vector or array.
    Index {
        /// The base.
        base: Box<HirExpr>,
        /// The index.
        index: Box<HirExpr>,
    },
    /// A call to a resolved function.
    Call {
        /// The resolved callee.
        callee: FunId,
        /// The arguments.
        args: Vec<HirExpr>,
    },
    /// An admitted constructor: `String::from`, `Vec::new`,
    /// `HashMap::new`, `Box::new`, and the `Option`/`Result` forms.
    Ctor(CtorOp, Vec<HirExpr>),
    /// A call through a closure or function-pointer value.
    IndirectCall {
        /// The callee value.
        callee: Box<HirExpr>,
        /// The arguments.
        args: Vec<HirExpr>,
    },
    /// An admitted method call.
    Method {
        /// The method to run.
        op: MethodOp,
        /// The receiver, a place or value.
        receiver: Box<HirExpr>,
        /// The receiver's place, when the method takes it by
        /// reference.
        receiver_place: Option<Place>,
        /// The arguments.
        args: Vec<HirExpr>,
    },
    /// A unary operator.
    Unary {
        /// The operator.
        op: UnOp,
        /// The operand.
        operand: Box<HirExpr>,
    },
    /// A binary operator.
    Binary {
        /// The operator.
        op: BinOp,
        /// The left operand.
        left: Box<HirExpr>,
        /// The right operand.
        right: Box<HirExpr>,
    },
    /// An assignment to a checked place.
    Assign {
        /// The compound operator, when written.
        op: Option<BinOp>,
        /// The assigned place.
        target: Place,
        /// The assigned value.
        value: Box<HirExpr>,
    },
    /// `if`.
    If {
        /// The `bool` test.
        test: Box<HirExpr>,
        /// The then-branch.
        then: Box<HirExpr>,
        /// The else branch; a unit literal when absent.
        else_branch: Box<HirExpr>,
    },
    /// `if let`.
    IfLet {
        /// The checked pattern.
        pat: HirPat,
        /// The scrutinee.
        value: Box<HirExpr>,
        /// The then-branch.
        then: Box<HirExpr>,
        /// The else branch; a unit literal when absent.
        else_branch: Box<HirExpr>,
    },
    /// `match` with exhaustive arms.
    Match {
        /// The scrutinee.
        scrutinee: Box<HirExpr>,
        /// The arms, in order.
        arms: Vec<(HirPat, HirExpr)>,
    },
    /// A block in expression position.
    Block(HirBlock),
    /// `loop` with its typed `break` value type.
    Loop {
        /// The loop body.
        body: HirBlock,
        /// The type `break` produces.
        break_ty: HostTy,
    },
    /// `while`.
    While {
        /// The `bool` test.
        test: Box<HirExpr>,
        /// The body.
        body: HirBlock,
    },
    /// `while let`.
    WhileLet {
        /// The matched pattern.
        pat: HirPat,
        /// The scrutinee.
        value: Box<HirExpr>,
        /// The body.
        body: HirBlock,
    },
    /// `for` over a checked iterable.
    For {
        /// The binding pattern.
        pat: HirPat,
        /// The iterable.
        iterable: Box<HirExpr>,
        /// The body.
        body: HirBlock,
    },
    /// A closure value.
    Closure(HirClosure),
    /// `return`.
    Return(Option<Box<HirExpr>>),
    /// `break` with its typed value.
    Break(Option<Box<HirExpr>>),
    /// `continue`.
    Continue,
    /// The postfix `?` on `Option` or `Result`.
    Try(Box<HirExpr>),
    /// `START..END`.
    Range(Box<HirExpr>, Box<HirExpr>),
}

/// A checked closure.
#[derive(Debug, Clone)]
pub struct HirClosure {
    /// The closure's inferred kind.
    pub kind: ClosureKind,
    /// The first binding slot the closure activation allocates.
    pub bind_base: u32,
    /// The number of local slots the closure activation allocates.
    pub frame_slots: u32,
    /// The parameters as fresh bindings with their types.
    pub params: Vec<(BindId, HostTy)>,
    /// The body: a block or a trailing expression.
    pub body: HirBlock,
    /// The captured parent bindings.
    pub captures: Vec<Capture>,
    /// The result type.
    pub ret: HostTy,
}

/// A checked pattern: bindings are fresh slots and constructors are
/// resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirPat {
    /// The pattern's shape.
    pub kind: HirPatKind,
    /// The pattern's location.
    pub span: Span,
}

/// The typed pattern forms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirPatKind {
    /// `_`.
    Wild,
    /// A fresh binding.
    Bind(BindId),
    /// An `i64` literal pattern.
    I64(i64),
    /// A `usize` literal pattern.
    Usize(u64),
    /// A `bool` literal pattern.
    Bool(bool),
    /// `(A, B)`.
    Tuple(Box<HirPat>, Box<HirPat>),
    /// A unit struct or unit variant.
    UnitPath(Resolved),
    /// A tuple-struct or tuple-variant pattern.
    TuplePath(Resolved, Vec<HirPat>),
    /// A struct or variant pattern with one sub-pattern per field in
    /// declaration order: the subset has no `..`, so none is omitted.
    StructPath(Resolved, Vec<HirPat>),
}

/// The parsed form of a format string (grammar §4): literal pieces
/// alternating with `{}` / `{:?}` placeholders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatSpec {
    /// The literal pieces; `pieces.len() == args + 1`.
    pub pieces: Vec<String>,
    /// Whether each placeholder is `{:?}`.
    pub debug: Vec<bool>,
}

/// The typed shape of one expression node, shared by engines that key
/// maps on node identity.
pub type NodeTypes = HashMap<NodeId, HostTy>;
