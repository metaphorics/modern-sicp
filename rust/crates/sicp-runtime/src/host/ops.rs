// SPDX-License-Identifier: GPL-3.0-only

//! The shared execution core of the teaching engines (grammar §1 and
//! §3): frames and places in the arena, checked arithmetic and its
//! traps, the closed constructor and method operations, iterator
//! stepping, atomic pattern binding, and the format renderings. Every
//! engine — direct, analyzed, explicit-control, compiled — runs these
//! leaves so observable values, effect order, and traps cannot drift.

use std::collections::HashMap;

use crate::host::diag::Span;
use crate::host::hir::{
    BinOp, BindId, CaptureMode, ClosureKind, CtorOp, FormatSpec, FunId, HirBlock, HirPat,
    HirPatKind, HostTy, MethodOp, Resolved, Sema, UnOp,
};
use crate::host::value::{Addr, ClosureVal, Effects, HostValue, IterVal, RtProj, Store, Trap};

/// The control outcome one evaluation step produces. Control flow is
/// the engine's own business; the leaves share this vocabulary.
#[derive(Debug, Clone)]
pub enum Flow {
    /// An ordinary value.
    Value(HostValue),
    /// A `return` unwind carrying its value.
    Return(HostValue),
    /// A `break` unwind carrying its typed value.
    Break(HostValue),
    /// A `continue` unwind.
    Continue,
}

/// One active function or closure activation.
#[derive(Debug, Clone)]
pub struct Activation {
    /// The activation's arena frame.
    pub frame: usize,
    /// The first binding slot the activation owns.
    pub bind_base: u32,
    /// The number of slots the activation owns.
    pub slots: u32,
    /// The active closure's captures, when the activation belongs to
    /// a closure.
    pub captures: Vec<(BindId, CaptureMode, HostValue)>,
}

/// The engine state every walker drives: the arena, the ordered
/// effects, and the activation stack.
#[derive(Debug)]
pub struct Engine {
    /// The typed program being executed.
    pub sema: Sema,
    /// The frame arena.
    pub store: Store,
    /// The ordered effect log.
    pub effects: Effects,
    /// The activation stack, innermost last.
    pub activations: Vec<Activation>,
}

impl Engine {
    /// Builds an engine over one checked program.
    #[must_use]
    pub fn new(sema: Sema) -> Self {
        Self {
            sema,
            store: Store::default(),
            effects: Effects::default(),
            activations: Vec::new(),
        }
    }

    /// Pushes one activation and writes its arguments.
    pub fn push_activation(
        &mut self,
        bind_base: u32,
        slots: u32,
        captures: Vec<(BindId, CaptureMode, HostValue)>,
    ) -> usize {
        let frame = self.store.push_frame(bind_base, slots);
        self.activations.push(Activation {
            frame,
            bind_base,
            slots,
            captures,
        });
        frame
    }

    /// Pops the innermost activation.
    pub fn pop_activation(&mut self) {
        self.activations.pop();
    }

    /// The innermost activation.
    ///
    /// # Panics
    /// Panics if no activation is active.
    #[must_use]
    pub fn current(&self) -> &Activation {
        self.activations.last().expect("an activation is active")
    }

    /// Whether a binding belongs to the innermost activation.
    fn owns(&self, bind: BindId) -> bool {
        let activation = self.current();
        bind.0 >= activation.bind_base && bind.0 < activation.bind_base + activation.slots
    }

    /// The place one local binding names in the innermost activation:
    /// its own slot, or, for a captured binding, the place its capture
    /// record refers to. Every capture record holds a reference: a
    /// borrowing capture refers to the captured slot itself, and an
    /// owning capture to the closure's own storage cell, so reads,
    /// writes, and borrows through a capture all address one place and
    /// an `FnMut` closure's state persists across its calls.
    ///
    /// # Errors
    /// [`Trap::Dangling`] when the binding is neither local nor
    /// captured.
    pub fn local_place(&self, bind: BindId) -> Result<(Addr, Vec<RtProj>), Trap> {
        if self.owns(bind) {
            let frame = self.current().frame;
            return Ok((Addr { frame, bind }, Vec::new()));
        }
        let record = self
            .current()
            .captures
            .iter()
            .rev()
            .find(|(captured, _, _)| *captured == bind)
            .ok_or(Trap::Dangling)?;
        let (addr, projs, _) = Self::ref_target(&record.2)?;
        Ok((addr, projs))
    }

    /// Writes one local binding of the innermost activation, routing
    /// captured bindings through their capture record.
    ///
    /// # Errors
    /// [`Trap::Dangling`] for a slot outside the arena.
    pub fn write_local(&mut self, bind: BindId, value: HostValue) -> Result<(), Trap> {
        let (addr, projs) = self.local_place(bind)?;
        self.write_at(addr, &projs, value)
    }

    /// The capture record value for one binding at closure-creation
    /// time: a reference to the captured place for a borrowing
    /// capture, or, for an owning capture, a reference to a fresh
    /// storage cell the value is copied or moved into.
    ///
    /// # Errors
    /// [`Trap::Dangling`] when the binding belongs to no active scope,
    /// or [`Trap::UseAfterMove`] for a moved value.
    pub fn capture_value(&mut self, bind: BindId, mode: CaptureMode) -> Result<HostValue, Trap> {
        let (addr, projs) = self.local_place(bind)?;
        let mutable = match mode {
            CaptureMode::Shared => false,
            CaptureMode::Mut => true,
            CaptureMode::Owned => {
                let copy = self
                    .sema
                    .bindings
                    .get(bind.0 as usize)
                    .is_some_and(|info| info.ty.is_copy());
                let value = if copy {
                    self.read_at(addr, &projs)?
                } else {
                    self.take_at(addr, &projs)?
                };
                let cell = Addr {
                    frame: self.store.push_frame(bind.0, 1),
                    bind,
                };
                self.store.write(cell, value)?;
                return Ok(HostValue::Ref {
                    addr: cell,
                    projs: Vec::new(),
                    mutable: true,
                });
            }
        };
        Ok(HostValue::Ref {
            addr,
            projs,
            mutable,
        })
    }

