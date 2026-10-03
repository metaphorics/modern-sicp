// SPDX-License-Identifier: GPL-3.0-only

//! The runtime values the teaching engines execute (grammar §1, §3,
//! §5): an arena of frames addressed by index, values that move rather
//! than alias, and references that point at frame slots instead of
//! granting copies. Closures capture by the checked capture mode and
//! keep their defining environment alive through the arena, never
//! through a host pointer the guest could observe. Traps (grammar §6.4)
//! stop execution; stdout is the ordered effect log (grammar §6.5).

use std::collections::HashMap;

use crate::host::hir::{
    BindId, CaptureMode, ClosureKind, FunId, HirBlock, HostTy, ItemDef, ItemKind,
};

/// The reserved item marker `Option` values carry, so a user enum
/// can never collide with them.
pub const BUILTIN: u32 = u32::MAX;

/// The reserved item marker `Result` values carry: kept distinct
/// from [`BUILTIN`] so the `Debug` text can name `Ok`/`Err` — a
/// shared sentinel would erase which of the two built-in types the
/// variant belongs to.
pub const BUILTIN_RES: u32 = u32::MAX - 1;

/// A frame slot address: one binding of one activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Addr {
    /// The activation's arena index.
    pub frame: usize,
    /// The binding slot inside the activation.
    pub bind: BindId,
}

/// One runtime projection applied to a place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RtProj {
    /// `.field` by resolved index.
    Field(u32),
    /// `[index]` with its runtime value.
    Index(i64),
    /// `*box`, the inner place of a `Box<T>`.
    BoxDeref,
    /// The value slot of one `HashMap` entry, by key: a reference
    /// through this projection reads and writes the entry itself.
    MapKey(String),
    /// The key of one `HashMap` entry, by key: a shared reference
    /// through this projection reads the key text and is never written.
    MapKeyOf(String),
}

/// A checked execution trap (grammar §6.4): the run stops here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trap {
    /// Checked-build integer overflow in the named operation.
    Overflow(&'static str),
    /// Division or remainder by zero.
    DivByZero,
    /// A vector or array index out of bounds.
    IndexOutOfBounds,
    /// A runtime slot read after a move: the checker prevents this, so
    /// reaching it is an engine invariant violation.
    UseAfterMove,
    /// A reference target no longer in the arena: impossible by the
    /// arena discipline; kept as an explicit outcome.
    Dangling,
}

/// The ordered effect log of one run (grammar §6.5).
#[derive(Debug, Clone, Default)]
pub struct Effects {
    /// Everything `print!` and `println!` wrote, in evaluation order.
    pub stdout: String,
}

impl Effects {
    /// Records one rendered line, the `println!` discipline.
    pub fn push_line(&mut self, text: &str) {
        self.stdout.push_str(text);
        self.stdout.push('\n');
    }

    /// Records one rendered fragment, the `print!` discipline.
    pub fn push_text(&mut self, text: &str) {
        self.stdout.push_str(text);
    }
}

/// A closure value: the checked capture list plus the body the
/// engines execute.
#[derive(Debug, Clone)]
pub struct ClosureVal {
    /// The closure's kind (grammar §5).
    pub kind: ClosureKind,
    /// The parameters as binding slots.
    pub params: Vec<(BindId, HostTy)>,
    /// The closure body.
    pub body: std::sync::Arc<HirBlock>,
    /// The captured slots in creation order.
    pub captures: Vec<(BindId, CaptureMode, HostValue)>,
    /// The result type.
    pub ret: HostTy,
    /// The number of slots the closure activation allocates.
    pub frame_slots: u32,
    /// The first binding slot the closure activation allocates.
    pub bind_base: u32,
    /// The compiled entry label, for closures the compiler built:
    /// the VM jumps to the pre-compiled instructions and never
    /// interprets syntax.
    pub compiled_entry: Option<String>,
}

