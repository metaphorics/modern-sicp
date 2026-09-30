// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.3

//! Section 5.3: memory and garbage collection. The heap is a pair of
//! `Vec`s addressed by checked `usize` indexes — a [`Word`] is an
//! integer or a heap address, never a host pointer — and the
//! collector is an explicit stop-and-copy loop with a work list, as
//! the section draws it.

use std::fmt::Write as _;

/// A shared memory handle the lesson machines close over when their
/// `perform` operations allocate and traverse cells.
pub type SharedMemory = std::rc::Rc<std::cell::RefCell<Memory>>;

/// One memory word: an integer datum or a heap address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Word {
    /// An integer datum.
    Int(i64),
    /// A heap address.
    Addr(usize),
}

/// A memory fault (grammar §5.3): heap addresses are checked indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryFault {
    /// An address outside the allocated region.
    OutOfBounds,
    /// Allocation with no free pair.
    OutOfMemory,
}

impl std::fmt::Display for MemoryFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OutOfBounds => "address outside the allocated region",
            Self::OutOfMemory => "no free pair to allocate",
        })
    }
}

impl std::error::Error for MemoryFault {}

/// The chunked pair memory of section 5.3: `the-cars`, `the-cdrs`,
/// the free pointer, and the collection statistics.
#[derive(Debug, Clone)]
pub struct Memory {
    cars: Vec<Word>,
    cdrs: Vec<Word>,
    free: usize,
    collections: u32,
    root_capacity: usize,
    /// The boundary between the two pair semispaces.
    split: usize,
    /// Which semispace allocation currently draws from.
    upper: bool,
}

impl Memory {
    /// Builds one memory with `size` pair cells and `root_capacity`
    /// root slots at the front of the heap.
    #[must_use]
    pub fn new(size: usize, root_capacity: usize) -> Self {
        let pair_region = size.saturating_sub(root_capacity);
        Self {
            cars: vec![Word::Int(0); size],
            cdrs: vec![Word::Int(0); size],
            free: root_capacity,
            collections: 0,
            root_capacity,
            split: root_capacity + pair_region / 2,
            upper: false,
        }
    }

    /// The exclusive end of the semispace allocation draws from.
    fn space_end(&self) -> usize {
        if self.upper {
            self.cars.len()
        } else {
            self.split
        }
    }

    /// The next free address.
    #[must_use]
    pub fn free(&self) -> usize {
        self.free
    }

    /// The number of collections performed.
    #[must_use]
    pub fn collections(&self) -> u32 {
        self.collections
    }

    /// Allocates one pair.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfMemory`] when the heap is full.
    pub fn cons(&mut self, car: Word, cdr: Word) -> Result<Word, MemoryFault> {
        if self.free >= self.space_end() {
            return Err(MemoryFault::OutOfMemory);
        }
        let address = self.free;
        self.cars[address] = car;
        self.cdrs[address] = cdr;
        self.free += 1;
        Ok(Word::Addr(address))
    }

    /// Reads one pair's `car`.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfBounds`] for a non-pair or absent address.
    pub fn car(&self, pair: Word) -> Result<Word, MemoryFault> {
        let Word::Addr(address) = pair else {
            return Err(MemoryFault::OutOfBounds);
        };
        self.cars
            .get(address)
            .copied()
            .ok_or(MemoryFault::OutOfBounds)
    }

    /// Reads one pair's `cdr`.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfBounds`] for a non-pair or absent address.
    pub fn cdr(&self, pair: Word) -> Result<Word, MemoryFault> {
        let Word::Addr(address) = pair else {
            return Err(MemoryFault::OutOfBounds);
        };
        self.cdrs
            .get(address)
            .copied()
            .ok_or(MemoryFault::OutOfBounds)
    }

    /// Mutates one pair's `car`.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfBounds`] for a non-pair or absent address.
    pub fn set_car(&mut self, pair: Word, value: Word) -> Result<(), MemoryFault> {
        let Word::Addr(address) = pair else {
            return Err(MemoryFault::OutOfBounds);
        };
        let slot = self.cars.get_mut(address).ok_or(MemoryFault::OutOfBounds)?;
        *slot = value;
        Ok(())
    }