    /// Reads one local binding, following closure captures.
    ///
    /// # Errors
    /// [`Trap::UseAfterMove`] for a moved slot, [`Trap::Dangling`]
    /// outside the arena.
    pub fn read_local(&self, bind: BindId) -> Result<HostValue, Trap> {
        let (addr, projs) = self.local_place(bind)?;
        self.read_at(addr, &projs)
    }

    /// Moves one local binding out, following closure captures.
    ///
    /// # Errors
    /// The same conditions as [`Engine::read_local`].
    pub fn take_local(&mut self, bind: BindId) -> Result<HostValue, Trap> {
        let (addr, projs) = self.local_place(bind)?;
        self.take_at(addr, &projs)
    }

    /// The referent of one reference value.
    ///
    /// # Errors
    /// [`Trap::Dangling`] when the value is not a reference.
    pub fn ref_target(value: &HostValue) -> Result<(Addr, Vec<RtProj>, bool), Trap> {
        match value {
            HostValue::Ref {
                addr,
                projs,
                mutable,
            } => Ok((*addr, projs.clone(), *mutable)),
            _ => Err(Trap::Dangling),
        }
    }

    /// Reads through one reference value.
    ///
    /// # Errors
    /// The same conditions as [`Engine::read_local`].
    pub fn deref_value(&self, value: &HostValue) -> Result<HostValue, Trap> {
        let (addr, projs, _) = Self::ref_target(value)?;
        self.read_at(addr, &projs)
    }

    /// Reads the value one place addresses.
    ///
    /// # Errors
    /// [`Trap::UseAfterMove`], [`Trap::IndexOutOfBounds`], or
    /// [`Trap::Dangling`].
    pub fn read_at(&self, addr: Addr, projs: &[RtProj]) -> Result<HostValue, Trap> {
        let value = self.store.read(addr)?;
        project_read(&value, projs)
    }

    /// Moves out of the place one place addresses.
    ///
    /// # Errors
    /// The same conditions as [`Engine::read_at`].
    pub fn take_at(&mut self, addr: Addr, projs: &[RtProj]) -> Result<HostValue, Trap> {
        if projs.is_empty() {
            return self.store.take(addr);
        }
        let mut value = self.store.take(addr)?;
        let taken = project_take(&mut value, projs)?;
        self.store.write(addr, value)?;
        Ok(taken)
    }

    /// Writes the place one place addresses.
    ///
    /// # Errors
    /// [`Trap::IndexOutOfBounds`] or [`Trap::Dangling`].
    pub fn write_at(&mut self, addr: Addr, projs: &[RtProj], value: HostValue) -> Result<(), Trap> {
        if projs.is_empty() {
            return self.store.write(addr, value);
        }
        let mut root = self.store.read(addr)?;
        project_write(&mut root, projs, value)?;
        self.store.write(addr, root)
    }
}

fn project_read(value: &HostValue, projs: &[RtProj]) -> Result<HostValue, Trap> {
    project_ref(value, projs).cloned()
}

fn project_ref<'a>(value: &'a HostValue, projs: &[RtProj]) -> Result<&'a HostValue, Trap> {
    let Some((head, rest)) = projs.split_first() else {
        return Ok(value);
    };
    match (head, value) {
        (RtProj::Field(index), HostValue::Struct(_, fields) | HostValue::Variant(_, _, fields)) => {
            let field = fields.get(*index as usize).ok_or(Trap::IndexOutOfBounds)?;
            project_ref(field, rest)
        }
        (RtProj::Field(0), HostValue::Tuple(left, _)) => project_ref(left, rest),
        (RtProj::Field(1), HostValue::Tuple(_, right)) => project_ref(right, rest),
        (RtProj::Index(index), HostValue::Vec(items) | HostValue::Array(items)) => {
            project_ref(index_item(items, *index)?, rest)
        }
        (RtProj::BoxDeref, HostValue::Box(inner)) => project_ref(inner, rest),
        (RtProj::MapKey(key), HostValue::Map(map)) => {
            let entry = map.get(key).ok_or(Trap::Dangling)?;
            project_ref(entry, rest)
        }
        _ => Err(Trap::Dangling),
    }
}

fn project_take(value: &mut HostValue, projs: &[RtProj]) -> Result<HostValue, Trap> {
    let Some((head, rest)) = projs.split_first() else {
        return Ok(std::mem::replace(value, HostValue::Unit));
    };
    match (head, value) {
        (RtProj::Field(index), HostValue::Struct(_, fields) | HostValue::Variant(_, _, fields)) => {
            let field = fields
                .get_mut(*index as usize)
                .ok_or(Trap::IndexOutOfBounds)?;
            project_take(field, rest)
        }
        (RtProj::Field(0), HostValue::Tuple(left, _)) => project_take(left, rest),
        (RtProj::Field(1), HostValue::Tuple(_, right)) => project_take(right, rest),
        (RtProj::Index(index), HostValue::Vec(items) | HostValue::Array(items)) => {
            let index = usize::try_from(*index).map_err(|_| Trap::IndexOutOfBounds)?;
            let item = items.get_mut(index).ok_or(Trap::IndexOutOfBounds)?;
            project_take(item, rest)
        }
        (RtProj::BoxDeref, HostValue::Box(inner)) => project_take(inner, rest),
        (RtProj::MapKey(key), HostValue::Map(map)) => {
            let entry = map.get_mut(key).ok_or(Trap::Dangling)?;
            project_take(entry, rest)
        }
        _ => Err(Trap::Dangling),
    }
}

