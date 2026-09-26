// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.3

//! Section 5.3: Storage allocation and garbage collection.
//!
//! The list-structure memory of 5.3.1: a pair is a cell in one of two
//! semispaces, the-cars and the-cdrs vectors hold the cells' two
//! halves, and every datum is a typed [`Word`] whose kind rides beside
//! its payload -- a pair pointer names a cell index, a number or a
//! symbol is its own payload, the empty list is [`Word::Empty`]. The
//! allocator hands out cells at the `free` pointer and bumps it; the
//! top [`Memory::new`] `root_capacity` cells of each semispace are a
//! reserved strip the allocator never enters, where a collection's
//! pre-allocated root list lives.
//!
//! The primitive list operations of 5.3.1 are the module's methods,
//! and [`memory_operations`] installs them as the operations of a
//! 5.2 register machine: the book's "assume that the list-structure
//! memory operations are available as machine primitives". Registers
//! carry words encoded as `Value`s ([`Word::Pair`] as the tagged
//! pointer `(p i)`, [`Word::Num`] as the plain integer, the empty list
//! as `()`), so the machines of exercises 5.21 and 5.22 run on the
//! unmodified simulator with the memory shared through
//! [`SharedMemory`].
//!
//! The stop-and-copy collector of 5.3.2 is the book's controller
//! listing run verbatim: [`collect`] stores the caller's roots into
//! the reserved strip as one list, hands the machine the two
//! semispaces' vector bases, and lets the book's `gc-loop` relocate
//! every reachable cell into the other space, broken hearts and
//! forwarding addresses included. After the book's `gc-flip` the
//! spaces have swapped roles, the relocated root list is walked to
//! hand each root its forwarded word, and allocation resumes after
//! the relocated data. Because this edition's stack is the
//! simulator's own vector rather than list structure in the memory,
//! collections are driven explicitly with the caller's live words,
//! and an exhausted memory is a typed fault naming the allocation.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::sec_5_2::{Fault, Machine, OpHandler, make_machine_from_datums};
use sicp_runtime::{Symbol, Value, read_program};

/// The memory shared between a program and its machines: every
/// operation closes over one and mutates it through the cell.
pub type SharedMemory = Rc<RefCell<Memory>>;

/// One typed word: the payload of a memory cell or of a machine
/// register. The book's letter prefixes name the kinds: a pair
/// pointer is `p3`, a number `n4`, the empty list `e0`, and the
/// collector's moved tag `broken-heart`.
#[derive(Clone, Debug, PartialEq)]
pub enum Word {
    /// A pointer to the pair stored at this cell index.
    Pair(usize),
    /// A number carried in the word itself, the book's footnote
    /// choice that makes `eq?` a sound equality for equal numbers.
    Num(i128),
    /// An interned symbol.
    Sym(Symbol),
    /// The empty list, the book's `e0`.
    Empty,
    /// The collector's moved-object tag; the forwarding address sits
    /// beside it in the cell's cdr.
    BrokenHeart,
}

impl Word {
    /// The word's rendering in the book's notation: `p3`, `n4`, the
    /// symbol's name, `e0`, `broken-heart`.
    #[must_use]
    pub fn to_word_string(&self) -> String {
        match self {
            Self::Pair(index) => format!("p{index}"),
            Self::Num(n) => format!("n{n}"),
            Self::Sym(name) => name.to_string(),
            Self::Empty => "e0".to_owned(),
            Self::BrokenHeart => "broken-heart".to_owned(),
        }
    }
}

impl From<Word> for Value {
    fn from(word: Word) -> Self {
        match word {
            // A cell index always fits i128, and a tagged pointer
            // keeps the pair kind distinct from the number words.
            Word::Pair(index) => Value::tagged(
                "p",
                Value::Int(i128::try_from(index).expect("usize fits i128")),
            ),
            Word::Num(n) => Value::Int(n),
            Word::Sym(name) => Value::Sym(name),
            Word::Empty => Value::Nil,
            Word::BrokenHeart => Value::tagged("broken-heart", Value::Nil),
        }
    }
}

/// Why a memory primitive refused: the typed failures of the
/// allocator and the selectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryFault {
    /// The free pointer reached the reserved strip: no cell is left
    /// for the allocator.
    Exhausted,
    /// A selector or mutator was handed a word that is not a pair.
    NotPair {
        /// The operation that refused.
        op: &'static str,
        /// The offending word's rendering.
        word: String,
    },
    /// A vector index left the vector.
    IndexOutOfRange {
        /// The operation that refused.
        op: &'static str,
    },
    /// A register carried a value that is not a word of this model.
    NotAWord {
        /// The value's printed form.
        value: String,
    },
    /// The reserved strip cannot hold the requested root list.
    RootsTooLarge {
        /// How many roots arrived.
        roots: usize,
        /// How many the strip holds.
        capacity: usize,
    },
}

impl fmt::Display for MemoryFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exhausted => write!(f, "the memory is exhausted"),
            Self::NotPair { op, word } => write!(f, "{op}: not a pair: {word}"),
            Self::IndexOutOfRange { op } => write!(f, "{op}: memory index out of range"),
            Self::NotAWord { value } => write!(f, "not a memory word: {value}"),
            Self::RootsTooLarge { roots, capacity } => {
                write!(f, "{roots} roots exceed the strip's {capacity} cells")
            }
        }
    }
}

impl std::error::Error for MemoryFault {}

