// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3

//! Section 3.3: Modeling with mutable data.
//!
//! The book's mutable pairs are the runtime's [`ConsCell`] with both
//! fields under `RefCell`, and every structure the section builds —
//! shared chains, queues, tables, wires, connectors — is a graph of `Rc`
//! handles over mutable cells. Sharing is pointer identity
//! (`Rc::ptr_eq`), so the section's aliasing questions become questions
//! about which names hold clones of one handle, and the types make the
//! sharing visible.

use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::{Rc, Weak};

use sicp_runtime::{Key, Pair, SchemeError, Value, cons_cell, set_cdr};

/// The book's `last-pair`: the last pair of a nonempty chain.
#[must_use]
pub fn last_pair(x: &Pair) -> Pair {
    let mut current = Rc::clone(x);
    loop {
        let next = current.cdr.borrow().clone();
        match next {
            Value::Pair(cell) => current = cell,
            _ => return current,
        }
    }
}

/// The book's `append!`: splices `y` onto the last pair of `x`, so every
/// name for `x` now sees one longer chain. The book's version returns an
/// unspecified value; this one returns the spliced list for convenience.
pub fn append_bang(x: &Pair, y: &Pair) -> Pair {
    let tail = last_pair(x);
    set_cdr(&tail, Value::Pair(Rc::clone(y)));
    Rc::clone(x)
}

/// The book's `make-cycle`: the last pair's `cdr` is set back to `x`
/// itself, so no pair's `cdr` chain ends in `Nil`.
pub fn make_cycle(x: &Pair) {
    set_cdr(&last_pair(x), Value::Pair(Rc::clone(x)));
}

/// The first pair of a `Value` list: the handle the section's mutators
/// work through. A non-list `Value` answers a one-element cell so the
/// callers stay total.
#[must_use]
pub fn first_pair(v: &Value) -> Pair {
    let Value::Pair(pair) = v else {
        return cons_cell(v.clone(), Value::Nil);
    };
    Pair::clone(pair)
}

/// Exercise 3.17's count: one per distinct pair, sharing counted once,
/// driven by a history of visited cells. The history key is the cell
/// address, the Rust `eq?` on pairs.
#[must_use]
pub fn count_pairs_distinct(x: &Value) -> u64 {
    fn walk(x: &Value, seen: &mut HashSet<usize>) -> u64 {
        let Value::Pair(pair) = x else {
            return 0;
        };
        if !seen.insert(Rc::as_ptr(pair).addr()) {
            return 0;
        }
        1 + walk(&pair.car.borrow(), seen) + walk(&pair.cdr.borrow(), seen)
    }
    let mut seen = HashSet::new();
    walk(x, &mut seen)
}

/// Exercise 3.18's test: does the chain starting at `x` contain a cycle?
/// The walk remembers every cell address it has visited.
#[must_use]
pub fn has_cycle(x: &Pair) -> bool {
    let mut seen = HashSet::new();
    let mut cursor = Some(Rc::clone(x));
    while let Some(cell) = cursor {
        if !seen.insert(Rc::as_ptr(&cell).addr()) {
            return true;
        }
        let next = cell.cdr.borrow().clone();
        cursor = match next {
            Value::Pair(next_cell) => Some(next_cell),
            _ => None,
        };
    }
    false
}

/// Exercise 3.19's test: the same question in constant space, with a
/// slow pointer one step at a time and a fast pointer two steps at a
/// time. If the chain is circular, the fast one laps the slow one.
#[must_use]
pub fn has_cycle_constant_space(x: &Pair) -> bool {
    let mut tortoise = Rc::clone(x);
    let mut hare = Rc::clone(x);
    loop {
        let step = hare.cdr.borrow().clone();
        let Value::Pair(next) = step else {
            return false;
        };
        let hop = next.cdr.borrow().clone();
        let Value::Pair(after) = hop else {
            return false;
        };
        hare = after;
        let step = tortoise.cdr.borrow().clone();
        let Value::Pair(step_cell) = step else {
            return false;
        };
        tortoise = step_cell;
        if Rc::ptr_eq(&tortoise, &hare) {
            return true;
        }
    }
}

/// Exercise 3.19a's test: do the two chains share their final cells? A
/// common suffix means one shared pair that both chains reach, so the
/// decision is pointer identity after aligning the two walks.
#[must_use]
pub fn shares_suffix(x: &Pair, y: &Pair) -> bool {
    fn steps_from(x: &Pair) -> u64 {
        let mut count = 0u64;
        let mut cursor = Some(Rc::clone(x));
        while let Some(cell) = cursor {
            count += 1;
            cursor = match cell.cdr.borrow().clone() {
                Value::Pair(next) => Some(next),
                _ => None,
            };
        }
        count
    }
    let ahead = steps_from(x);
    let behind = steps_from(y);
    let mut long = Rc::clone(x);
    let mut short = Rc::clone(y);
    for _ in 0..ahead.saturating_sub(behind) {
        let next = long.cdr.borrow().clone();
        long = match next {
            Value::Pair(next_cell) => next_cell,
            _ => return false,
        };
    }
    for _ in 0..behind.saturating_sub(ahead) {
        let next = short.cdr.borrow().clone();
        short = match next {
            Value::Pair(next_cell) => next_cell,
            _ => return false,
        };
    }
    loop {
        if Rc::ptr_eq(&long, &short) {
            return true;
        }
        let next_long = long.cdr.borrow().clone();
        let next_short = short.cdr.borrow().clone();
        match (next_long, next_short) {
            (Value::Pair(a), Value::Pair(b)) => {
                long = a;
                short = b;
            }
            _ => return false,
        }
    }
}