fn project_write(value: &mut HostValue, projs: &[RtProj], new: HostValue) -> Result<(), Trap> {
    let Some((head, rest)) = projs.split_first() else {
        *value = new;
        return Ok(());
    };
    match (head, value) {
        (RtProj::Field(index), HostValue::Struct(_, fields) | HostValue::Variant(_, _, fields)) => {
            let field = fields
                .get_mut(*index as usize)
                .ok_or(Trap::IndexOutOfBounds)?;
            project_write(field, rest, new)
        }
        (RtProj::Field(0), HostValue::Tuple(left, _)) => project_write(left, rest, new),
        (RtProj::Field(1), HostValue::Tuple(_, right)) => project_write(right, rest, new),
        (RtProj::Index(index), HostValue::Vec(items) | HostValue::Array(items)) => {
            let index = usize::try_from(*index).map_err(|_| Trap::IndexOutOfBounds)?;
            let item = items.get_mut(index).ok_or(Trap::IndexOutOfBounds)?;
            project_write(item, rest, new)
        }
        (RtProj::BoxDeref, HostValue::Box(inner)) => project_write(inner, rest, new),
        (RtProj::MapKey(key), HostValue::Map(map)) => {
            let entry = map.entry(key.clone()).or_insert(HostValue::Unit);
            project_write(entry, rest, new)
        }
        _ => Err(Trap::Dangling),
    }
}

fn index_item(items: &[HostValue], index: i64) -> Result<&HostValue, Trap> {
    let index = usize::try_from(index).map_err(|_| Trap::IndexOutOfBounds)?;
    items.get(index).ok_or(Trap::IndexOutOfBounds)
}

/// The checked-build arithmetic of grammar §3: every operation traps
/// on overflow, division or remainder by zero.
///
/// # Errors
/// [`Trap::Overflow`] or [`Trap::DivByZero`].
pub fn checked_binary(op: BinOp, left: &HostValue, right: &HostValue) -> Result<HostValue, Trap> {
    if op == BinOp::Eq {
        return Ok(HostValue::Bool(values_equal(left, right)));
    }
    if op == BinOp::Ne {
        return Ok(HostValue::Bool(!values_equal(left, right)));
    }
    if let (HostValue::Usize(a), HostValue::Usize(b)) = (left, right) {
        return usize_binary(op, *a, *b);
    }
    if let (HostValue::Text(a), HostValue::Text(b)) = (left, right) {
        return Ok(HostValue::Bool(match op {
            BinOp::Lt => a < b,
            BinOp::Le => a <= b,
            BinOp::Gt => a > b,
            BinOp::Ge => a >= b,
            _ => return Err(Trap::Dangling),
        }));
    }
    let (HostValue::Int(a), HostValue::Int(b)) = (left, right) else {
        return Err(Trap::Dangling);
    };
    let (a, b) = (*a, *b);
    let result = match op {
        // Eq/Ne returned at the entry; arms kept only for exhaustiveness.
        BinOp::Eq | BinOp::Ne | BinOp::And | BinOp::Or => return Err(Trap::Dangling),
        BinOp::Add => a.checked_add(b).ok_or(Trap::Overflow("add"))?,
        BinOp::Sub => a.checked_sub(b).ok_or(Trap::Overflow("subtract"))?,
        BinOp::Mul => a.checked_mul(b).ok_or(Trap::Overflow("multiply"))?,
        BinOp::Div => {
            if b == 0 {
                return Err(Trap::DivByZero);
            }
            a.checked_div(b).ok_or(Trap::Overflow("divide"))?
        }
        BinOp::Rem => {
            if b == 0 {
                return Err(Trap::DivByZero);
            }
            a.checked_rem(b).ok_or(Trap::Overflow("remainder"))?
        }
        BinOp::Lt => return Ok(HostValue::Bool(a < b)),
        BinOp::Le => return Ok(HostValue::Bool(a <= b)),
        BinOp::Gt => return Ok(HostValue::Bool(a > b)),
        BinOp::Ge => return Ok(HostValue::Bool(a >= b)),
    };
    Ok(HostValue::Int(result))
}

/// The admitted `usize` arithmetic of grammar §3: `+` and `-` only,
/// with checked-build overflow behavior; every other operator is
/// rejected before effects and traps here if reached.
fn usize_binary(op: BinOp, a: u64, b: u64) -> Result<HostValue, Trap> {
    Ok(match op {
        BinOp::Add => HostValue::Usize(a.checked_add(b).ok_or(Trap::Overflow("add"))?),
        BinOp::Sub => HostValue::Usize(a.checked_sub(b).ok_or(Trap::Overflow("subtract"))?),
        BinOp::Lt => HostValue::Bool(a < b),
        BinOp::Le => HostValue::Bool(a <= b),
        BinOp::Gt => HostValue::Bool(a > b),
        BinOp::Ge => HostValue::Bool(a >= b),
        _ => return Err(Trap::Dangling),
    })
}

/// Equality on the admitted `PartialEq` forms.
#[must_use]
pub fn values_equal(left: &HostValue, right: &HostValue) -> bool {
    left == right
}