/// Decodes a register value back into the word it carries.
///
/// # Errors
/// [`MemoryFault::NotAWord`] when the value is not a word of this
/// model.
pub fn word_of(value: &Value) -> Result<Word, MemoryFault> {
    match value {
        Value::Int(n) => Ok(Word::Num(*n)),
        Value::Sym(name) => Ok(Word::Sym(Rc::clone(name))),
        Value::Nil => Ok(Word::Empty),
        Value::Tagged { tag, data } => match (tag.as_ref(), data.as_ref()) {
            ("p", Value::Int(index)) => {
                let index = usize::try_from(*index).map_err(|_| not_a_word(value))?;
                Ok(Word::Pair(index))
            }
            ("broken-heart", data) if data.is_nil() => Ok(Word::BrokenHeart),
            _ => Err(not_a_word(value)),
        },
        _ => Err(not_a_word(value)),
    }
}

fn not_a_word(value: &Value) -> MemoryFault {
    MemoryFault::NotAWord {
        value: sicp_runtime::display_value(value),
    }
}

/// The index part of a word: what the book's numeric operations on
/// pointers and vector indexing use.
fn index_part(word: &Word) -> Result<usize, MemoryFault> {
    match word {
        Word::Pair(index) => Ok(*index),
        Word::Num(n) => usize::try_from(*n).map_err(|_| MemoryFault::NotAWord {
            value: word.to_word_string(),
        }),
        _ => Err(MemoryFault::NotAWord {
            value: word.to_word_string(),
        }),
    }
}

/// One semispace: the-cars and the-cdrs vectors of equal length.
#[derive(Clone, Debug)]
struct Space {
    cars: Vec<Word>,
    cdrs: Vec<Word>,
}

impl Space {
    fn blank(size: usize) -> Self {
        Self {
            cars: vec![Word::Empty; size],
            cdrs: vec![Word::Empty; size],
        }
    }
}

/// The list-structure memory: two semispaces, the free pointer, and
/// which semispace is working. The top `root_capacity` cells of each
/// semispace form the reserved strip the allocator never enters.
#[derive(Debug)]
pub struct Memory {
    size: usize,
    root_capacity: usize,
    spaces: [Space; 2],
    free: usize,
    working: usize,
    collections: u32,
}

impl Memory {
    /// A fresh memory: `size` cells per semispace, the top
    /// `root_capacity` of them reserved for a collection's root list,
    /// and allocation starting at `free` in semispace 0. Cell 0 stays
    /// untouched unless `free` starts there, so a drawing drawn from
    /// `free = 1` shows the book's blank column.
    ///
    /// # Panics
    /// When `root_capacity >= size` or `free` opens inside the strip:
    /// a memory with no data cells cannot exist.
    #[must_use]
    pub fn new(size: usize, root_capacity: usize, free: usize) -> Self {
        assert!(root_capacity < size, "the reserved strip leaves no data");
        assert!(free <= size - root_capacity, "free opens in the strip");
        Self {
            size,
            root_capacity,
            spaces: [Space::blank(size), Space::blank(size)],
            free,
            working: 0,
            collections: 0,
        }
    }

    /// The allocator's ceiling: the free pointer never enters the
    /// reserved strip.
    #[must_use]
    pub fn data_limit(&self) -> usize {
        self.size - self.root_capacity
    }

    /// The free pointer as the pair word it is: the book gives free
    /// "a pair pointer containing the next available index".
    #[must_use]
    pub fn free_word(&self) -> Word {
        Word::Pair(self.free)
    }

    /// How many collections have run.
    #[must_use]
    pub fn collections(&self) -> u32 {
        self.collections
    }

    /// Which semispace is working: 0 or 1.
    #[must_use]
    pub fn working(&self) -> usize {
        self.working
    }