/// A request to the book's procedural `cons` of the main text: the two
/// selectors, and the two mutators carrying their new value.
#[derive(Clone, Debug, PartialEq)]
pub enum PairRequest {
    /// The book's `'car` message.
    Car,
    /// The book's `'cdr` message.
    Cdr,
    /// The book's `'set-car!` message with the new content.
    SetCar(Value),
    /// The book's `'set-cdr!` message with the new content.
    SetCdr(Value),
}

/// The book's procedural pair: two hidden cells and one dispatch closure
/// over them, exactly the main text's local-state `cons`. Two names for
/// one `ProcPair` alias the same two cells, which is the point of
/// exercise 3.20.
#[derive(Clone)]
pub struct ProcPair {
    dispatch: Rc<dyn Fn(PairRequest) -> Value>,
}

/// Builds the main text's procedural `(cons x y)`: the cells live in the
/// closure's captured state, and nothing other than the returned
/// dispatch can reach them.
#[must_use]
pub fn procedural_cons(x: Value, y: Value) -> ProcPair {
    let car_cell = RefCell::new(x);
    let tail_cell = RefCell::new(y);
    ProcPair {
        dispatch: Rc::new(move |request| match request {
            PairRequest::Car => car_cell.borrow().clone(),
            PairRequest::Cdr => tail_cell.borrow().clone(),
            PairRequest::SetCar(v) => {
                *car_cell.borrow_mut() = v;
                Value::sym("done")
            }
            PairRequest::SetCdr(v) => {
                *tail_cell.borrow_mut() = v;
                Value::sym("done")
            }
        }),
    }
}

impl ProcPair {
    /// Sends one request to the pair. The selectors answer with the
    /// cell's content; the mutators answer with the `done` symbol.
    #[must_use]
    pub fn send(&self, request: PairRequest) -> Value {
        (self.dispatch)(request)
    }
}

/// The book's queue: a front pointer and a rear pointer over mutable
/// pairs, so both ends of the chain are reachable in constant time.
///
/// The items form the usual pair chain; the rear pointer only exists so
/// an insertion never scans. Cloning a `Queue` gives a second name for
/// the same object, as in the book.
#[derive(Debug, Default)]
pub struct Queue {
    front: RefCell<Option<Pair>>,
    rear: RefCell<Option<Pair>>,
}

impl Queue {
    /// The book's `make-queue`: an empty queue, both pointers unset.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `empty-queue?`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.front.borrow().is_none()
    }

    /// The book's `front-queue`: the item at the front.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] with the book's message when the queue
    /// is empty.
    pub fn front(&self) -> Result<Value, SchemeError> {
        let front = self.front.borrow().clone();
        match front {
            Some(pair) => Ok(pair.car.borrow().clone()),
            None => Err(SchemeError::UserRaised {
                message: "front called with an empty queue".into(),
                irritants: vec![],
            }),
        }
    }

    /// The book's `insert-queue!`: one new pair at the rear, in constant
    /// time because the rear pointer spares the scan.
    pub fn insert(&self, item: Value) {
        let new_pair = cons_cell(item, Value::Nil);
        let mut rear = self.rear.borrow_mut();
        match rear.as_ref() {
            Some(rear_pair) => set_cdr(rear_pair, Value::Pair(Rc::clone(&new_pair))),
            None => *self.front.borrow_mut() = Some(Rc::clone(&new_pair)),
        }
        *rear = Some(new_pair);
    }

    /// The book's `delete-queue!`: the front pointer moves to the second
    /// item; the rear pointer needs no update, because `is_empty` looks
    /// only at the front.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] with the book's message when the queue
    /// is empty before the deletion.
    pub fn delete(&self) -> Result<(), SchemeError> {
        let front = self.front.borrow().clone();
        let Some(pair) = front else {
            return Err(SchemeError::UserRaised {
                message: "delete! called with an empty queue".into(),
                irritants: vec![],
            });
        };
        let rest = pair.cdr.borrow().clone();
        *self.front.borrow_mut() = match rest {
            Value::Pair(cell) => Some(cell),
            Value::Nil => None,
            other => {
                return Err(SchemeError::TypeMismatch(format!(
                    "queue node with a non-list tail: {other}"
                )));
            }
        };
        Ok(())
    }

    /// Ben Bitdiddle's `print-queue` of exercise 3.21: the items in
    /// queue order, not the front/rear pointer pair the default printer
    /// would show.
    #[must_use]
    pub fn items(&self) -> Vec<Value> {
        let mut items = Vec::new();
        let mut cursor = self.front.borrow().clone();
        while let Some(pair) = cursor {
            items.push(pair.car.borrow().clone());
            cursor = match pair.cdr.borrow().clone() {
                Value::Pair(cell) => Some(cell),
                _ => None,
            };
        }
        items
    }
}