/// The unary operations of grammar §4.
///
/// # Errors
/// [`Trap::Overflow`] for negation overflow.
pub fn checked_unary(op: UnOp, value: &HostValue) -> Result<HostValue, Trap> {
    match (op, value) {
        (UnOp::Neg, HostValue::Int(a)) => Ok(HostValue::Int(
            a.checked_neg().ok_or(Trap::Overflow("negate"))?,
        )),
        (UnOp::Not, HostValue::Bool(a)) => Ok(HostValue::Bool(!a)),
        _ => Err(Trap::Dangling),
    }
}

/// Constructs one admitted constructor's value.
///
/// # Errors
/// [`Trap::Dangling`] for a malformed call shape.
pub fn construct(op: CtorOp, args: &[HostValue]) -> Result<HostValue, Trap> {
    match (op, args) {
        (CtorOp::StringFrom, [HostValue::Text(text)]) => Ok(HostValue::Text(text.clone())),
        (CtorOp::VecNew, []) | (CtorOp::VecWithCapacity, [HostValue::Int(_)]) => {
            Ok(HostValue::Vec(Vec::new()))
        }
        (CtorOp::MapNew, []) => Ok(HostValue::Map(HashMap::new())),
        (CtorOp::BoxNew, [value]) => Ok(HostValue::Box(Box::new(value.clone()))),
        (CtorOp::OptSome | CtorOp::ResOk, [value]) => {
            Ok(HostValue::Variant(BUILTIN, 0, vec![value.clone()]))
        }
        (CtorOp::OptNone, []) => Ok(HostValue::Variant(BUILTIN, 1, Vec::new())),
        (CtorOp::ResErr, [value]) => Ok(HostValue::Variant(BUILTIN, 1, vec![value.clone()])),
        _ => Err(Trap::Dangling),
    }
}

/// The element position an index value names: indexes are `usize` in
/// the subset, and the arena keeps positions as `i64`.
///
/// # Errors
/// [`Trap::IndexOutOfBounds`] for a position past `i64`, or
/// [`Trap::Dangling`] for a non-integer, which the checker excludes.
pub fn index_position(value: &HostValue) -> Result<i64, Trap> {
    match value {
        HostValue::Int(at) => Ok(*at),
        HostValue::Usize(at) => i64::try_from(*at).map_err(|_| Trap::IndexOutOfBounds),
        _ => Err(Trap::Dangling),
    }
}

/// The `Option` and `Result` discriminants `construct` produces.
#[must_use]
pub fn builtin_variant(value: &HostValue) -> Option<(u32, &Vec<HostValue>)> {
    match value {
        HostValue::Variant(BUILTIN, index, payload) => Some((*index, payload)),
        _ => None,
    }
}

/// The reserved item marker the `Option` and `Result` values carry, so
/// a user enum can never collide with them.
pub const BUILTIN: u32 = u32::MAX;

/// Runs one admitted method for its value and effects. The second
/// result is the updated receiver when the method mutates it; the
/// caller writes it back to the receiver's place.
///
/// # Errors
/// [`Trap::IndexOutOfBounds`], [`Trap::Dangling`], or the trap a
/// nested operation raises.
pub fn apply_method(
    op: MethodOp,
    receiver: HostValue,
    receiver_place: Option<(Addr, Vec<RtProj>)>,
    args: &[HostValue],
) -> Result<(HostValue, Option<HostValue>), Trap> {
    match op {
        MethodOp::StrPushStr | MethodOp::AsStr | MethodOp::StrLen => {
            apply_string_method(op, receiver, args)
        }
        MethodOp::VecPush
        | MethodOp::VecPop
        | MethodOp::VecLen
        | MethodOp::VecIsEmpty
        | MethodOp::VecGet
        | MethodOp::VecGetMut => apply_vec_method(op, receiver, receiver_place, args),
        MethodOp::MapInsert
        | MethodOp::MapGet
        | MethodOp::MapGetMut
        | MethodOp::MapContainsKey
        | MethodOp::MapRemove
        | MethodOp::MapLen
        | MethodOp::MapIsEmpty => apply_map_method(op, receiver, receiver_place, args),
        MethodOp::Iter
        | MethodOp::IterMut
        | MethodOp::IntoIter
        | MethodOp::Next
        | MethodOp::Enumerate
        | MethodOp::Zip => apply_iterator_method(op, receiver, receiver_place, args),
        MethodOp::BoxAsRef | MethodOp::BoxAsMut => {
            let (addr, mut projs) = receiver_place.ok_or(Trap::Dangling)?;
            projs.push(RtProj::BoxDeref);
            Ok((
                HostValue::Ref {
                    addr,
                    projs,
                    mutable: op == MethodOp::BoxAsMut,
                },
                Some(receiver),
            ))
        }
        MethodOp::Clone => {
            let cloned = receiver.clone();
            Ok((cloned, Some(receiver)))
        }
    }
}

fn apply_string_method(
    op: MethodOp,
    receiver: HostValue,
    args: &[HostValue],
) -> Result<(HostValue, Option<HostValue>), Trap> {
    match op {
        MethodOp::StrPushStr => {
            let (HostValue::Text(text), [HostValue::Text(extra)]) = (&receiver, args) else {
                return Err(Trap::Dangling);
            };
            let mut text = text.clone();
            text.push_str(extra);
            Ok((HostValue::Unit, Some(HostValue::Text(text))))
        }
        MethodOp::AsStr => {
            let HostValue::Text(text) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Text(text.clone()), Some(receiver)))
        }
        MethodOp::StrLen => {
            let HostValue::Text(text) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Usize(len_of(text.len())?), Some(receiver)))
        }
        _ => Err(Trap::Dangling),
    }
}