    /// Mutates one pair's `cdr`.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfBounds`] for a non-pair or absent address.
    pub fn set_cdr(&mut self, pair: Word, value: Word) -> Result<(), MemoryFault> {
        let Word::Addr(address) = pair else {
            return Err(MemoryFault::OutOfBounds);
        };
        let slot = self.cdrs.get_mut(address).ok_or(MemoryFault::OutOfBounds)?;
        *slot = value;
        Ok(())
    }

    /// Renders one word the way the section prints heap contents.
    #[must_use]
    pub fn write(&self, word: Word) -> String {
        match word {
            Word::Int(value) => value.to_string(),
            Word::Addr(address) => format!("[{address}]"),
        }
    }

    /// The whole heap as one dump: each live pair on its own line.
    #[must_use]
    pub fn dump(&self) -> String {
        let mut out = String::new();
        for address in self.root_capacity..self.free {
            // `String`'s formatting writer is infallible.
            let _ = writeln!(
                &mut out,
                "{address}: {} . {}",
                self.write(self.cars[address]),
                self.write(self.cdrs[address])
            );
        }
        out
    }

    /// Stop-and-copy collection: relocate every pair reachable from
    /// `roots`, preserving all pair structure, and answer the updated
    /// roots in the same order.
    ///
    /// # Errors
    /// [`MemoryFault::OutOfBounds`] when a root names no live cell.
    pub fn collect(&mut self, roots: &[Word]) -> Result<Vec<Word>, MemoryFault> {
        self.collections += 1;
        // Stop-and-copy across two semispaces: flip first so every
        // live pair is copied into the empty half and the previous
        // half is reclaimed in the same phase.
        self.upper = !self.upper;
        self.free = if self.upper {
            self.split
        } else {
            self.root_capacity
        };
        let mut relocated: Vec<(usize, usize)> = Vec::new();
        let mut updated = Vec::with_capacity(roots.len());
        for root in roots {
            updated.push(self.relocate(*root, &mut relocated)?);
        }
        Ok(updated)
    }

    fn relocate(
        &mut self,
        word: Word,
        relocated: &mut Vec<(usize, usize)>,
    ) -> Result<Word, MemoryFault> {
        let Word::Addr(old) = word else {
            return Ok(word);
        };
        if old < self.root_capacity {
            return Ok(word);
        }
        if let Some((_, new)) = relocated.iter().find(|(from, _)| *from == old) {
            return Ok(Word::Addr(*new));
        }
        let car = self
            .cars
            .get(old)
            .copied()
            .ok_or(MemoryFault::OutOfBounds)?;
        let cdr = self
            .cdrs
            .get(old)
            .copied()
            .ok_or(MemoryFault::OutOfBounds)?;
        let new = self.free;
        if new >= self.space_end() {
            return Err(MemoryFault::OutOfMemory);
        }
        self.free += 1;
        self.cars[new] = car;
        self.cdrs[new] = cdr;
        relocated.push((old, new));
        // The forwarding work list: relocate the children after the
        // parent has its new home.
        let relocated_first = self.relocate(car, relocated)?;
        let relocated_rest = self.relocate(cdr, relocated)?;
        self.cars[new] = relocated_first;
        self.cdrs[new] = relocated_rest;
        Ok(Word::Addr(new))
    }
}