    /// The allocation path: stores the two halves at the free
    /// pointer, bumps it, and answers the pair word.
    ///
    /// # Errors
    /// [`MemoryFault::Exhausted`] when the free pointer has reached
    /// the reserved strip.
    #[expect(
        clippy::similar_names,
        reason = "the book names a pair's two halves the car and the cdr"
    )]
    pub fn cons(&mut self, car_word: Word, cdr_word: Word) -> Result<Word, MemoryFault> {
        if self.free >= self.data_limit() {
            return Err(MemoryFault::Exhausted);
        }
        let index = self.free;
        self.spaces[self.working].cars[index] = car_word;
        self.spaces[self.working].cdrs[index] = cdr_word;
        self.free += 1;
        Ok(Word::Pair(index))
    }

    /// The car of a pair, read through the index part of the pointer.
    ///
    /// # Errors
    /// [`MemoryFault::NotPair`] when `pair_word` is not a pair,
    /// [`MemoryFault::IndexOutOfRange`] when the index leaves the
    /// vector.
    pub fn car(&self, pair_word: &Word) -> Result<Word, MemoryFault> {
        let Word::Pair(index) = pair_word else {
            return Err(not_pair("car", pair_word));
        };
        self.read_car(*index)
    }

    /// The cdr of a pair.
    ///
    /// # Errors
    /// As [`Memory::car`].
    pub fn cdr(&self, pair_word: &Word) -> Result<Word, MemoryFault> {
        let Word::Pair(index) = pair_word else {
            return Err(not_pair("cdr", pair_word));
        };
        self.read_cdr(*index)
    }

    /// `set-car!`: overwrites the car half of a pair.
    ///
    /// # Errors
    /// As [`Memory::car`].
    pub fn set_car(&mut self, pair_word: &Word, value: Word) -> Result<(), MemoryFault> {
        let Word::Pair(index) = pair_word else {
            return Err(not_pair("set-car!", pair_word));
        };
        self.write_car(*index, value)
    }

    /// `set-cdr!`: overwrites the cdr half of a pair.
    ///
    /// # Errors
    /// As [`Memory::car`].
    pub fn set_cdr(&mut self, pair_word: &Word, value: Word) -> Result<(), MemoryFault> {
        let Word::Pair(index) = pair_word else {
            return Err(not_pair("set-cdr!", pair_word));
        };
        self.write_cdr(*index, value)
    }

    fn read_car(&self, index: usize) -> Result<Word, MemoryFault> {
        self.spaces[self.working]
            .cars
            .get(index)
            .cloned()
            .ok_or(MemoryFault::IndexOutOfRange { op: "car" })
    }

    fn read_cdr(&self, index: usize) -> Result<Word, MemoryFault> {
        self.spaces[self.working]
            .cdrs
            .get(index)
            .cloned()
            .ok_or(MemoryFault::IndexOutOfRange { op: "cdr" })
    }

    fn write_car(&mut self, index: usize, value: Word) -> Result<(), MemoryFault> {
        let slot = self.spaces[self.working]
            .cars
            .get_mut(index)
            .ok_or(MemoryFault::IndexOutOfRange { op: "set-car!" })?;
        *slot = value;
        Ok(())
    }

    fn write_cdr(&mut self, index: usize, value: Word) -> Result<(), MemoryFault> {
        let slot = self.spaces[self.working]
            .cdrs
            .get_mut(index)
            .ok_or(MemoryFault::IndexOutOfRange { op: "set-cdr!" })?;
        *slot = value;
        Ok(())
    }

    /// The memory-vector drawing of the working semispace: the index
    /// row and the the-cars and the-cdrs rows of Figure 5.14, one
    /// column per cell, blank cells showing `e0`.
    #[must_use]
    #[expect(
        clippy::similar_names,
        reason = "the book names the two semispace vectors the-cars and the-cdrs"
    )]
    pub fn dump(&self) -> String {
        let space = &self.spaces[self.working];
        let index_row: Vec<String> = (0..self.size).map(|i| i.to_string()).collect();
        let cars_row: Vec<String> = space.cars.iter().map(Word::to_word_string).collect();
        let cdrs_row: Vec<String> = space.cdrs.iter().map(Word::to_word_string).collect();
        let widths: Vec<usize> = (0..self.size)
            .map(|i| {
                index_row[i]
                    .len()
                    .max(cars_row[i].len())
                    .max(cdrs_row[i].len())
                    + 2
            })
            .collect();
        let row = |name: &str, fields: &[String]| -> String {
            let mut line = String::from(name);
            for (field, width) in fields.iter().zip(&widths) {
                let padding = width.saturating_sub(field.len()).max(1);
                line.push_str(field);
                line.push_str(&" ".repeat(padding));
            }
            line.trim_end().to_owned()
        };
        [
            row("index    ", &index_row),
            row("the-cars ", &cars_row),
            row("the-cdrs ", &cdrs_row),
        ]
        .join("\n")
    }

    /// The printed form of a structure: `(1 2 (3 4))` across proper
    /// lists, `(a . b)` across a dotted tail. The walk reads the
    /// working semispace and follows no cycle guard: the structures
    /// of this section are the finite ones the exercises build.
    ///
    /// # Errors
    /// [`MemoryFault::NotPair`] or [`MemoryFault::IndexOutOfRange`]
    /// when a pointer leaves the vector, [`MemoryFault::NotAWord`]
    /// when a count misnames a cell.
    #[expect(
        clippy::similar_names,
        reason = "the book names a pair's two halves the car and the cdr"
    )]
    pub fn write(&self, word: &Word) -> Result<String, MemoryFault> {
        match word {
            Word::Num(n) => Ok(n.to_string()),
            Word::Sym(name) => Ok(name.to_string()),
            Word::Empty => Ok("()".to_owned()),
            Word::BrokenHeart => Ok("broken-heart".to_owned()),
            Word::Pair(_) => match self.cdr(word)? {
                Word::Pair(_) | Word::Empty => {
                    let mut parts = Vec::new();
                    let mut cursor = word.clone();
                    while let Word::Pair(_) = cursor {
                        parts.push(self.write(&self.car(&cursor)?)?);
                        cursor = self.cdr(&cursor)?;
                    }
                    Ok(format!("({})", parts.join(" ")))
                }
                dotted => {
                    let car_text = self.write(&self.car(word)?)?;
                    let cdr_text = self.write(&dotted)?;
                    Ok(format!("({car_text} . {cdr_text})"))
                }
            },
        }
    }
}

fn not_pair(op: &'static str, word: &Word) -> MemoryFault {
    MemoryFault::NotPair {
        op,
        word: word.to_word_string(),
    }
}

/// One semispace vector the collector names: the `the-cars` /
/// `the-cdrs` / `new-cars` / `new-cdrs` registers hold these as their
/// vector bases, and the book's `gc-flip` swaps them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VectorBase {
    /// The working space's the-cars vector before the flip.
    TheCars,
    /// The working space's the-cdrs vector before the flip.
    TheCdrs,
    /// The other space's new-cars vector before the flip.
    NewCars,
    /// The other space's new-cdrs vector before the flip.
    NewCdrs,
}