fn apply_vec_method(
    op: MethodOp,
    receiver: HostValue,
    receiver_place: Option<(Addr, Vec<RtProj>)>,
    args: &[HostValue],
) -> Result<(HostValue, Option<HostValue>), Trap> {
    match op {
        MethodOp::VecPush => {
            let (HostValue::Vec(items), [value]) = (&receiver, args) else {
                return Err(Trap::Dangling);
            };
            let mut items = items.clone();
            items.push(value.clone());
            Ok((HostValue::Unit, Some(HostValue::Vec(items))))
        }
        MethodOp::VecPop => {
            let HostValue::Vec(items) = &receiver else {
                return Err(Trap::Dangling);
            };
            let mut items = items.clone();
            let popped = items.pop();
            Ok((option_value(popped), Some(HostValue::Vec(items))))
        }
        MethodOp::VecLen => {
            let HostValue::Vec(items) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Usize(len_of(items.len())?), Some(receiver)))
        }
        MethodOp::VecIsEmpty => {
            let HostValue::Vec(items) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Bool(items.is_empty()), Some(receiver)))
        }
        MethodOp::VecGet | MethodOp::VecGetMut => {
            let HostValue::Vec(items) = &receiver else {
                return Err(Trap::Dangling);
            };
            let [HostValue::Usize(index)] = args else {
                return Err(Trap::Dangling);
            };
            let count = items.len();
            let index = usize::try_from(*index).map_err(|_| Trap::IndexOutOfBounds)?;
            let (addr, projs) = receiver_place.ok_or(Trap::Dangling)?;
            let found = if index < count {
                let mut projs = projs;
                projs.push(RtProj::Index(
                    i64::try_from(index).map_err(|_| Trap::IndexOutOfBounds)?,
                ));
                Some(HostValue::Ref {
                    addr,
                    projs,
                    mutable: op == MethodOp::VecGetMut,
                })
            } else {
                None
            };
            Ok((option_value(found), Some(receiver)))
        }
        _ => Err(Trap::Dangling),
    }
}

fn apply_map_method(
    op: MethodOp,
    receiver: HostValue,
    receiver_place: Option<(Addr, Vec<RtProj>)>,
    args: &[HostValue],
) -> Result<(HostValue, Option<HostValue>), Trap> {
    match op {
        MethodOp::MapInsert => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            let [HostValue::Text(key), value] = args else {
                return Err(Trap::Dangling);
            };
            let mut map = map.clone();
            let previous = map.insert(key.clone(), value.clone());
            Ok((option_value(previous), Some(HostValue::Map(map))))
        }
        MethodOp::MapGet | MethodOp::MapGetMut => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            let [HostValue::Text(key)] = args else {
                return Err(Trap::Dangling);
            };
            let (addr, projs) = receiver_place.ok_or(Trap::Dangling)?;
            let found = map.contains_key(key).then(|| {
                let mut projs = projs;
                // The reference addresses the keyed entry's value
                // slot: reads see the entry, writes update it.
                projs.push(RtProj::MapKey(key.clone()));
                HostValue::Ref {
                    addr,
                    projs,
                    mutable: op == MethodOp::MapGetMut,
                }
            });
            Ok((option_value(found), Some(receiver)))
        }
        MethodOp::MapContainsKey => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            let [HostValue::Text(key)] = args else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Bool(map.contains_key(key)), Some(receiver)))
        }
        MethodOp::MapRemove => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            let [HostValue::Text(key)] = args else {
                return Err(Trap::Dangling);
            };
            let mut map = map.clone();
            let removed = map.remove(key);
            Ok((option_value(removed), Some(HostValue::Map(map))))
        }
        MethodOp::MapLen => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Usize(len_of(map.len())?), Some(receiver)))
        }
        MethodOp::MapIsEmpty => {
            let HostValue::Map(map) = &receiver else {
                return Err(Trap::Dangling);
            };
            Ok((HostValue::Bool(map.is_empty()), Some(receiver)))
        }
        _ => Err(Trap::Dangling),
    }
}

fn apply_iterator_method(
    op: MethodOp,
    receiver: HostValue,
    receiver_place: Option<(Addr, Vec<RtProj>)>,
    args: &[HostValue],
) -> Result<(HostValue, Option<HostValue>), Trap> {
    match op {
        MethodOp::Iter | MethodOp::IterMut => {
            let len = collection_len(&receiver)?;
            let (addr, base) = receiver_place.ok_or(Trap::Dangling)?;
            let iterator = refs_iterator(addr, base, len, op == MethodOp::IterMut);
            Ok((iterator, Some(receiver)))
        }
        MethodOp::IntoIter => {
            let (HostValue::Vec(items) | HostValue::Array(items)) = receiver else {
                return Err(Trap::Dangling);
            };
            Ok((items_iterator(items), None))
        }
        MethodOp::Next => {
            let mut value = receiver;
            let step = iterator_next(&mut value)?;
            Ok((step, Some(value)))
        }
        MethodOp::Enumerate => {
            let wrapped = iterator_enumerate(receiver);
            Ok((wrapped.clone(), Some(wrapped)))
        }
        MethodOp::Zip => {
            let [right] = args else {
                return Err(Trap::Dangling);
            };
            let wrapped = iterator_zip(receiver, right.clone());
            Ok((wrapped.clone(), Some(wrapped)))
        }
        _ => Err(Trap::Dangling),
    }
}

fn len_of(count: usize) -> Result<u64, Trap> {
    u64::try_from(count).map_err(|_| Trap::Overflow("len"))
}