/// The iterator shapes the admitted iterator operations produce.
#[derive(Debug, Clone)]
pub enum IterVal {
    /// `START..END` over `i64`.
    Range {
        /// The next value.
        cur: i64,
        /// The excluded end.
        end: i64,
    },
    /// `START..END` over `usize`.
    RangeU {
        /// The next value.
        cur: u64,
        /// The excluded end.
        end: u64,
    },
    /// By-value iteration over owned elements.
    Items {
        /// The owned elements.
        items: Vec<HostValue>,
        /// The next position.
        pos: usize,
    },
    /// Element references into one collection place.
    Refs {
        /// The collection's slot.
        addr: Addr,
        /// The projections from the slot to the collection.
        base: Vec<RtProj>,
        /// The element count at iteration start.
        len: usize,
        /// The next position.
        pos: usize,
        /// Whether the element references are exclusive.
        mutable: bool,
    },
    /// `enumerate`.
    Enumerate {
        /// The wrapped iterator.
        inner: Box<IterVal>,
        /// The next position.
        pos: usize,
    },
    /// `zip`.
    Zip {
        /// The left iterator.
        left: Box<IterVal>,
        /// The right iterator.
        right: Box<IterVal>,
    },
}

/// The dynamic value of one checked slot. Values move: cloning happens
/// only where the checker admitted `Copy` or an explicit `clone()`.
#[derive(Debug, Clone)]
pub enum HostValue {
    /// `i64` values.
    Int(i64),
    /// `usize` values: the full admitted `u64` range (grammar §3).
    Usize(u64),
    /// `bool`.
    Bool(bool),
    /// `()`.
    Unit,
    /// `String` and `&str` values; `&str` is a borrowed slice view.
    Text(String),
    /// `Vec<T>`.
    Vec(Vec<HostValue>),
    /// `HashMap<String, T>`.
    Map(HashMap<String, HostValue>),
    /// `Box<T>`.
    Box(Box<HostValue>),
    /// `[T; N]`.
    Array(Vec<HostValue>),
    /// `(T, U)`.
    Tuple(Box<HostValue>, Box<HostValue>),
    /// A user struct with fields in declaration order.
    Struct(u32, Vec<HostValue>),
    /// A user enum variant with payload in declaration order.
    Variant(u32, u32, Vec<HostValue>),
    /// A top-level function as a value.
    FnPtr(FunId),
    /// A closure value.
    Closure(std::sync::Arc<ClosureVal>),
    /// A shared or exclusive reference into a frame slot.
    Ref {
        /// The referenced slot.
        addr: Addr,
        /// The projections from the slot.
        projs: Vec<RtProj>,
        /// Whether the reference is exclusive.
        mutable: bool,
    },
    /// An iterator value.
    Iter(Box<IterVal>),
}