impl VectorBase {
    /// The base's register rendering.
    #[must_use]
    fn name(self) -> &'static str {
        match self {
            Self::TheCars => "the-cars",
            Self::TheCdrs => "the-cdrs",
            Self::NewCars => "new-cars",
            Self::NewCdrs => "new-cdrs",
        }
    }

    /// Which semispace the base addresses before the flip.
    #[must_use]
    fn space(self) -> usize {
        match self {
            Self::TheCars | Self::TheCdrs => 0,
            Self::NewCars | Self::NewCdrs => 1,
        }
    }

    /// Whether the base names a cars vector rather than a cdrs one.
    #[must_use]
    fn is_cars(self) -> bool {
        matches!(self, Self::TheCars | Self::NewCars)
    }

    /// The base a register value names, if any.
    #[must_use]
    fn of(value: &Value) -> Option<Self> {
        let Value::Sym(name) = value else {
            return None;
        };
        match name.as_ref() {
            "the-cars" => Some(Self::TheCars),
            "the-cdrs" => Some(Self::TheCdrs),
            "new-cars" => Some(Self::NewCars),
            "new-cdrs" => Some(Self::NewCdrs),
            _ => None,
        }
    }
}

/// A register-carrying fault: every operation failure of the memory
/// rides back to the simulator as [`Fault::Op`].
fn op_fault(op: &'static str, error: &MemoryFault) -> Fault {
    Fault::Op {
        op: op.to_owned(),
        message: error.to_string(),
        step: 0,
    }
}

/// Decodes the operation's inputs, refusing a non-word register.
fn words_of(op: &'static str, args: &[Value]) -> Result<Vec<Word>, Fault> {
    let words: Result<Vec<Word>, MemoryFault> = args.iter().map(word_of).collect();
    words.map_err(|e| op_fault(op, &e))
}

/// The 5.3.1 primitive list operations over one shared memory, as
/// 5.2 machine operations: `cons`, `car`, `cdr`, `set-car!`,
/// `set-cdr!`, `eq?`, and the type predicates. A failure comes back
/// as [`Fault::Op`] naming the operation and the refused word.
#[expect(
    clippy::similar_names,
    reason = "the book names a pair's two halves the car and the cdr"
)]
#[must_use]
pub fn memory_operations(mem: &SharedMemory) -> Vec<(&'static str, OpHandler)> {
    type Selector = fn(&Memory, &Word) -> Result<Word, MemoryFault>;
    type Mutator = fn(&mut Memory, &Word, Word) -> Result<(), MemoryFault>;

    let mut ops: Vec<(&'static str, OpHandler)> = Vec::new();
    let cons_mem = Rc::clone(mem);
    ops.push((
        "cons",
        Rc::new(move |_machine, args| {
            let words = words_of("cons", args)?;
            let [car_word, cdr_word] = words.as_slice() else {
                return Err(arity("cons", 2, args.len()));
            };
            cons_mem
                .borrow_mut()
                .cons(car_word.clone(), cdr_word.clone())
                .map(Value::from)
                .map_err(|e| op_fault("cons", &e))
        }),
    ));
    for (name, select) in [("car", select_car as Selector), ("cdr", select_cdr)] {
        let loop_mem = Rc::clone(mem);
        ops.push((
            name,
            Rc::new(move |_machine, args| {
                let words = words_of(name, args)?;
                let [pair_word] = words.as_slice() else {
                    return Err(arity(name, 1, args.len()));
                };
                select(&loop_mem.borrow(), pair_word)
                    .map(Value::from)
                    .map_err(|e| op_fault(name, &e))
            }),
        ));
    }
    for (name, mutate) in [
        ("set-car!", Memory::set_car as Mutator),
        ("set-cdr!", Memory::set_cdr),
    ] {
        let loop_mem = Rc::clone(mem);
        ops.push((
            name,
            Rc::new(move |_machine, args| {
                let words = words_of(name, args)?;
                let [pair_word, value] = words.as_slice() else {
                    return Err(arity(name, 2, args.len()));
                };
                mutate(&mut loop_mem.borrow_mut(), pair_word, value.clone())
                    .map(|()| Value::sym("done"))
                    .map_err(|e| op_fault(name, &e))
            }),
        ));
    }
    for (name, test) in [
        ("null?", is_null as fn(&Word) -> bool),
        ("pair?", is_pair),
        ("symbol?", is_symbol),
        ("number?", is_number),
    ] {
        ops.push((
            name,
            Rc::new(move |_machine, args| {
                let words = words_of(name, args)?;
                let [word] = words.as_slice() else {
                    return Err(arity(name, 1, args.len()));
                };
                Ok(Value::Bool(test(word)))
            }),
        ));
    }
    ops.push((
        "eq?",
        Rc::new(move |_machine, args| {
            let words = words_of("eq?", args)?;
            let [left, right] = words.as_slice() else {
                return Err(arity("eq?", 2, args.len()));
            };
            Ok(Value::Bool(left == right))
        }),
    ));
    ops
}

fn select_car(mem: &Memory, pair_word: &Word) -> Result<Word, MemoryFault> {
    mem.car(pair_word)
}

fn select_cdr(mem: &Memory, pair_word: &Word) -> Result<Word, MemoryFault> {
    mem.cdr(pair_word)
}

fn is_null(word: &Word) -> bool {
    matches!(word, Word::Empty)
}

fn is_pair(word: &Word) -> bool {
    matches!(word, Word::Pair(_))
}

fn is_symbol(word: &Word) -> bool {
    matches!(word, Word::Sym(_))
}

fn is_number(word: &Word) -> bool {
    matches!(word, Word::Num(_))
}

fn arity(op: &'static str, want: usize, got: usize) -> Fault {
    Fault::Op {
        op: op.to_owned(),
        message: format!("{op} takes {want} word arguments, got {got}"),
        step: 0,
    }
}

/// The stop-and-copy collector: the book's arrangement around its
/// 5.3.2 controller. The roots go into one pre-allocated list in the
/// working semispace's reserved strip, the machine relocates every
/// cell reachable from it into the other semispace, the book's
/// `gc-flip` swaps the vector bases, and the relocated root list is
/// walked to return each root's forwarded word. Broken hearts and
/// forwarding addresses mark the old space; garbage is never copied.
///
/// # Errors
/// A typed [`Fault::Op`] when the roots overflow the strip or the
/// reachable data does not fit the other semispace's data area, plus
/// any assembly fault of the collector machine.
#[expect(
    clippy::similar_names,
    reason = "the book names the base registers the-cars, the-cdrs, new-cars, and new-cdrs"
)]
pub fn collect(mem: &SharedMemory, roots: &[Word]) -> Result<Vec<Word>, Fault> {
    let root = plant_root_list(mem, roots)?;
    let mut machine = make_machine_from_datums(
        &[
            "root",
            "the-cars",
            "the-cdrs",
            "new-cars",
            "new-cdrs",
            "free",
            "scan",
            "old",
            "oldcr",
            "new",
            "relocate-continue",
            "temp",
        ],
        &collector_operations(mem),
        &collector_datums()?,
    )?;
    let working = mem.borrow().working;
    // The controller's four vector registers are fixed names; the
    // base each one holds depends on which semispace is working.
    let (cars, cdrs, new_cars, new_cdrs) = match working {
        0 => (
            VectorBase::TheCars,
            VectorBase::TheCdrs,
            VectorBase::NewCars,
            VectorBase::NewCdrs,
        ),
        _ => (
            VectorBase::NewCars,
            VectorBase::NewCdrs,
            VectorBase::TheCars,
            VectorBase::TheCdrs,
        ),
    };
    machine.set_register("root", Value::from(root))?;
    machine.set_register("the-cars", Value::sym(cars.name()))?;
    machine.set_register("the-cdrs", Value::sym(cdrs.name()))?;
    machine.set_register("new-cars", Value::sym(new_cars.name()))?;
    machine.set_register("new-cdrs", Value::sym(new_cdrs.name()))?;
    machine.start()?;
    finish_collection(mem, &machine, roots.len())
}

