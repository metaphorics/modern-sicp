// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.22: two register machines
//! over the list-structure memory. The `append` machine copies the
//! first list's cells, consing each car onto the appended tail; the
//! `append!` machine walks to the last pair and splices the second
//! list in with `set-cdr!`, allocating nothing.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_2::{Fault, Machine, make_machine_from_datums};
use ch05::sec_5_3::{Memory, SharedMemory, Word, memory_operations};
use sicp_runtime::{Value, read_program};

mod ex_5_22 {
    //! Exercise 5.22: design a register machine to implement
    //! `append` of 3.12, which appends two lists to form a new list,
    //! and one to implement `append!`, which splices two lists
    //! together, assuming the list-structure memory operations are
    //! available as primitive operations.

    use super::*;

    /// Builds a machine whose operations are the memory primitives,
    /// assembled from the book's controller datums.
    fn machine(
        memory: &SharedMemory,
        registers: &[&str],
        controller: &str,
    ) -> Result<Machine, Fault> {
        let operations = memory_operations(memory);
        let datums = read_program(controller).map_err(|error| Fault::Parse(error.to_string()))?;
        make_machine_from_datums(registers, &operations, &datums)
    }

    /// The proper list of the numbers, built bottom up: the
    /// innermost cell allocates first.
    fn num_list(memory: &SharedMemory, items: &[i128]) -> Word {
        let mut word = Word::Empty;
        for item in items.iter().rev() {
            word = memory
                .borrow_mut()
                .cons(Word::Num(*item), word)
                .expect("room for the lists");
        }
        word
    }

    /// The `append` machine of 3.12: recursion down the cdr side,
    /// then one cons per cell of the first list on the way back.
    fn append_machine(memory: &SharedMemory) -> Machine {
        machine(
            memory,
            &["x", "y", "result", "car-val", "continue"],
            "
begin-append
  (assign continue (label append-done))
append-loop
  (test (op null?) (reg x))
  (branch (label null-x))
  (save continue)
  (save x)
  (assign x (op cdr) (reg x))
  (assign continue (label after-cdr))
  (goto (label append-loop))
after-cdr
  (restore x)
  (restore continue)
  (assign car-val (op car) (reg x))
  (assign result (op cons) (reg car-val) (reg result))
  (goto (reg continue))
null-x
  (assign result (reg y))
  (goto (reg continue))
append-done",
        )
        .expect("the controller assembles")
    }

    /// The `append!` machine of 3.12: walk to the last pair of the
    /// first list and splice the second list onto it. The result is
    /// the first list's own head, unchanged as a pointer.
    fn append_bang_machine(memory: &SharedMemory) -> Machine {
        machine(
            memory,
            &["x", "y", "temp"],
            "
begin-append!
last-pair-loop
  (assign temp (op cdr) (reg x))
  (test (op null?) (reg temp))
  (branch (label found-last))
  (assign x (reg temp))
  (goto (label last-pair-loop))
found-last
  (perform (op set-cdr!) (reg x) (reg y))
append!-done",
        )
        .expect("the controller assembles")
    }

    /// Appends `(1 2 3)` and `(4 5)` with the first machine: the
    /// result reads as one list of five, built from three fresh
    /// cells, and the last of them is the second list itself, shared
    /// rather than copied.
    #[test]
    fn ex_5_22_append_copies_the_first_list() {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(16, 4, 1)));
        let x = num_list(&memory, &[1, 2, 3]);
        let y = num_list(&memory, &[4, 5]);
        let before_free = memory.borrow().free_word();
        let mut machine = append_machine(&memory);
        machine.set_register("x", Value::from(x.clone())).unwrap();
        machine.set_register("y", Value::from(y.clone())).unwrap();
        machine.start().unwrap();
        let result = match ch05::sec_5_3::word_of(&machine.get_register("result").unwrap()).unwrap()
        {
            word @ Word::Pair(_) => word,
            other => panic!("result is not a pair: {other:?}"),
        };
        assert_eq!(memory.borrow().write(&result).unwrap(), "(1 2 3 4 5)");
        // The first list's own cells are untouched by the copy.
        assert_eq!(memory.borrow().write(&x).unwrap(), "(1 2 3)");
        // Three cells were allocated, and the last new cell's tail
        // is y itself: the second list is shared, not copied.
        assert_eq!(
            memory.borrow().free_word(),
            Word::Pair(before_free_index(&before_free) + 3)
        );
        let fourth = walk(&memory, &result, 3);
        assert_eq!(fourth, y);
    }

    /// Splices `(1 2 3)` and `(4 5)` with the second machine: the
    /// first list's last pair now cdrs into the second list, no cell
    /// was allocated, and the two arguments have become one list of
    /// five.
    #[test]
    fn ex_5_22_append_bang_splices_the_lists() {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(16, 4, 1)));
        let x = num_list(&memory, &[1, 2, 3]);
        let y = num_list(&memory, &[4, 5]);
        let before_free = memory.borrow().free_word();
        let mut machine = append_bang_machine(&memory);
        machine.set_register("x", Value::from(x.clone())).unwrap();
        machine.set_register("y", Value::from(y.clone())).unwrap();
        machine.start().unwrap();
        // The head is the same pointer, the printout is the spliced
        // list, and nothing was allocated.
        assert_eq!(memory.borrow().write(&x).unwrap(), "(1 2 3 4 5)");
        assert_eq!(memory.borrow().free_word(), before_free);
        // The splice: the cell after the 3 is the second list's own
        // head cell.
        let third = walk(&memory, &x, 2);
        let spliced = memory.borrow().cdr(&third).unwrap();
        assert_eq!(spliced, y);
    }

    /// The word at the nth position of a proper list.
    fn walk(memory: &SharedMemory, word: &Word, n: usize) -> Word {
        let mut cursor = word.clone();
        for _ in 0..n {
            cursor = memory.borrow().cdr(&cursor).expect("a proper list");
        }
        cursor
    }

    /// The index a free pointer word names.
    fn before_free_index(word: &Word) -> usize {
        match word {
            Word::Pair(index) => *index,
            other => panic!("free is always a pair pointer: {other:?}"),
        }
    }
}