fn collection_len(value: &HostValue) -> Result<usize, Trap> {
    match value {
        HostValue::Vec(items) | HostValue::Array(items) => Ok(items.len()),
        HostValue::Map(map) => Ok(map.len()),
        _ => Err(Trap::Dangling),
    }
}

/// Steps one iterator to its next item.
///
/// # Errors
/// [`Trap::Dangling`] for a non-iterator value.
pub fn iter_next(iterator: &mut IterVal) -> Result<HostValue, Trap> {
    match iterator {
        IterVal::Range { cur, end } => {
            if *cur >= *end {
                return Ok(none_value());
            }
            let value = *cur;
            *cur += 1;
            Ok(some_value(HostValue::Int(value)))
        }
        IterVal::RangeU { cur, end } => {
            if *cur >= *end {
                return Ok(none_value());
            }
            let value = *cur;
            *cur += 1;
            Ok(some_value(HostValue::Usize(value)))
        }
        IterVal::Items { items, pos } => {
            let Some(value) = items.get(*pos).cloned() else {
                return Ok(none_value());
            };
            *pos += 1;
            Ok(some_value(value))
        }
        IterVal::Refs {
            addr,
            base,
            len,
            pos,
            mutable,
        } => {
            if *pos >= *len {
                return Ok(none_value());
            }
            let mut projs = base.clone();
            projs.push(RtProj::Index(
                i64::try_from(*pos).map_err(|_| Trap::IndexOutOfBounds)?,
            ));
            let value = HostValue::Ref {
                addr: *addr,
                projs,
                mutable: *mutable,
            };
            *pos += 1;
            Ok(some_value(value))
        }
        IterVal::Enumerate { inner, pos } => {
            let step = iter_next(inner)?;
            let Some(payload) = payload_of(step) else {
                return Ok(none_value());
            };
            let index = *pos;
            *pos += 1;
            Ok(some_value(HostValue::Tuple(
                Box::new(HostValue::Usize(
                    u64::try_from(index).map_err(|_| Trap::Overflow("enumerate"))?,
                )),
                Box::new(payload),
            )))
        }
        IterVal::Zip { left, right } => {
            let a = iter_next(left)?;
            let b = iter_next(right)?;
            let (Some(a), Some(b)) = (payload_of(a), payload_of(b)) else {
                return Ok(none_value());
            };
            Ok(some_value(HostValue::Tuple(Box::new(a), Box::new(b))))
        }
    }
}

fn none_value() -> HostValue {
    HostValue::Variant(BUILTIN, 1, Vec::new())
}

fn some_value(value: HostValue) -> HostValue {
    HostValue::Variant(BUILTIN, 0, vec![value])
}

fn payload_of(value: HostValue) -> Option<HostValue> {
    match value {
        HostValue::Variant(BUILTIN, 0, mut payload) if payload.len() == 1 => {
            Some(payload.remove(0))
        }
        _ => None,
    }
}

/// The `Some` shape of one value.
#[must_use]
pub fn option_value(value: Option<HostValue>) -> HostValue {
    match value {
        Some(inner) => some_value(inner),
        None => none_value(),
    }
}

/// Binds one pattern against one value, answering every binding the
/// match would create without committing any of them. The caller
/// commits only when the whole pattern matched, so a failed arm leaves
/// no half-bound slots.
///
/// A reference scrutinee follows Rust's match ergonomics: a binding
/// or wildcard takes the reference itself, while a structural pattern
/// reads through it and its sub-patterns see references to the
/// referent's parts (the `ref` default binding mode), so a binding
/// under a borrowed scrutinee is a genuine borrow of the matched place.
///
/// # Errors
/// [`Trap::Dangling`] for a shape the checker had already ruled out,
/// or the trap a read through a reference raises.
pub fn bind_pattern(
    engine: &Engine,
    pat: &HirPat,
    value: &HostValue,
) -> Result<Option<Vec<(BindId, HostValue)>>, Trap> {
    let mut bindings = Vec::new();
    if match_pattern(engine, pat, value, &mut bindings)? {
        Ok(Some(bindings))
    } else {
        Ok(None)
    }
}

fn match_pattern(
    engine: &Engine,
    pat: &HirPat,
    value: &HostValue,
    bindings: &mut Vec<(BindId, HostValue)>,
) -> Result<bool, Trap> {
    match (&pat.kind, value) {
        (HirPatKind::Wild, _) => Ok(true),
        (HirPatKind::Bind(binding), _) => {
            bindings.push((*binding, value.clone()));
            Ok(true)
        }
        (
            _,
            HostValue::Ref {
                addr,
                projs,
                mutable,
            },
        ) => match_through_ref(engine, pat, *addr, projs, *mutable, bindings),
        (HirPatKind::I64(expected), _) => {
            Ok(matches!(value, HostValue::Int(found) if *found == *expected))
        }
        (HirPatKind::Usize(expected), _) => Ok(match value {
            HostValue::Usize(found) => *found == *expected,
            HostValue::Int(found) => u64::try_from(*found).ok() == Some(*expected),
            _ => false,
        }),
        (HirPatKind::Bool(expected), _) => {
            Ok(matches!(value, HostValue::Bool(found) if *found == *expected))
        }
        (HirPatKind::Tuple(left, right), _) => {
            let HostValue::Tuple(a, b) = value else {
                return Err(Trap::Dangling);
            };
            match_all(engine, [&**left, &**right], [&**a, &**b], bindings)
        }
        (HirPatKind::UnitPath(resolved), _) => Ok(match_unit_path(resolved, value)),
        (HirPatKind::TuplePath(resolved, subs), _) => {
            let Some(payload) = path_payload(resolved, value)? else {
                return Ok(false);
            };
            if payload.len() != subs.len() {
                return Err(Trap::Dangling);
            }
            match_all(engine, subs, payload, bindings)
        }
        (HirPatKind::StructPath(resolved, fields), _) => {
            let Some(payload) = path_payload(resolved, value)? else {
                return Ok(false);
            };
            let present = fields
                .iter()
                .zip(payload)
                .filter_map(|(sub, item)| sub.as_ref().map(|sub| (sub, item)));
            let (subs, items): (Vec<&HirPat>, Vec<&HostValue>) = present.unzip();
            match_all(engine, subs, items, bindings)
        }
    }
}