/// Stores the roots as one list in the reserved strip and answers its
/// head, or the empty list for a rootless collection.
fn plant_root_list(mem: &SharedMemory, roots: &[Word]) -> Result<Word, Fault> {
    let mut space = mem.borrow_mut();
    if roots.len() > space.root_capacity {
        let too_large = MemoryFault::RootsTooLarge {
            roots: roots.len(),
            capacity: space.root_capacity,
        };
        return Err(op_fault("collect", &too_large));
    }
    let limit = space.data_limit();
    let working = space.working;
    for (offset, word) in roots.iter().enumerate() {
        let cell = limit + offset;
        let next = if offset + 1 < roots.len() {
            Word::Pair(cell + 1)
        } else {
            Word::Empty
        };
        space.spaces[working].cars[cell] = word.clone();
        space.spaces[working].cdrs[cell] = next;
    }
    Ok(if roots.is_empty() {
        Word::Empty
    } else {
        Word::Pair(limit)
    })
}

/// Reads the collector machine's after-state: the flip decides which
/// semispace is working, the free pointer sits after the relocated
/// data, and the relocated root list hands each root its forwarded
/// word.
fn finish_collection(
    mem: &SharedMemory,
    machine: &Machine,
    root_count: usize,
) -> Result<Vec<Word>, Fault> {
    let new_working = VectorBase::of(&machine.get_register("the-cars")?)
        .map(VectorBase::space)
        .ok_or_else(|| Fault::Op {
            op: "collect".to_owned(),
            message: "the collector's vector registers lost their bases".to_owned(),
            step: 0,
        })?;
    let free_word = word_of(&machine.get_register("free")?).map_err(|e| op_fault("collect", &e))?;
    let relocated_root =
        word_of(&machine.get_register("root")?).map_err(|e| op_fault("collect", &e))?;
    {
        let mut space = mem.borrow_mut();
        space.working = new_working;
        space.free = index_part(&free_word).map_err(|e| op_fault("collect", &e))?;
        space.collections += 1;
    }
    let mut forwarded = Vec::with_capacity(root_count);
    let mut cursor = relocated_root;
    while let Word::Pair(cell) = cursor {
        let space = mem.borrow();
        let working = &space.spaces[space.working];
        forwarded.push(read_cell("collect", &working.cars, cell)?);
        cursor = read_cell("collect", &working.cdrs, cell)?;
    }
    Ok(forwarded)
}

/// A fault carrying only the collector's message.
fn message_fault(op: &'static str, message: String) -> Fault {
    Fault::Op {
        op: op.to_owned(),
        message,
        step: 0,
    }
}

/// The named semispace vector a register value must hold.
fn base_of(op: &'static str, value: &Value) -> Result<VectorBase, Fault> {
    VectorBase::of(value).ok_or_else(|| {
        message_fault(
            op,
            format!(
                "{} names no semispace vector",
                sicp_runtime::display_value(value)
            ),
        )
    })
}