impl HostValue {
    /// The Rust `Display` rendering of the admitted scalar forms.
    ///
    /// # Errors
    /// [`Trap::UseAfterMove`] is impossible here; the function returns
    /// `Result` so callers share one error channel with the engines.
    pub fn display_text(&self) -> Result<String, Trap> {
        Ok(match self {
            Self::Int(value) => value.to_string(),
            Self::Usize(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Text(text) => text.clone(),
            Self::Unit => "unit".to_owned(),
            other => format!("{other:?}"),
        })
    }

    /// The Rust `Debug` rendering of the admitted forms. Struct and
    /// variant values carry only their item index, so the declared
    /// `items` restore the type, variant, and field names a derived
    /// `Debug` impl prints; a `Box` forwards to its contents exactly
    /// like `Box`'s own `Debug` impl.
    #[must_use]
    pub fn debug_text(&self, items: &[ItemDef]) -> String {
        match self {
            Self::Int(value) => value.to_string(),
            Self::Usize(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Unit => "()".to_owned(),
            Self::Text(text) => format!("{text:?}"),
            Self::Vec(values) | Self::Array(values) => {
                let rendered: Vec<String> =
                    values.iter().map(|value| value.debug_text(items)).collect();
                format!("[{}]", rendered.join(", "))
            }
            Self::Map(entries) => {
                let mut keys: Vec<&String> = entries.keys().collect();
                keys.sort();
                let rendered: Vec<String> = keys
                    .iter()
                    .map(|key| format!("{key:?}: {}", entries[*key].debug_text(items)))
                    .collect();
                format!("{{{}}}", rendered.join(", "))
            }
            Self::Box(inner) => inner.debug_text(items),
            Self::Tuple(left, right) => {
                format!("({}, {})", left.debug_text(items), right.debug_text(items))
            }
            Self::Struct(item, fields) => {
                if let Some((name, fields_def)) = items.get(*item as usize).and_then(|def| {
                    if let ItemKind::Struct(names) = &def.kind {
                        Some((def.name.as_str(), names))
                    } else {
                        None
                    }
                }) {
                    render_ctor(name, fields_def, fields, items)
                } else {
                    let rendered: Vec<String> =
                        fields.iter().map(|value| value.debug_text(items)).collect();
                    format!("{{{}}}", rendered.join(", "))
                }
            }
            Self::Variant(item, variant, payload) => {
                if let Some(name) = builtin_variant_name(*item, *variant) {
                    let rendered: Vec<String> = payload
                        .iter()
                        .map(|value| value.debug_text(items))
                        .collect();
                    return if rendered.is_empty() {
                        name.to_owned()
                    } else {
                        format!("{name}({})", rendered.join(", "))
                    };
                }
                if let Some((name, fields_def)) = items.get(*item as usize).and_then(|def| {
                    if let ItemKind::Enum(variants) = &def.kind {
                        variants.get(*variant as usize)
                    } else {
                        None
                    }
                }) {
                    render_ctor(name, fields_def, payload, items)
                } else {
                    let rendered: Vec<String> = payload
                        .iter()
                        .map(|value| value.debug_text(items))
                        .collect();
                    format!("({})", rendered.join(", "))
                }
            }
            Self::FnPtr(id) => format!("fn{}", id.0),
            Self::Closure(_) => "closure".to_owned(),
            Self::Ref { .. } => "reference".to_owned(),
            Self::Iter(_) => "iterator".to_owned(),
        }
    }
}

/// The variant name a built-in `Option` or `Result` value carries:
/// rustc prints `Some`/`None`/`Ok`/`Err`, so the marker decides
/// which family the discriminant names.
fn builtin_variant_name(item: u32, variant: u32) -> Option<&'static str> {
    match (item, variant) {
        (BUILTIN, 0) => Some("Some"),
        (BUILTIN, _) => Some("None"),
        (BUILTIN_RES, 0) => Some("Ok"),
        (BUILTIN_RES, _) => Some("Err"),
        _ => None,
    }
}

/// Renders one struct or enum variant the way its derived `Debug`
/// impl does: `Name` when there are no fields, `Name(v, ..)` for
/// positional fields, `Name { f: v, .. }` for named ones. The
/// declared field names are `0`, `1`, and so on for positional
/// fields, which no user-written field name can be.
fn render_ctor(
    name: &str,
    fields: &[(String, HostTy)],
    values: &[HostValue],
    items: &[ItemDef],
) -> String {
    if fields.is_empty() {
        return name.to_owned();
    }
    let rendered: Vec<String> = values.iter().map(|value| value.debug_text(items)).collect();
    if fields
        .iter()
        .all(|(field, _)| field.bytes().next().is_some_and(|c| c.is_ascii_digit()))
    {
        format!("{name}({})", rendered.join(", "))
    } else {
        let named: Vec<String> = fields
            .iter()
            .zip(rendered)
            .map(|((field, _), value)| format!("{field}: {value}"))
            .collect();
        format!("{name} {{ {} }}", named.join(", "))
    }
}

/// One activation: a contiguous binding range plus its slots.
#[derive(Debug, Clone)]
pub struct Frame {
    /// The first binding slot of the activation.
    pub bind_base: u32,
    /// The slots, indexed by `bind - bind_base`.
    pub slots: Vec<Option<HostValue>>,
}

/// The arena of activations. Frames live for the whole run: an arena
/// index never dangles, which is exactly the discipline the guest
/// evaluator witness teaches.
#[derive(Debug, Clone, Default)]
pub struct Store {
    /// The frames, in creation order.
    pub frames: Vec<Frame>,
}

impl Store {
    /// Allocates one activation and answers its arena index.
    #[must_use]
    pub fn push_frame(&mut self, bind_base: u32, slots: u32) -> usize {
        let index = self.frames.len();
        self.frames.push(Frame {
            bind_base,
            slots: (0..slots).map(|_| None).collect(),
        });
        index
    }