/// One table's key comparison: the book's `same-key?` predicate,
/// `equal?` by default.
pub type SameKey = dyn Fn(&Value, &Value) -> bool;

/// The default key equality of the book's `assoc`: `equal?`, here the
/// structural `PartialEq` of `Value`.
#[must_use]
pub fn structural_same_key(a: &Value, b: &Value) -> bool {
    a == b
}

/// The book's table of 3.3.3: a dummy `*table*` header pair whose tail
/// is the list of `(key . value)` records. All state sits behind the one
/// `RefCell`, so a `Table` clone is a second name for the same records,
/// and the `same_key` predicate is per table, which is exercise 3.24's
/// constructor made a field.
pub struct Table {
    header: Rc<RefCell<Pair>>,
    same_key: Rc<SameKey>,
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table")
            .field("records", &self.records())
            .finish_non_exhaustive()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// The book's `(make-table)`: a fresh headed table comparing keys
    /// with `equal?`.
    #[must_use]
    pub fn new() -> Self {
        Self::with_same_key(Rc::new(structural_same_key))
    }

    /// Exercise 3.24's constructor: a table whose key comparison is the
    /// given `same_key` predicate instead of `equal?`.
    #[must_use]
    pub fn with_same_key(same_key: Rc<SameKey>) -> Self {
        Table {
            header: Rc::new(RefCell::new(cons_cell(Value::sym("*table*"), Value::Nil))),
            same_key,
        }
    }

    /// A clone of the header handle: the fixed location the book's
    /// `insert!` mutates through.
    #[must_use]
    pub fn header(&self) -> Pair {
        self.header.borrow().clone()
    }

    /// The book's `assoc`: the first record whose key satisfies the
    /// table's predicate, or `None`.
    #[must_use]
    pub fn assoc(&self, key: &Value, records: &Value) -> Option<Pair> {
        let same_key = Rc::clone(&self.same_key);
        assoc_by(key, records, move |a, b| same_key(a, b))
    }

    /// The book's `lookup`: the value stored under `key`, or `None`.
    #[must_use]
    pub fn lookup(&self, key: &Value) -> Option<Value> {
        let records = self.header().cdr.borrow().clone();
        self.assoc(key, &records)
            .map(|record| record.cdr.borrow().clone())
    }

    /// The book's `insert!`: replaces the value of an existing record or
    /// splices a new one at the front of the record list.
    pub fn insert(&self, key: Value, value: Value) {
        let header = self.header();
        let records = header.cdr.borrow().clone();
        match self.assoc(&key, &records) {
            Some(record) => set_cdr(&record, value),
            None => set_cdr(
                &header,
                Value::Pair(cons_cell(Value::Pair(cons_cell(key, value)), records)),
            ),
        }
    }

    /// The records of the table as `(key, value)` pairs, front to back.
    #[must_use]
    pub fn records(&self) -> Vec<(Value, Value)> {
        let mut out = Vec::new();
        let mut cursor = self.header().cdr.borrow().clone();
        while let Value::Pair(node) = cursor {
            let record = node.car.borrow().clone();
            if let Value::Pair(rec) = record {
                out.push((rec.car.borrow().clone(), rec.cdr.borrow().clone()));
            }
            cursor = node.cdr.borrow().clone();
        }
        out
    }
}

/// Walks a `Value` record list, returning the first `(key . value)`
/// record whose key satisfies `same_key`. The list elements are the
/// records, so the walk reads each element's `car` as the record and
/// advances by the element's `cdr`.
#[must_use]
pub fn assoc_by(
    key: &Value,
    records: &Value,
    same_key: impl Fn(&Value, &Value) -> bool,
) -> Option<Pair> {
    let mut cursor = records.clone();
    while let Value::Pair(node) = cursor {
        if let Some(rec) = matching_record(&node, key, &same_key) {
            return Some(rec);
        }
        cursor = node.cdr.borrow().clone();
    }
    None
}

/// The record held by one list element, when its key satisfies
/// `same_key`.
fn matching_record(
    node: &Pair,
    key: &Value,
    same_key: &impl Fn(&Value, &Value) -> bool,
) -> Option<Pair> {
    let Value::Pair(rec) = node.car.borrow().clone() else {
        return None;
    };
    let record_key = rec.car.borrow().clone();
    same_key(&record_key, key).then_some(rec)
}

/// The book's two-dimensional `lookup`: the first key names a subtable,
/// the second a record within it.
#[must_use]
pub fn lookup_2d(key1: &Value, key2: &Value, table: &Table) -> Option<Value> {
    let subtable = table.assoc(key1, &table.header().cdr.borrow())?;
    let sub_records = subtable.cdr.borrow().clone();
    table
        .assoc(key2, &sub_records)
        .map(|record| record.cdr.borrow().clone())
}