/// The index part of a register value: the collector's numeric view
/// of a pointer, the address arithmetic of 5.3.1.
fn index_of(op: &'static str, value: &Value) -> Result<usize, Fault> {
    let word = word_of(value).map_err(|e| op_fault(op, &e))?;
    index_part(&word).map_err(|e| op_fault(op, &e))
}

/// One word out of a semispace vector.
fn read_cell(op: &'static str, vector: &[Word], index: usize) -> Result<Word, Fault> {
    vector
        .get(index)
        .cloned()
        .ok_or_else(|| op_fault(op, &MemoryFault::IndexOutOfRange { op }))
}

/// The book's `vector-ref`: one word out of the named semispace
/// vector.
fn vector_ref_handler(mem: SharedMemory) -> OpHandler {
    Rc::new(move |_machine, args| {
        let [base, index] = args else {
            return Err(arity("vector-ref", 2, args.len()));
        };
        let base = base_of("vector-ref", base)?;
        let index = index_of("vector-ref", index)?;
        let memory = mem.borrow();
        let space = &memory.spaces[base.space()];
        let vector = if base.is_cars() {
            &space.cars
        } else {
            &space.cdrs
        };
        read_cell("vector-ref", vector, index).map(Value::from)
    })
}

/// The book's `vector-set!`: stores one word into the named
/// semispace vector. The relocation target must stay inside the data
/// area, since the strip holds no collected data; the working space
/// keeps its strip, where the root list planted for the collection
/// takes its broken hearts.
fn vector_set_handler(mem: SharedMemory) -> OpHandler {
    Rc::new(move |_machine, args| {
        let [base, index, value] = args else {
            return Err(arity("vector-set!", 3, args.len()));
        };
        let base = base_of("vector-set!", base)?;
        let index = index_of("vector-set!", index)?;
        let value = word_of(value).map_err(|e| op_fault("vector-set!", &e))?;
        let mut memory = mem.borrow_mut();
        if base.space() != memory.working && index >= memory.data_limit() {
            return Err(message_fault(
                "vector-set!",
                "the collection does not fit the data area".to_owned(),
            ));
        }
        let space = &mut memory.spaces[base.space()];
        let vector = if base.is_cars() {
            &mut space.cars
        } else {
            &mut space.cdrs
        };
        let slot = vector.get_mut(index).ok_or_else(|| {
            op_fault(
                "vector-set!",
                &MemoryFault::IndexOutOfRange { op: "vector-set!" },
            )
        })?;
        *slot = value;
        Ok(Value::sym("done"))
    })
}

/// The scan/free comparison as index-part arithmetic.
fn index_compare(_machine: &mut Machine, args: &[Value]) -> Result<Value, Fault> {
    let [left, right] = args else {
        return Err(arity("=", 2, args.len()));
    };
    Ok(Value::Bool(index_of("=", left)? == index_of("=", right)?))
}

/// The scan/free increment as index-part arithmetic: the collector
/// only ever bumps the free or the scan pointer, so the answer is
/// the bumped pointer.
fn index_increment(_machine: &mut Machine, args: &[Value]) -> Result<Value, Fault> {
    let [left, right] = args else {
        return Err(arity("+", 2, args.len()));
    };
    let sum = index_of("+", left)?.checked_add(index_of("+", right)?);
    let sum = sum.ok_or_else(|| message_fault("+", "the index part overflowed".to_owned()))?;
    Ok(Value::from(Word::Pair(sum)))
}

/// The collector's own operations: the two vector primitives over the
/// named bases, the scan/free comparison and increment as the
/// index-part arithmetic of 5.3.1, and the two low-level predicates
/// the book's footnote names.
fn collector_operations(mem: &SharedMemory) -> Vec<(&'static str, OpHandler)> {
    let mut ops: Vec<(&'static str, OpHandler)> = vec![
        ("vector-ref", vector_ref_handler(Rc::clone(mem))),
        ("vector-set!", vector_set_handler(Rc::clone(mem))),
        ("=", Rc::new(index_compare)),
        ("+", Rc::new(index_increment)),
    ];
    for (name, test) in [
        ("pointer-to-pair?", is_pair as fn(&Word) -> bool),
        ("broken-heart?", is_broken_heart),
    ] {
        ops.push((
            name,
            Rc::new(move |_machine, args| {
                let words = words_of(name, args)?;
                let [word] = words.as_slice() else {
                    return Err(arity(name, 1, args.len()));
                };
                Ok(Value::Bool(test(word)))
            }),
        ));
    }
    ops
}

fn is_broken_heart(word: &Word) -> bool {
    matches!(word, Word::BrokenHeart)
}

