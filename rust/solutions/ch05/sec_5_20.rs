// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.20: the three conses of
//! `(define x (cons 1 2))` and `(define y (list x x))` through the
//! allocation path of the list-structure memory, `free` opening at
//! `p1` as the exercise states.
//!
//! The box-and-pointer drawing the exercise asks for, of the
//! structure the tests below rebuild and check cell by cell:
//!
//! ```text
//!         +---+---+
//!    x    | 1 | 2 |
//!         +---+---+
//!
//!         +---+---+     +---+---+
//!    y    | * | *-+---> | * | / |
//!         +-|-+---+     +-|-+---+
//!           |             |
//!           v             v
//!         (the x box above, shared by both)
//! ```
//!
//! and its memory-vector form, which `memory_vector` renders from
//! the live heap.

use ch05::sec_5_3::{Memory, MemoryFault, SharedMemory, Word};

/// One word drawn as a box-and-pointer compartment: a datum as
/// itself, the empty list as `/`, and a pair cell as `*` with its
/// address named.
fn slot(word: Word) -> String {
    match word {
        Word::Int(value) => value.to_string(),
        Word::Addr(0) => "/".to_owned(),
        Word::Addr(address) => format!("*{address}"),
    }
}

/// One cell's two drawn compartments and its column index. The
/// reserved cell 0 is the empty list and is drawn as such.
fn cell_column(memory: &Memory, address: usize) -> (String, String, String) {
    let column = format!("{address:>3}");
    if address == 0 {
        return (column, "  /".to_owned(), "  /".to_owned());
    }
    let cell = Word::Addr(address);
    let car = memory.car(cell).unwrap_or(Word::Addr(0));
    let cdr = memory.cdr(cell).unwrap_or(Word::Addr(0));
    (
        column,
        format!("{:>3}", slot(car)),
        format!("{:>3}", slot(cdr)),
    )
}

/// The memory-vector representation (Figure 5.14): one column per
/// cell, the two vector rows, and the three pointers the exercise
/// asks about.
fn memory_vector(memory: &Memory, x: Word, y: Word) -> String {
    let mut index = String::from("index    ");
    let mut head_row = String::from("the-cars ");
    let mut tail_row = String::from("the-cdrs ");
    for address in 0..memory.free() {
        let (column, car, cdr) = cell_column(memory, address);
        index.push_str(&column);
        head_row.push_str(&car);
        tail_row.push_str(&cdr);
    }
    format!(
        "{index}\n{head_row}\n{tail_row}\nx = {}, y = {}, free = {}",
        slot(x),
        slot(y),
        slot(Word::Addr(memory.free()))
    )
}

/// The two definitions of the exercise in allocation order: the pair
/// `(1 . 2)` first, then `(list x x)`, which is
/// `(cons x (cons x '()))` and so allocates its inner cons before
/// the outer cell that names it.
fn run() -> Result<(SharedMemory, Word, Word), MemoryFault> {
    let memory: SharedMemory = SharedMemory::new(std::cell::RefCell::new(Memory::new(8, 1)));
    let x = memory.borrow_mut().cons(Word::Int(1), Word::Int(2))?;
    let inner = memory.borrow_mut().cons(x, Word::Addr(0))?;
    let y = memory.borrow_mut().cons(x, inner)?;
    Ok((memory, x, y))
}

mod ex_5_20 {
    //! Exercise 5.20: draw the box-and-pointer and memory-vector
    //! representations of the structure produced by
    //! `(define x (cons 1 2))` and `(define y (list x x))`, with the
    //! free pointer initially `p1`, and answer what `free`, `x`, and
    //! `y` are.

    use super::*;

    /// The three answers: `free` ends at `p4`, `x` is the pointer
    /// `p1`, and `y` is the pointer `p3`.
    #[test]
    fn ex_5_20_free_and_the_two_pointers() -> Result<(), MemoryFault> {
        let (memory, x, y) = run()?;
        assert_eq!(x, Word::Addr(1));
        assert_eq!(y, Word::Addr(3));
        assert_eq!(memory.borrow().free(), 4);
        Ok(())
    }

    /// The sharing the box-and-pointer drawing shows as two arrows
    /// into one box: both elements of `y` are the same cell `p1`,
    /// proved by reading the second element back to `x` itself.
    #[test]
    fn ex_5_20_y_shares_x() -> Result<(), MemoryFault> {
        let (memory, x, y) = run()?;
        let heap = memory.borrow();
        let rest = heap.cdr(y)?;
        let second = heap.car(rest)?;
        assert_eq!(second, x);
        Ok(())
    }

    /// The memory-vector drawing, one column per cell: the reserved
    /// cell 0, the `(1 . 2)` pair at `p1`, the two elements of `y`
    /// at `p2` and `p3` both naming `p1`, and `free` stopping at
    /// `p4`.
    #[test]
    fn ex_5_20_memory_vector_drawing() -> Result<(), MemoryFault> {
        let (memory, x, y) = run()?;
        assert_eq!(
            memory_vector(&memory.borrow(), x, y),
            concat!(
                "index      0  1  2  3\n",
                "the-cars   /  1 *1 *1\n",
                "the-cdrs   /  2  / *2\n",
                "x = *1, y = *3, free = *4"
            )
        );
        Ok(())
    }
}