/// The book's two-dimensional `insert!`: reuses an existing subtable or
/// builds one holding the single new record.
pub fn insert_2d(key1: Value, key2: Value, value: Value, table: &Table) {
    let header = table.header();
    let records = header.cdr.borrow().clone();
    let Some(subtable) = table.assoc(&key1, &records) else {
        let record = Value::Pair(cons_cell(key2, value));
        let sub_records = cons_cell(record, Value::Nil);
        let subtable = cons_cell(key1, Value::Pair(sub_records));
        set_cdr(
            &header,
            Value::Pair(cons_cell(Value::Pair(subtable), records)),
        );
        return;
    };
    let sub_records = subtable.cdr.borrow().clone();
    if let Some(record) = table.assoc(&key2, &sub_records) {
        set_cdr(&record, value);
        return;
    }
    let record = cons_cell(key2, value);
    set_cdr(
        &subtable,
        Value::Pair(cons_cell(Value::Pair(record), sub_records)),
    );
}

/// The memo table of the edition plan: the host shape of the book's
/// local table, a `HashMap` behind one `RefCell`. `lookup_insert` is the
/// memo-fib discipline: answer a stored value, or compute once and
/// store.
#[derive(Default)]
pub struct MemoTable(RefCell<HashMap<Key, Value>>);

impl std::fmt::Debug for MemoTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoTable").finish_non_exhaustive()
    }
}

impl MemoTable {
    /// An empty memo table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the stored value for `key`, or computes it once with
    /// `compute`, stores, and returns it.
    ///
    /// # Errors
    /// Whatever `compute` raises; nothing is stored on failure.
    pub fn lookup_insert(
        &self,
        key: Key,
        compute: impl FnOnce() -> Result<Value, SchemeError>,
    ) -> Result<Value, SchemeError> {
        if let Some(stored) = self.0.borrow().get(&key) {
            return Ok(stored.clone());
        }
        let computed = compute()?;
        self.0.borrow_mut().insert(key, computed.clone());
        Ok(computed)
    }

    /// How many entries the table holds, for the exercise 3.27 traces.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    /// Whether the table stores nothing yet.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

/// The printed line of one probe observation.
pub type ProbeLog = Rc<RefCell<Vec<String>>>;

/// A fresh, empty probe log.
#[must_use]
pub fn probe_log() -> ProbeLog {
    Rc::new(RefCell::new(Vec::new()))
}

/// The book's `inverter-delay` of the sample simulation.
pub const INVERTER_DELAY: u64 = 2;
/// The book's `and-gate-delay`.
pub const AND_GATE_DELAY: u64 = 3;
/// The book's `or-gate-delay`.
pub const OR_GATE_DELAY: u64 = 5;

struct WireState {
    signal: Cell<u8>,
    actions: RefCell<Vec<Rc<dyn Fn()>>>,
}

/// One wire: a signal cell plus the action procedures to run whenever
/// the signal changes. The signal is 0 or 1, the book's digital levels.
#[derive(Clone)]
pub struct Wire {
    state: Rc<WireState>,
}

impl Default for Wire {
    fn default() -> Self {
        Wire {
            state: Rc::new(WireState {
                signal: Cell::new(0),
                actions: RefCell::new(Vec::new()),
            }),
        }
    }
}

impl Wire {
    /// The book's `make-wire`: signal 0, no actions.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `get-signal`.
    #[must_use]
    pub fn signal(&self) -> u8 {
        self.state.signal.get()
    }

    /// The book's `set-signal!`: stores the new value and, when it
    /// differs, calls each action in turn.
    pub fn set_signal(&self, value: u8) {
        if self.state.signal.get() == value {
            return;
        }
        self.state.signal.set(value);
        let snapshot: Vec<_> = self.state.actions.borrow().clone();
        for action in snapshot {
            action();
        }
    }

    /// The book's `add-action!`: the action joins the wire's list and
    /// then runs once immediately, which is the initialization exercise
    /// 3.31 asks about.
    pub fn add_action(&self, action: &Rc<dyn Fn()>) {
        self.attach(Rc::clone(action));
        action();
    }

    /// Exercise 3.31's variant: attaches without the immediate run, so a
    /// simulation can show what the initialization is for.
    pub fn add_action_deferred(&self, action: &Rc<dyn Fn()>) {
        self.attach(Rc::clone(action));
    }

    fn attach(&self, action: Rc<dyn Fn()>) {
        self.state.actions.borrow_mut().insert(0, action);
    }

    /// A weak handle to this wire, for actions stored on the wire
    /// itself: the wire owns its actions, so an action may not own the
    /// wire back without making the two immortal.
    fn downgrade(&self) -> Weak<WireState> {
        Rc::downgrade(&self.state)
    }
}

/// One scheduled action of the agenda; ordered by `(time, seq)` only.
struct Scheduled {
    time: u64,
    seq: u64,
    action: Rc<dyn Fn()>,
}

impl PartialEq for Scheduled {
    fn eq(&self, other: &Self) -> bool {
        self.seq == other.seq
    }
}

impl Eq for Scheduled {}

impl PartialOrd for Scheduled {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Scheduled {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.time, self.seq).cmp(&(other.time, other.seq))
    }
}