/// The book's controller listing of 5.3.2, with the constants given
/// the collector's types: the `(const 0)` initializers name the pair
/// pointer `p0` -- free and scan are pair pointers -- and `(const
/// broken-heart)` is the moved tag.
fn collector_datums() -> Result<Vec<Value>, Fault> {
    const CONTROLLER: &str = "
begin-garbage-collection
  (assign free (const 0))
  (assign scan (const 0))
  (assign old (reg root))
  (assign relocate-continue (label reassign-root))
  (goto (label relocate-old-result-in-new))
reassign-root
  (assign root (reg new))
  (goto (label gc-loop))
gc-loop
  (test (op =) (reg scan) (reg free))
  (branch (label gc-flip))
  (assign old (op vector-ref) (reg new-cars) (reg scan))
  (assign relocate-continue (label update-car))
  (goto (label relocate-old-result-in-new))
update-car
  (perform (op vector-set!) (reg new-cars) (reg scan) (reg new))
  (assign old (op vector-ref) (reg new-cdrs) (reg scan))
  (assign relocate-continue (label update-cdr))
  (goto (label relocate-old-result-in-new))
update-cdr
  (perform (op vector-set!) (reg new-cdrs) (reg scan) (reg new))
  (assign scan (op +) (reg scan) (const 1))
  (goto (label gc-loop))
relocate-old-result-in-new
  (test (op pointer-to-pair?) (reg old))
  (branch (label pair))
  (assign new (reg old))
  (goto (reg relocate-continue))
pair
  (assign oldcr (op vector-ref) (reg the-cars) (reg old))
  (test (op broken-heart?) (reg oldcr))
  (branch (label already-moved))
  (assign new (reg free))
  (assign free (op +) (reg free) (const 1))
  (perform (op vector-set!) (reg new-cars) (reg new) (reg oldcr))
  (assign oldcr (op vector-ref) (reg the-cdrs) (reg old))
  (perform (op vector-set!) (reg new-cdrs) (reg new) (reg oldcr))
  (perform (op vector-set!) (reg the-cars) (reg old) (const broken-heart))
  (perform (op vector-set!) (reg the-cdrs) (reg old) (reg new))
  (goto (reg relocate-continue))
already-moved
  (assign new (op vector-ref) (reg the-cdrs) (reg old))
  (goto (reg relocate-continue))
gc-flip
  (assign temp (reg the-cdrs))
  (assign the-cdrs (reg new-cdrs))
  (assign new-cdrs (reg temp))
  (assign temp (reg the-cars))
  (assign the-cars (reg new-cars))
  (assign new-cars (reg temp))";

    fn retype(value: &Value) -> Result<Value, Fault> {
        let Value::Pair(_) = value else {
            return Ok(match value {
                Value::Int(0) => Value::from(Word::Pair(0)),
                Value::Sym(name) if name.as_ref() == "broken-heart" => {
                    Value::from(Word::BrokenHeart)
                }
                _ => value.clone(),
            });
        };
        let items = value
            .list_items()
            .map_err(|error| Fault::Parse(error.to_string()))?;
        let retyped = items.iter().map(retype).collect::<Result<Vec<_>, _>>()?;
        Ok(Value::list(retyped))
    }

    let datums = read_program(CONTROLLER).map_err(|error| Fault::Parse(error.to_string()))?;
    datums.iter().map(retype).collect()
}

#[cfg(test)]
mod tests {
    //! The module's own proofs: the primitives of 5.3.1 over one
    //! memory, and the stop-and-copy collector of 5.3.2 against its
    //! relocation, flip, and fault contracts.

    use super::*;

    /// A memory with six data cells and a four-cell reserved strip,
    /// allocation opening at `p1` as in the exercise drawing.
    fn memory() -> SharedMemory {
        Rc::new(RefCell::new(Memory::new(10, 4, 1)))
    }

    /// The proper list of the given numbers, the book's `(list n ...)`
    /// built bottom up, so the innermost cell allocates first.
    fn num_list(memory: &SharedMemory, items: &[i128]) -> Result<Word, MemoryFault> {
        let mut word = Word::Empty;
        for item in items.iter().rev() {
            word = memory.borrow_mut().cons(Word::Num(*item), word)?;
        }
        Ok(word)
    }

    #[test]
    fn primitives_read_and_write_the_working_space() -> Result<(), MemoryFault> {
        let memory = memory();
        let pair = memory.borrow_mut().cons(Word::Num(1), Word::Num(2))?;
        assert_eq!(memory.borrow().car(&pair)?, Word::Num(1));
        assert_eq!(memory.borrow().cdr(&pair)?, Word::Num(2));
        memory.borrow_mut().set_car(&pair, Word::Sym("a".into()))?;
        let tail = num_list(&memory, &[3, 4])?;
        memory.borrow_mut().set_cdr(&pair, tail)?;
        assert_eq!(memory.borrow().car(&pair)?, Word::Sym("a".into()));
        assert_eq!(memory.borrow().write(&pair)?, "(a 3 4)");
        assert_eq!(pair, Word::Pair(1));
        // The (3 4) tail took the two cells after the pair.
        assert_eq!(memory.borrow().free_word(), Word::Pair(4));
        Ok(())
    }

    #[test]
    fn selectors_refuse_a_non_pair() {
        let memory = memory();
        let err = memory.borrow().car(&Word::Num(4)).unwrap_err();
        assert_eq!(
            err,
            MemoryFault::NotPair {
                op: "car",
                word: "n4".to_owned()
            }
        );
    }

    #[test]
    fn allocation_stops_at_the_reserved_strip() -> Result<(), MemoryFault> {
        let memory = memory();
        // Six data cells, allocation opening at p1: five conses fit.
        for _ in 0..5 {
            memory.borrow_mut().cons(Word::Num(0), Word::Empty)?;
        }
        assert_eq!(
            memory.borrow_mut().cons(Word::Num(1), Word::Empty),
            Err(MemoryFault::Exhausted)
        );
        Ok(())
    }