    /// Writes one slot.
    ///
    /// # Errors
    /// [`Trap::Dangling`] when the address names no live slot; the
    /// arena discipline makes this unreachable for checked programs.
    pub fn write(&mut self, addr: Addr, value: HostValue) -> Result<(), Trap> {
        let frame = self.frames.get_mut(addr.frame).ok_or(Trap::Dangling)?;
        let offset = addr.bind.0.wrapping_sub(frame.bind_base) as usize;
        let slot = frame.slots.get_mut(offset).ok_or(Trap::Dangling)?;
        *slot = Some(value);
        Ok(())
    }

    /// Reads one slot without consuming it.
    ///
    /// # Errors
    /// [`Trap::UseAfterMove`] when the slot holds a move marker,
    /// [`Trap::Dangling`] for an address outside the arena.
    pub fn read(&self, addr: Addr) -> Result<HostValue, Trap> {
        self.read_ref(addr).cloned()
    }

    /// Borrows one slot's value without cloning it.
    ///
    /// # Errors
    /// The same conditions as [`Store::read`].
    pub(crate) fn read_ref(&self, addr: Addr) -> Result<&HostValue, Trap> {
        let frame = self.frames.get(addr.frame).ok_or(Trap::Dangling)?;
        let offset = addr.bind.0.wrapping_sub(frame.bind_base) as usize;
        let slot = frame.slots.get(offset).ok_or(Trap::Dangling)?;
        slot.as_ref().ok_or(Trap::UseAfterMove)
    }

    /// Moves one slot's value out, leaving the move marker.
    ///
    /// # Errors
    /// The same conditions as [`Store::read`].
    pub fn take(&mut self, addr: Addr) -> Result<HostValue, Trap> {
        let frame = self.frames.get_mut(addr.frame).ok_or(Trap::Dangling)?;
        let offset = addr.bind.0.wrapping_sub(frame.bind_base) as usize;
        let slot = frame.slots.get_mut(offset).ok_or(Trap::Dangling)?;
        slot.take().ok_or(Trap::UseAfterMove)
    }
}

impl PartialEq for HostValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Usize(a), Self::Usize(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Unit, Self::Unit) => true,
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Vec(a), Self::Vec(b)) | (Self::Array(a), Self::Array(b)) => a == b,
            (Self::Map(a), Self::Map(b)) => a == b,
            (Self::Box(a), Self::Box(b)) => a == b,
            (Self::Tuple(a1, a2), Self::Tuple(b1, b2)) => a1 == b1 && a2 == b2,
            (Self::Struct(i1, f1), Self::Struct(i2, f2)) => i1 == i2 && f1 == f2,
            (Self::Variant(i1, v1, p1), Self::Variant(i2, v2, p2)) => {
                i1 == i2 && v1 == v2 && p1 == p2
            }
            (Self::FnPtr(a), Self::FnPtr(b)) => a == b,
            (Self::Closure(a), Self::Closure(b)) => std::sync::Arc::ptr_eq(a, b),
            (
                Self::Ref {
                    addr: a1,
                    projs: p1,
                    mutable: m1,
                },
                Self::Ref {
                    addr: a2,
                    projs: p2,
                    mutable: m2,
                },
            ) => a1 == a2 && p1 == p2 && m1 == m2,
            _ => false,
        }
    }
}

/// The rendering contract the corpus holds observed output to.
#[must_use]
pub fn render_display(value: &HostValue) -> String {
    value.display_text().unwrap_or_default()
}

/// The debug rendering contract for `{?}` arguments.
#[must_use]
pub fn render_debug(value: &HostValue, items: &[ItemDef]) -> String {
    value.debug_text(items)
}