/// The book's agenda: a schedule keyed by time. The heap orders by
/// `(time, sequence)`, and the sequence number is the insertion order,
/// so actions added to the same segment run in the order they were
/// added, FIFO, which is exactly what exercise 3.32 relies on.
#[derive(Default)]
pub struct Agenda {
    now: Cell<u64>,
    next_seq: Cell<u64>,
    heap: RefCell<BinaryHeap<std::cmp::Reverse<Scheduled>>>,
}

impl std::fmt::Debug for Agenda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agenda")
            .field("current_time", &self.current_time())
            .field("pending", &self.heap.borrow().len())
            .finish_non_exhaustive()
    }
}

impl Agenda {
    /// The book's `make-agenda`: time 0, nothing scheduled.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `current-time`: the time of the most recently
    /// processed action.
    #[must_use]
    pub fn current_time(&self) -> u64 {
        self.now.get()
    }

    /// The book's `empty-agenda?`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.heap.borrow().is_empty()
    }

    /// The book's `add-to-agenda!`: schedules `action` at `time`.
    pub fn add_to(&self, time: u64, action: Rc<dyn Fn()>) {
        let seq = self.next_seq.get();
        self.next_seq.set(seq + 1);
        self.heap
            .borrow_mut()
            .push(std::cmp::Reverse(Scheduled { time, seq, action }));
    }

    /// The book's `after-delay`: schedules `action` `delay` from now.
    pub fn after_delay(self: &Rc<Self>, delay: u64, action: Rc<dyn Fn()>) {
        self.add_to(self.current_time() + delay, action);
    }

    /// The book's `propagate`: runs every scheduled action in time
    /// order, FIFO within one time, until the agenda is empty. The heap
    /// borrow ends before each action runs, because a gate action may
    /// schedule new work.
    pub fn propagate(&self) {
        loop {
            let next = self.heap.borrow_mut().pop();
            let Some(item) = next else {
                break;
            };
            let scheduled = item.0;
            self.now.set(scheduled.time);
            (scheduled.action)();
        }
    }
}

/// The book's `logical-not`; the digital levels are only 0 and 1, so the
/// book's `Invalid signal` error cannot arise by construction.
#[must_use]
pub fn logical_not(value: u8) -> u8 {
    u8::from(value == 0)
}

/// The book's `logical-and`.
#[must_use]
pub fn logical_and(a: u8, b: u8) -> u8 {
    u8::from(a != 0 && b != 0)
}

/// The book's `logical-or`.
#[must_use]
pub fn logical_or(a: u8, b: u8) -> u8 {
    u8::from(a != 0 || b != 0)
}

/// The book's `inverter`: when the input changes, the output takes
/// `logical-not` of it one `INVERTER_DELAY` later. The action reads the
/// input weakly, because it is stored on the input's own action list.
pub fn inverter(input: &Wire, output: &Wire, agenda: &Rc<Agenda>) {
    let input_weak = input.downgrade();
    let output_state = Rc::clone(&output.state);
    let agenda = Rc::clone(agenda);
    let action: Rc<dyn Fn()> = Rc::new(move || {
        let Some(input) = input_weak.upgrade() else {
            return;
        };
        let new_value = logical_not(input.signal.get());
        schedule_set(&agenda, &output_state, INVERTER_DELAY, new_value);
    });
    input.add_action(&action);
}

/// The book's `and-gate`: the output follows the conjunction of the two
/// inputs, `AND_GATE_DELAY` after the change that computed it.
pub fn and_gate(a1: &Wire, a2: &Wire, output: &Wire, agenda: &Rc<Agenda>) {
    let a1_weak = a1.downgrade();
    let a2_weak = a2.downgrade();
    let output_state = Rc::clone(&output.state);
    let agenda = Rc::clone(agenda);
    let action: Rc<dyn Fn()> = Rc::new(move || {
        let (Some(x), Some(y)) = (a1_weak.upgrade(), a2_weak.upgrade()) else {
            return;
        };
        let new_value = logical_and(x.signal.get(), y.signal.get());
        schedule_set(&agenda, &output_state, AND_GATE_DELAY, new_value);
    });
    a1.add_action(&action);
    a2.add_action(&action);
}

/// The book's `or-gate` as a primitive function box, the shape exercise
/// 3.28 asks for.
pub fn or_gate(a1: &Wire, a2: &Wire, output: &Wire, agenda: &Rc<Agenda>) {
    let a1_weak = a1.downgrade();
    let a2_weak = a2.downgrade();
    let output_state = Rc::clone(&output.state);
    let agenda = Rc::clone(agenda);
    let action: Rc<dyn Fn()> = Rc::new(move || {
        let (Some(x), Some(y)) = (a1_weak.upgrade(), a2_weak.upgrade()) else {
            return;
        };
        let new_value = logical_or(x.signal.get(), y.signal.get());
        schedule_set(&agenda, &output_state, OR_GATE_DELAY, new_value);
    });
    a1.add_action(&action);
    a2.add_action(&action);
}

