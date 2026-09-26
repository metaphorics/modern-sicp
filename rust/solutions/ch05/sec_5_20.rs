// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.20: the three conses of
//! `(define x (cons 1 2))` and `(define y (list x x))` run through
//! the allocation path of the list-structure memory, free opening at
//! `p1` as the exercise states, the memory-vector drawing pinned as
//! the module renders it.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_3::{Memory, Word};

mod ex_5_20 {
    //! Exercise 5.20: draw the box-and-pointer and memory-vector
    //! representations of the structure produced by
    //! `(define x (cons 1 2))` and `(define y (list x x))`, with the
    //! free pointer initially `p1`, and answer what `free`, `x`, and
    //! `y` are.

    use super::*;

    /// A drawing-sized memory: eight cells per semispace, the top
    /// four reserved for a collector's root list, allocation opening
    /// at cell 1 so the drawing shows the book's blank cell 0.
    fn memory() -> Rc<RefCell<Memory>> {
        Rc::new(RefCell::new(Memory::new(8, 4, 1)))
    }

    /// Runs the exercise's two definitions in allocation order: the
    /// pair `(1 . 2)` first, then the innermost cons of `(list x x)`,
    /// whose cdr argument must exist before the outer cell can name
    /// it.
    fn run() -> (Rc<RefCell<Memory>>, Word, Word) {
        let memory = memory();
        let x = memory
            .borrow_mut()
            .cons(Word::Num(1), Word::Num(2))
            .expect("cell 1");
        // (list x x) is (cons x (cons x '())): the inner cons first.
        let inner = memory
            .borrow_mut()
            .cons(x.clone(), Word::Empty)
            .expect("cell 2");
        let y = memory.borrow_mut().cons(x.clone(), inner).expect("cell 3");
        (memory, x, y)
    }

    /// The three answers: free ends at `p4`, `x` is the pointer
    /// `p1`, and `y` is the pointer `p3`.
    #[test]
    fn ex_5_20_free_and_the_two_pointers() {
        let (memory, x, y) = run();
        assert_eq!(x, Word::Pair(1));
        assert_eq!(y, Word::Pair(3));
        assert_eq!(memory.borrow().free_word(), Word::Pair(4));
        assert_eq!(memory.borrow().write(&y).unwrap(), "((1 . 2) (1 . 2))");
    }

    /// The sharing the box-and-pointer drawing shows as two arrows
    /// into one box: both elements of `y` are the same cell `p1`,
    /// proved by reading the second element back to `x` itself.
    #[test]
    fn ex_5_20_y_shares_x() {
        let (memory, x, y) = run();
        let second = memory
            .borrow()
            .car(&memory.borrow().cdr(&y).unwrap())
            .unwrap();
        assert_eq!(second, x);
    }

    /// The memory-vector drawing, one column per cell: the blank
    /// cell 0, the `(1 . 2)` pair at `p1`, the two elements of `y`
    /// at `p2` and `p3` both naming `p1`, and free stopping at `p4`.
    #[test]
    fn ex_5_20_memory_vector_drawing() {
        let (memory, _, _) = run();
        assert_eq!(
            memory.borrow().dump(),
            "index    0   1   2   3   4   5   6   7\n\
             the-cars e0  n1  p1  p1  e0  e0  e0  e0\n\
             the-cdrs e0  n2  e0  p2  e0  e0  e0  e0"
        );
    }
}