/// Matches sub-patterns against their parts left to right; a failed
/// part discards every binding the whole pattern collected.
fn match_all<'p, 'v>(
    engine: &Engine,
    subs: impl IntoIterator<Item = &'p HirPat>,
    items: impl IntoIterator<Item = &'v HostValue>,
    bindings: &mut Vec<(BindId, HostValue)>,
) -> Result<bool, Trap> {
    for (sub, item) in subs.into_iter().zip(items) {
        if !match_pattern(engine, sub, item, bindings)? {
            bindings.clear();
            return Ok(false);
        }
    }
    Ok(true)
}

/// Matches a structural pattern through one reference: the referent's
/// shape decides the match, and each part is offered to its sub-pattern
/// as a reference to that part's place.
fn match_through_ref(
    engine: &Engine,
    pat: &HirPat,
    addr: Addr,
    projs: &[RtProj],
    mutable: bool,
    bindings: &mut Vec<(BindId, HostValue)>,
) -> Result<bool, Trap> {
    let referent = engine.read_at(addr, projs)?;
    let part_count = match (&pat.kind, &referent) {
        // `&&T` reads through every layer before the shape is tested.
        (_, HostValue::Ref { .. }) => return match_pattern(engine, pat, &referent, bindings),
        (HirPatKind::Tuple(..), HostValue::Tuple(..)) => 2,
        (HirPatKind::TuplePath(resolved, _) | HirPatKind::StructPath(resolved, _), _) => {
            match path_payload(resolved, &referent)? {
                Some(payload) => payload.len(),
                None => return Ok(false),
            }
        }
        // Literal and unit patterns test the referent's value directly.
        _ => return match_pattern(engine, pat, &referent, bindings),
    };
    let parts: Vec<HostValue> = (0..part_count)
        .map(|index| {
            let mut part_projections = projs.to_vec();
            part_projections.push(RtProj::Field(
                u32::try_from(index).expect("payloads are small"),
            ));
            HostValue::Ref {
                addr,
                projs: part_projections,
                mutable,
            }
        })
        .collect();
    match &pat.kind {
        HirPatKind::Tuple(left, right) => match_all(engine, [&**left, &**right], &parts, bindings),
        HirPatKind::TuplePath(_, subs) => {
            if parts.len() != subs.len() {
                return Err(Trap::Dangling);
            }
            match_all(engine, subs, &parts, bindings)
        }
        HirPatKind::StructPath(_, fields) => {
            let present = fields
                .iter()
                .zip(&parts)
                .filter_map(|(sub, item)| sub.as_ref().map(|sub| (sub, item)));
            let (subs, items): (Vec<&HirPat>, Vec<&HostValue>) = present.unzip();
            match_all(engine, subs, items, bindings)
        }
        _ => Err(Trap::Dangling),
    }
}

fn match_unit_path(resolved: &Resolved, value: &HostValue) -> bool {
    match resolved {
        Resolved::Ctor(CtorOp::OptNone) => {
            builtin_variant(value).is_some_and(|(index, payload)| index == 1 && payload.is_empty())
        }
        Resolved::Variant(id, index) => matches!(
            value,
            HostValue::Variant(found, at, payload)
                if *found == id.0 && *at == *index && payload.is_empty()
        ),
        Resolved::UnitStruct(id) => matches!(
            value,
            HostValue::Struct(found, fields) if *found == id.0 && fields.is_empty()
        ),
        Resolved::TupleStruct(id) => matches!(
            value,
            HostValue::Struct(found, _) if *found == id.0
        ),
        _ => false,
    }
}

/// The payload a constructor pattern destructures: `None` when the
/// value is another variant of the same type (the arm fails), and a
/// trap only for a value of the wrong type, which the checker excludes.
fn path_payload<'a>(
    resolved: &Resolved,
    value: &'a HostValue,
) -> Result<Option<&'a Vec<HostValue>>, Trap> {
    match (resolved, value) {
        (Resolved::Ctor(op), _) => {
            let index = match op {
                CtorOp::OptSome | CtorOp::ResOk => 0,
                CtorOp::ResErr => 1,
                _ => return Err(Trap::Dangling),
            };
            let (found, payload) = builtin_variant(value).ok_or(Trap::Dangling)?;
            Ok((found == index).then_some(payload))
        }
        (Resolved::Variant(id, index), HostValue::Variant(found, at, payload))
            if *found == id.0 =>
        {
            Ok((*at == *index).then_some(payload))
        }
        (
            Resolved::TupleStruct(id) | Resolved::UnitStruct(id),
            HostValue::Struct(found, fields),
        ) if *found == id.0 => Ok(Some(fields)),
        _ => Err(Trap::Dangling),
    }
}

/// Wraps one iterator value in `enumerate`.
#[must_use]
pub fn iterator_enumerate(value: HostValue) -> HostValue {
    let HostValue::Iter(inner) = value else {
        return value;
    };
    HostValue::Iter(Box::new(IterVal::Enumerate { inner, pos: 0 }))
}