/// Schedules one `set-signal` of `new_value` on the driven wire, the
/// gate action's second half, delayed past the gate's delay.
fn schedule_set(agenda: &Rc<Agenda>, output: &Rc<WireState>, delay: u64, new_value: u8) {
    let out = Wire {
        state: Rc::clone(output),
    };
    agenda.after_delay(
        delay,
        Rc::new(move || {
            out.set_signal(new_value);
        }),
    );
}

/// The book's `half-adder`: `s` is the sum bit, `c` the carry bit, of
/// `a` plus `b`. The internal wires `d` and `e` stay alive because the
/// gate actions the outer wires hold capture them, exactly the book's
/// story of closures keeping internal state.
#[expect(
    clippy::many_single_char_names,
    reason = "a b s c d e are the book's own wire names"
)]
pub fn half_adder(a: &Wire, b: &Wire, s: &Wire, c: &Wire, agenda: &Rc<Agenda>) {
    let d = Wire::new();
    let e = Wire::new();
    or_gate(a, b, &d, agenda);
    and_gate(a, b, c, agenda);
    inverter(c, &e, agenda);
    and_gate(&d, &e, s, agenda);
}

/// The book's `full-adder`: two half-adders and an or-gate, wired over
/// the internal wires `c1`, `c2`, and `s`.
pub fn full_adder(a: &Wire, b: &Wire, c_in: &Wire, sum: &Wire, c_out: &Wire, agenda: &Rc<Agenda>) {
    let c1 = Wire::new();
    let c2 = Wire::new();
    let s = Wire::new();
    half_adder(c_in, b, &s, &c2, agenda);
    half_adder(a, &s, sum, &c1, agenda);
    or_gate(&c1, &c2, c_out, agenda);
}

/// The book's `probe` for wires: every change of the wire appends the
/// book's line `name time  New-value = v` to the log.
pub fn probe_wire(name: &str, wire: &Wire, agenda: &Rc<Agenda>, log: &ProbeLog) {
    let name = name.to_string();
    let wire_weak = wire.downgrade();
    let agenda = Rc::clone(agenda);
    let log = Rc::clone(log);
    let wire_action: Rc<dyn Fn()> = Rc::new(move || {
        let Some(wire) = wire_weak.upgrade() else {
            return;
        };
        log.borrow_mut().push(format!(
            "{name} {}  New-value = {}",
            agenda.current_time(),
            wire.signal.get()
        ));
    });
    wire.add_action(&wire_action);
}

/// One participant in a constraint network: the book's constraint object
/// that answers `I-have-a-value` and `I-lost-my-value`. `informant`
/// names the object, so a connector can notify everyone except the
/// constraint that just talked to it.
pub trait Constraint {
    /// The identity the constraint sets connector values under.
    fn informant(&self) -> &Informant;

    /// The book's `I-have-a-value`.
    fn inform_about_value(&self);

    /// The book's `I-lost-my-value`.
    fn inform_about_no_value(&self);
}

/// An identity for whoever sets a connector: the book's `informant` and
/// `retractor` arguments. The `user` is one; each constraint makes its
/// own. Two informants are the same exactly when they are the same
/// object.
#[derive(Clone, Debug, Default)]
pub struct Informant(Rc<()>);

impl PartialEq for Informant {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Informant {
    /// A fresh identity.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `'user`.
    #[must_use]
    pub fn user() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct ConnectorState {
    value: RefCell<Option<f64>>,
    informant: RefCell<Option<Informant>>,
    constraints: RefCell<Vec<Weak<dyn Constraint>>>,
}

/// The book's connector: an object that holds a value and the list of
/// constraints it participates in. The constraint links are weak, so a
/// connector and its constraints do not keep each other alive; the
/// [`Network`] is the strong root of the whole graph.
#[derive(Clone, Default)]
pub struct Connector {
    state: Rc<ConnectorState>,
}

impl Connector {
    /// The book's `make-connector`: valueless, no constraints.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `has-value?`: whether anyone currently owns the value.
    #[must_use]
    pub fn has_value(&self) -> bool {
        self.state.informant.borrow().is_some()
    }

    /// The book's `get-value`: the current value, or `None` when the
    /// connector has none.
    #[must_use]
    pub fn value(&self) -> Option<f64> {
        if !self.has_value() {
            return None;
        }
        *self.state.value.borrow()
    }

    /// The book's `set-value!`: a valueless connector takes the value and
    /// notifies every constraint except the setter; a valued connector
    /// agrees with an equal value and raises a contradiction otherwise.
    ///
    /// # Errors
    /// [`SchemeError::UserRaised`] with the book's `Contradiction`
    /// message when the connector already holds a different value.
    pub fn set_value(&self, new_value: f64, setter: &Informant) -> Result<(), SchemeError> {
        if !self.has_value() {
            *self.state.value.borrow_mut() = Some(new_value);
            *self.state.informant.borrow_mut() = Some(setter.clone());
            self.notify_value_except(setter);
            return Ok(());
        }
        let stored = *self.state.value.borrow();
        if stored == Some(new_value) {
            return Ok(());
        }
        let mut irritants = Vec::new();
        if let Some(old) = stored {
            irritants.push(Value::real(old));
        }
        irritants.push(Value::real(new_value));
        Err(SchemeError::UserRaised {
            message: "Contradiction".into(),
            irritants,
        })
    }