/// The memory operations of the section's machines (exercises 5.21 and
/// 5.22): each closes over the shared heap and takes its arguments as
/// machine values, where an address is carried in the integer cell.
#[must_use]
pub fn memory_operations(mem: &SharedMemory) -> Vec<(String, crate::sec_5_2::OpHandler)> {
    use crate::sec_5_2::OpHandler;
    let mut ops: Vec<(String, OpHandler)> = Vec::new();
    {
        let mem = std::rc::Rc::clone(mem);
        ops.push((
            "cons".to_owned(),
            std::rc::Rc::new(move |args: &[i64]| {
                let [car, cdr] = args else {
                    return Err(crate::sec_5_2::Fault::UndefinedOperation("cons".to_owned()));
                };
                let cell = mem.borrow_mut().cons(word_of(*car), word_of(*cdr));
                let address = addr_of(cell.map_err(fault_of)?);
                i64::try_from(address).map_err(|_| crate::sec_5_2::Fault::Overflow("heap"))
            }),
        ));
    }
    {
        let mem = std::rc::Rc::clone(mem);
        ops.push((
            "car".to_owned(),
            std::rc::Rc::new(move |args: &[i64]| {
                let [pair] = args else {
                    return Err(crate::sec_5_2::Fault::UndefinedOperation("car".to_owned()));
                };
                let cell = mem.borrow().car(word_of(*pair)).map_err(fault_of)?;
                Ok(value_of(cell))
            }),
        ));
    }
    {
        let mem = std::rc::Rc::clone(mem);
        ops.push((
            "cdr".to_owned(),
            std::rc::Rc::new(move |args: &[i64]| {
                let [pair] = args else {
                    return Err(crate::sec_5_2::Fault::UndefinedOperation("cdr".to_owned()));
                };
                let cell = mem.borrow().cdr(word_of(*pair)).map_err(fault_of)?;
                Ok(value_of(cell))
            }),
        ));
    }
    {
        let mem = std::rc::Rc::clone(mem);
        ops.push((
            "set-car".to_owned(),
            std::rc::Rc::new(move |args: &[i64]| {
                let [pair, value] = args else {
                    return Err(crate::sec_5_2::Fault::UndefinedOperation(
                        "set-car".to_owned(),
                    ));
                };
                mem.borrow_mut()
                    .set_car(word_of(*pair), word_of(*value))
                    .map_err(fault_of)?;
                Ok(0)
            }),
        ));
    }
    {
        let mem = std::rc::Rc::clone(mem);
        ops.push((
            "set-cdr".to_owned(),
            std::rc::Rc::new(move |args: &[i64]| {
                let [pair, value] = args else {
                    return Err(crate::sec_5_2::Fault::UndefinedOperation(
                        "set-cdr".to_owned(),
                    ));
                };
                mem.borrow_mut()
                    .set_cdr(word_of(*pair), word_of(*value))
                    .map_err(fault_of)?;
                Ok(0)
            }),
        ));
    }
    ops.push((
        "pair?".to_owned(),
        std::rc::Rc::new(|args: &[i64]| {
            let [value] = args else {
                return Err(crate::sec_5_2::Fault::UndefinedOperation(
                    "pair?".to_owned(),
                ));
            };
            // The empty list is the reserved address 0; every pair
            // cell lives at an address >= 1.
            Ok(i64::from(*value > 0))
        }),
    ));
    ops.push((
        "null?".to_owned(),
        std::rc::Rc::new(|args: &[i64]| {
            let [value] = args else {
                return Err(crate::sec_5_2::Fault::UndefinedOperation(
                    "null?".to_owned(),
                ));
            };
            Ok(i64::from(*value == 0))
        }),
    ));
    ops
}

// The memory-word discipline of the section: the empty list is the
// reserved address 0, pair cells occupy addresses >= 1 (construct
// memories with `root_capacity >= 1`), and an integer datum n is
// carried in the machine word -n-1 (so data 0 is -1, data 1 is -2).
// `pair?` is `word > 0`, `null?` is `word == 0`.

fn word_of(value: i64) -> Word {
    if value <= -1 {
        // Integer datum n is carried as the word -n-1: data 0 is -1,
        // data 1 is -2, and so on.
        Word::Int(-value - 1)
    } else {
        Word::Addr(usize::try_from(value).unwrap_or(usize::MAX))
    }
}

fn value_of(word: Word) -> i64 {
    match word {
        Word::Int(value) => -value - 1,
        Word::Addr(address) => i64::try_from(address).unwrap_or(i64::MAX),
    }
}

fn addr_of(word: Word) -> usize {
    match word {
        Word::Addr(address) => address,
        Word::Int(value) => usize::try_from(value).unwrap_or(0),
    }
}

fn fault_of(fault: MemoryFault) -> crate::sec_5_2::Fault {
    match fault {
        MemoryFault::OutOfBounds => {
            crate::sec_5_2::Fault::UndefinedOperation("memory address".to_owned())
        }
        MemoryFault::OutOfMemory => crate::sec_5_2::Fault::Overflow("heap"),
    }
}