    /// The live structure of the collection proofs: `x` is the pair
    /// `(1 . 2)` and `y` is `(x x)`, two pointers into one shared
    /// cell, with one unreachable pair of garbage beside them.
    fn shared_with_garbage(memory: &SharedMemory) -> Result<(Word, Word), MemoryFault> {
        let garbage = memory.borrow_mut().cons(Word::Num(9), Word::Num(9))?;
        let x = memory.borrow_mut().cons(Word::Num(1), Word::Num(2))?;
        // x names the shared cell: both list elements copy the same
        // pointer, and the test hands the original to the collector.
        let inner = memory.borrow_mut().cons(x.clone(), Word::Empty)?;
        let y = memory.borrow_mut().cons(x.clone(), inner)?;
        assert_eq!(garbage, Word::Pair(1));
        Ok((x, y))
    }

    #[test]
    fn collect_relocates_the_live_data_and_flips() -> Result<(), Box<dyn std::error::Error>> {
        let memory = memory();
        let (x, y) = shared_with_garbage(&memory).map_err(|e| op_fault("test", &e))?;
        let forwarded = collect(&memory, &[x.clone(), y])?;
        // The roots keep their structure across the move, sharing
        // included: both elements of y are the same relocated x.
        assert_eq!(forwarded.len(), 2);
        let (x2, y2) = (forwarded[0].clone(), forwarded[1].clone());
        assert_ne!(x2, x);
        assert_eq!(memory.borrow().write(&x2)?, "(1 . 2)");
        assert_eq!(memory.borrow().write(&y2)?, "((1 . 2) (1 . 2))");
        assert_eq!(memory.borrow().car(&y2)?, x2);
        // The flip made semispace 1 the working one, allocation
        // resumes after the relocated data, and the counter moved.
        {
            let borrowed = memory.borrow();
            assert_eq!(borrowed.working(), 1);
            assert_eq!(borrowed.collections(), 1);
            // The copy moves the two root-list cells and the three
            // live pairs: free sits right after the fifth.
            assert_eq!(borrowed.free_word(), Word::Pair(5));
        }
        let fresh = memory.borrow_mut().cons(Word::Num(7), Word::Empty)?;
        assert_eq!(fresh, Word::Pair(5));
        Ok(())
    }

    #[test]
    fn collect_marks_broken_hearts_and_drops_the_garbage() -> Result<(), Fault> {
        let memory = memory();
        let (x, y) = shared_with_garbage(&memory).map_err(|e| op_fault("test", &e))?;
        collect(&memory, &[x, y])?;
        // The new working semispace holds the relocated data only:
        // the n9 garbage pair was never copied. The old semispace's
        // moved cells wear broken hearts with their forwarding
        // addresses in the cdr.
        assert!(!memory.borrow().dump().contains("n9"));
        let space = &memory.borrow().spaces[0];
        for cell in [2, 3, 4, 6, 7] {
            assert_eq!(space.cars[cell], Word::BrokenHeart, "cell {cell}");
        }
        assert!(matches!(space.cdrs[2], Word::Pair(_)));
        Ok(())
    }

    #[test]
    fn collect_runs_again_on_the_flipped_spaces() -> Result<(), Box<dyn std::error::Error>> {
        let memory = memory();
        let (x, y) = shared_with_garbage(&memory).map_err(|e| op_fault("test", &e))?;
        let first = collect(&memory, &[x.clone(), y])?;
        let second = collect(&memory, &[first[0].clone()])?;
        assert_eq!(memory.borrow().collections(), 2);
        assert_eq!(memory.borrow().working(), 0);
        assert_eq!(memory.borrow().write(&second[0])?, "(1 . 2)");
        Ok(())
    }

    #[test]
    fn collect_frees_an_exhausted_memory() -> Result<(), Box<dyn std::error::Error>> {
        let memory = memory();
        memory
            .borrow_mut()
            .cons(Word::Num(9), Word::Num(9))
            .map_err(|e| op_fault("test", &e))?;
        let live = num_list(&memory, &[1, 2, 3]).map_err(|e| op_fault("test", &e))?;
        memory
            .borrow_mut()
            .cons(Word::Num(8), Word::Num(8))
            .map_err(|e| op_fault("test", &e))?;
        assert_eq!(
            memory.borrow_mut().cons(Word::Num(0), Word::Empty),
            Err(MemoryFault::Exhausted)
        );
        let forwarded = collect(&memory, &[live])?;
        let word = forwarded[0].clone();
        assert_eq!(memory.borrow().write(&word)?, "(1 2 3)");
        // The two garbage pairs were not copied: allocation has room
        // again in the flipped space.
        memory.borrow_mut().cons(Word::Num(7), Word::Empty)?;
        Ok(())
    }

    #[test]
    fn collect_refuses_roots_past_the_strip() {
        let memory = memory();
        let (x, y) = shared_with_garbage(&memory).expect("structure");
        let z = memory
            .borrow_mut()
            .cons(x.clone(), y.clone())
            .expect("cons");
        let roots = vec![x.clone(), y.clone(), z, x, y];
        let err = collect(&memory, &roots).expect_err("too many roots");
        assert!(
            err.to_string()
                .contains("5 roots exceed the strip's 4 cells"),
            "unexpected fault: {err}"
        );
    }

    #[test]
    fn collect_refuses_a_collection_that_does_not_fit() {
        // Four live pairs plus a three-cell root list: seven cells
        // against a six-cell data area, the one overflow the root
        // list itself can produce.
        let memory = memory();
        let (x, y) = shared_with_garbage(&memory).expect("structure");
        let z = memory
            .borrow_mut()
            .cons(x.clone(), y.clone())
            .expect("cons");
        let err = collect(&memory, &[x, y, z]).expect_err("does not fit");
        assert!(
            err.to_string().contains("does not fit"),
            "unexpected fault: {err}"
        );
    }
}