    /// The book's `forget-value!`: only the informant that set the value
    /// can retract it; the connector then tells its constraints the value
    /// is gone.
    pub fn forget_value(&self, retractor: &Informant) {
        let owned = self
            .state
            .informant
            .borrow()
            .as_ref()
            .is_some_and(|informant| informant == retractor);
        if !owned {
            return;
        }
        *self.state.informant.borrow_mut() = None;
        self.notify_no_value_except(retractor);
    }

    /// The book's `connect`: joins the constraint unless it is already
    /// here, and brings it up to date when the connector has a value.
    pub fn connect(&self, constraint: &Rc<dyn Constraint>) {
        self.add_constraint_once(constraint);
        if self.has_value() {
            constraint.inform_about_value();
        }
    }

    fn add_constraint_once(&self, constraint: &Rc<dyn Constraint>) {
        let mut constraints = self.state.constraints.borrow_mut();
        let present = constraints.iter().any(|weak| {
            weak.upgrade()
                .is_some_and(|known| known.informant() == constraint.informant())
        });
        if !present {
            constraints.insert(0, Rc::downgrade(constraint));
        }
    }

    fn each_constraint_except(&self, exception: &Informant) -> Vec<Rc<dyn Constraint>> {
        self.state
            .constraints
            .borrow()
            .iter()
            .filter_map(Weak::upgrade)
            .filter(|constraint| constraint.informant() != exception)
            .collect()
    }

    /// The book's `for-each-except` with `inform-about-value`.
    fn notify_value_except(&self, exception: &Informant) {
        for constraint in self.each_constraint_except(exception) {
            constraint.inform_about_value();
        }
    }

    /// The book's `for-each-except` with `inform-about-no-value`.
    fn notify_no_value_except(&self, exception: &Informant) {
        for constraint in self.each_constraint_except(exception) {
            constraint.inform_about_no_value();
        }
    }
}

/// The strong root of a constraint network: every connector and every
/// constraint the section's constructors build registers here, which is
/// the Rust answer to the book's question of what keeps the internal
/// connectors alive.
#[derive(Clone, Default)]
pub struct Network {
    connectors: RefCell<Vec<Connector>>,
    constraints: RefCell<Vec<Rc<dyn Constraint>>>,
}

impl Network {
    /// An empty network.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The book's `make-connector`, registered so the network owns it.
    #[must_use]
    pub fn connector(&self) -> Connector {
        let connector = Connector::new();
        self.connectors.borrow_mut().push(connector.clone());
        connector
    }

    /// Adopts a connector built outside the network, typically one of
    /// the two named ends the user keeps.
    pub fn adopt(&self, connector: &Connector) {
        self.connectors.borrow_mut().push(connector.clone());
    }

    fn register(&self, constraint: Rc<dyn Constraint>) {
        self.constraints.borrow_mut().push(constraint);
    }
}

fn weak_connector(connector: &Connector) -> Weak<ConnectorState> {
    Rc::downgrade(&connector.state)
}

fn connector_of(weak: &Weak<ConnectorState>) -> Option<Connector> {
    weak.upgrade().map(|state| Connector { state })
}

/// The book's `adder`: constrains `a1 + a2 = sum`, in any direction the
/// information allows.
pub fn adder(
    network: &Network,
    a1: &Connector,
    a2: &Connector,
    sum: &Connector,
) -> Rc<dyn Constraint> {
    let constraint = Rc::new(Adder {
        a1: weak_connector(a1),
        a2: weak_connector(a2),
        sum: weak_connector(sum),
        me: Informant::new(),
    });
    let object: Rc<dyn Constraint> = constraint.clone();
    a1.connect(&object);
    a2.connect(&object);
    sum.connect(&object);
    network.register(object);
    constraint
}

struct Adder {
    a1: Weak<ConnectorState>,
    a2: Weak<ConnectorState>,
    sum: Weak<ConnectorState>,
    me: Informant,
}

impl Adder {
    fn process_new_value(&self) {
        let (Some(a1), Some(a2), Some(sum)) = (
            connector_of(&self.a1),
            connector_of(&self.a2),
            connector_of(&self.sum),
        ) else {
            return;
        };
        let (x, y, total) = (a1.value(), a2.value(), sum.value());
        if let (Some(x), Some(y)) = (x, y) {
            let _ = sum.set_value(x + y, &self.me);
        } else if let (Some(x), Some(total)) = (x, total) {
            let _ = a2.set_value(total - x, &self.me);
        } else if let (Some(y), Some(total)) = (y, total) {
            let _ = a1.set_value(total - y, &self.me);
        }
    }
}

impl Constraint for Adder {
    fn informant(&self) -> &Informant {
        &self.me
    }

    fn inform_about_value(&self) {
        self.process_new_value();
    }