/// Zips two iterator values.
#[must_use]
pub fn iterator_zip(left: HostValue, right: HostValue) -> HostValue {
    let (HostValue::Iter(left), HostValue::Iter(right)) = (left, right) else {
        return HostValue::Iter(Box::new(IterVal::Items {
            items: Vec::new(),
            pos: 0,
        }));
    };
    HostValue::Iter(Box::new(IterVal::Zip { left, right }))
}

/// Steps one iterator value held in a place, returning its next item.
///
/// # Errors
/// [`Trap::Dangling`] when the value is not an iterator.
pub fn iterator_next(value: &mut HostValue) -> Result<HostValue, Trap> {
    match value {
        HostValue::Iter(inner) => iter_next(inner),
        _ => Err(Trap::Dangling),
    }
}

/// Builds the by-value iterator over owned elements.
#[must_use]
pub fn items_iterator(items: Vec<HostValue>) -> HostValue {
    HostValue::Iter(Box::new(IterVal::Items { items, pos: 0 }))
}

/// Builds the element-reference iterator over one collection place.
#[must_use]
pub fn refs_iterator(addr: Addr, base: Vec<RtProj>, len: usize, mutable: bool) -> HostValue {
    HostValue::Iter(Box::new(IterVal::Refs {
        addr,
        base,
        len,
        pos: 0,
        mutable,
    }))
}

/// Builds the half-open `i64` range iterator.
#[must_use]
pub fn range_iterator(cur: i64, end: i64) -> HostValue {
    HostValue::Iter(Box::new(IterVal::Range { cur, end }))
}

/// Builds the half-open `usize` range iterator.
#[must_use]
pub fn range_iterator_u(cur: u64, end: u64) -> HostValue {
    HostValue::Iter(Box::new(IterVal::RangeU { cur, end }))
}

/// Builds the half-open range iterator matching the operands' static
/// integer type: `i64` pairs and `usize` pairs never mix (grammar §3).
///
/// # Errors
/// [`Trap::Dangling`] when the operands are not one admitted integer
/// type each.
pub fn range_of(start: &HostValue, end: &HostValue) -> Result<HostValue, Trap> {
    match (start, end) {
        (HostValue::Int(a), HostValue::Int(b)) => Ok(range_iterator(*a, *b)),
        (HostValue::Usize(a), HostValue::Usize(b)) => Ok(range_iterator_u(*a, *b)),
        // Grammar section 3 never mixes `i64` and `usize` pairs: any
        // other combination is a typed error, not a range.
        (_, _) => Err(Trap::Dangling),
    }
}

/// The ordered rendering of one format call (grammar §4).
///
/// # Errors
/// A formatted reference that names a moved or dangling value traps.
pub fn render_format(spec: &FormatSpec, args: &[HostValue], store: &Store) -> Result<String, Trap> {
    let mut out = String::new();
    for (index, piece) in spec.pieces.iter().enumerate() {
        out.push_str(piece);
        if let Some(is_debug) = spec.debug.get(index)
            && let Some(arg) = args.get(index)
        {
            out.push_str(&render_format_argument(store, arg, *is_debug)?);
        }
    }
    Ok(out)
}

fn render_format_argument(
    store: &Store,
    value: &HostValue,
    is_debug: bool,
) -> Result<String, Trap> {
    if let HostValue::Ref { addr, projs, .. } = value {
        let referent = project_ref(store.read_ref(*addr)?, projs)?;
        return render_format_argument(store, referent, is_debug);
    }
    if is_debug {
        Ok(value.debug_text())
    } else {
        value.display_text()
    }
}

/// Builds a closure value from its checked record.
#[must_use]
pub fn closure_value(
    kind: ClosureKind,
    body: std::sync::Arc<HirBlock>,
    params: Vec<(BindId, HostTy)>,
    captures: Vec<(BindId, CaptureMode, HostValue)>,
    ret: HostTy,
    frame_slots: u32,
    bind_base: u32,
) -> HostValue {
    HostValue::Closure(std::sync::Arc::new(ClosureVal {
        kind,
        params,
        body,
        captures,
        ret,
        frame_slots,
        bind_base,
        compiled_entry: None,
    }))
}

/// Builds one closure value whose application jumps to a compiled
/// entry label: the body syntax is never interpreted.
#[must_use]
pub fn compiled_closure_value(
    kind: crate::host::hir::ClosureKind,
    params: Vec<(crate::host::hir::BindId, crate::host::hir::HostTy)>,
    captures: Vec<(
        crate::host::hir::BindId,
        crate::host::hir::CaptureMode,
        HostValue,
    )>,
    frame_slots: u32,
    bind_base: u32,
    entry: String,
) -> HostValue {
    HostValue::Closure(std::sync::Arc::new(ClosureVal {
        kind,
        params,
        body: std::sync::Arc::new(crate::host::hir::HirBlock {
            stmts: Vec::new(),
            tail: None,
        }),
        captures,
        ret: crate::host::hir::HostTy::Unit,
        frame_slots,
        bind_base,
        compiled_entry: Some(entry),
    }))
}

/// The trap report one run carries out of the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrapReport {
    /// The trap that stopped the run.
    pub trap: Trap,
    /// Where the trap occurred.
    pub span: Span,
}

/// The observable outcome of one run.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    /// Everything `print!` and `println!` wrote, in evaluation order.
    pub stdout: String,
    /// The trap that stopped the run, when one did.
    pub trap: Option<TrapReport>,
}

/// The `main` entry of a checked program.
#[must_use]
pub fn main_fun(sema: &Sema) -> FunId {
    sema.main
}