    fn inform_about_no_value(&self) {
        if let Some(sum) = connector_of(&self.sum) {
            sum.forget_value(&self.me);
        }
        if let Some(a1) = connector_of(&self.a1) {
            a1.forget_value(&self.me);
        }
        if let Some(a2) = connector_of(&self.a2) {
            a2.forget_value(&self.me);
        }
        self.process_new_value();
    }
}

/// The book's `multiplier`: constrains `m1 * m2 = product`, with the
/// book's shortcut that a zero factor fixes the product at 0 even when
/// the other factor is unknown.
pub fn multiplier(
    network: &Network,
    m1: &Connector,
    m2: &Connector,
    product: &Connector,
) -> Rc<dyn Constraint> {
    let constraint = Rc::new(Multiplier {
        m1: weak_connector(m1),
        m2: weak_connector(m2),
        product: weak_connector(product),
        me: Informant::new(),
    });
    let object: Rc<dyn Constraint> = constraint.clone();
    m1.connect(&object);
    m2.connect(&object);
    product.connect(&object);
    network.register(object);
    constraint
}

struct Multiplier {
    m1: Weak<ConnectorState>,
    m2: Weak<ConnectorState>,
    product: Weak<ConnectorState>,
    me: Informant,
}

impl Multiplier {
    fn process_new_value(&self) {
        let (Some(m1), Some(m2), Some(product)) = (
            connector_of(&self.m1),
            connector_of(&self.m2),
            connector_of(&self.product),
        ) else {
            return;
        };
        let (x, y, total) = (m1.value(), m2.value(), product.value());
        if x == Some(0.0) || y == Some(0.0) {
            let _ = product.set_value(0.0, &self.me);
        } else if let (Some(x), Some(y)) = (x, y) {
            let _ = product.set_value(x * y, &self.me);
        } else if let (Some(total), Some(x)) = (total, x) {
            let _ = m2.set_value(total / x, &self.me);
        } else if let (Some(total), Some(y)) = (total, y) {
            let _ = m1.set_value(total / y, &self.me);
        }
    }
}

impl Constraint for Multiplier {
    fn informant(&self) -> &Informant {
        &self.me
    }

    fn inform_about_value(&self) {
        self.process_new_value();
    }

    fn inform_about_no_value(&self) {
        if let Some(product) = connector_of(&self.product) {
            product.forget_value(&self.me);
        }
        if let Some(m1) = connector_of(&self.m1) {
            m1.forget_value(&self.me);
        }
        if let Some(m2) = connector_of(&self.m2) {
            m2.forget_value(&self.me);
        }
        self.process_new_value();
    }
}

/// The book's `constant`: fixes one connector at `value` forever. A
/// correct network never notifies a constant box, so the book's error
/// for a stray message has nothing to answer.
pub fn constant(network: &Network, value: f64, connector: &Connector) -> Rc<dyn Constraint> {
    struct ConstantBox(Informant);

    impl Constraint for ConstantBox {
        fn informant(&self) -> &Informant {
            &self.0
        }

        fn inform_about_value(&self) {}

        fn inform_about_no_value(&self) {}
    }

    let me = Rc::new(ConstantBox(Informant::new()));
    let object: Rc<dyn Constraint> = me.clone();
    connector.connect(&object);
    let _ = connector.set_value(value, me.informant());
    network.register(object);
    me
}

/// The book's `probe` for connectors: every value change appends
/// `Probe: name = v`, and every retraction appends `Probe: name = ?`.
pub fn probe_connector(
    network: &Network,
    name: &str,
    connector: &Connector,
    log: &ProbeLog,
) -> Rc<dyn Constraint> {
    struct Probe {
        name: String,
        connector: Weak<ConnectorState>,
        log: ProbeLog,
        me: Informant,
    }

    impl Constraint for Probe {
        fn informant(&self) -> &Informant {
            &self.me
        }

        fn inform_about_value(&self) {
            let Some(connector) = self.connector.upgrade() else {
                return;
            };
            let probe = Connector { state: connector };
            if let Some(value) = probe.value() {
                self.log
                    .borrow_mut()
                    .push(format!("Probe: {} = {value}", self.name));
            }
        }

        fn inform_about_no_value(&self) {
            self.log
                .borrow_mut()
                .push(format!("Probe: {} = ?", self.name));
        }
    }

    let me = Rc::new(Probe {
        name: name.to_string(),
        connector: weak_connector(connector),
        log: Rc::clone(log),
        me: Informant::new(),
    });
    let object: Rc<dyn Constraint> = me.clone();
    connector.connect(&object);
    network.register(object);
    me
}

/// The book's `celsius-fahrenheit-converter`: the network of
/// `9C = 5(F - 32)` over the two named connectors. Both ends must be
/// adopted by the same network so the whole graph stays alive together.
#[expect(
    clippy::many_single_char_names,
    reason = "u v w x y c f are the book's own connector names"
)]
pub fn celsius_fahrenheit_converter(network: &Network, c: &Connector, f: &Connector) {
    network.adopt(c);
    network.adopt(f);
    let u = network.connector();
    let v = network.connector();
    let w = network.connector();
    let x = network.connector();
    let y = network.connector();
    multiplier(network, c, &w, &u);
    multiplier(network, &v, &x, &u);
    adder(network, &v, &y, f);
    constant(network, 9.0, &w);
    constant(network, 5.0, &x);
    constant(network, 32.0, &y);
}
