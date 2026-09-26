// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.21: two register machines
//! that count the leaves of a tree stored in the list-structure
//! memory. The first computes each subtree's count and joins the two
//! with `+`; the second threads one explicit counter through the
//! recursion the way the book's `count-iter` does. Both run on the
//! 5.2 simulator with the 5.3.1 memory primitives.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_2::{Fault, Machine, OpHandler, make_machine_from_datums};
use ch05::sec_5_3::{SharedMemory, Word, memory_operations};
use sicp_runtime::{Value, read_program};

mod ex_5_21 {
    //! Exercise 5.21: implement register machines for recursive
    //! `count-leaves`, and for recursive `count-leaves` with an
    //! explicit counter, assuming the list-structure memory
    //! operations are available as machine primitives.

    use super::*;

    /// The counting addition over number registers.
    fn plus() -> OpHandler {
        Rc::new(|_machine, args| match args {
            [Value::Int(left), Value::Int(right)] => Ok(Value::Int(left + right)),
            _ => Err(Fault::Op {
                op: "+".to_owned(),
                message: "plus takes two numbers".to_owned(),
                step: 0,
            }),
        })
    }

    /// Builds a machine whose operations are the memory primitives
    /// plus the counting addition, assembled from the book's
    /// controller datums.
    fn machine(
        memory: &SharedMemory,
        registers: &[&str],
        controller: &str,
    ) -> Result<Machine, Fault> {
        let mut operations = memory_operations(memory);
        operations.push(("+", plus()));
        let datums = read_program(controller).map_err(|error| Fault::Parse(error.to_string()))?;
        make_machine_from_datums(registers, &operations, &datums)
    }

    /// The proper list of the numbers, built bottom up the way the
    /// machine's cons allocates: the innermost cell first.
    fn num_list(memory: &SharedMemory, items: &[i128]) -> Word {
        let mut word = Word::Empty;
        for item in items.iter().rev() {
            word = memory
                .borrow_mut()
                .cons(Word::Num(*item), word)
                .expect("room for the tree");
        }
        word
    }

    /// The tree `((1 2) 3 4)`, the list of Figure 5.14, as a word in
    /// the memory: a pair whose car is the pair `(1 2)` and whose cdr
    /// is the list `(3 4)`.
    fn figure_tree(memory: &SharedMemory) -> Word {
        let one_two = num_list(memory, &[1, 2]);
        let three_four = num_list(memory, &[3, 4]);
        memory
            .borrow_mut()
            .cons(one_two, three_four)
            .expect("room for the tree")
    }

    /// The tree `(1 (2 (3 4)))`: the same four leaves at a deeper
    /// cdr nesting, so the two machines agree on a different shape.
    fn deep_tree(memory: &SharedMemory) -> Word {
        let inner = num_list(memory, &[3, 4]);
        let middle = memory
            .borrow_mut()
            .cons(Word::Num(2), inner)
            .expect("room for the tree");
        memory
            .borrow_mut()
            .cons(Word::Num(1), middle)
            .expect("room for the tree")
    }

    /// The first machine: the recursive count-leaves whose two
    /// subcounts are joined with `+`. Each level saves its continue,
    /// its pair, and the car side's count; `n` holds the count the
    /// level returns.
    fn count_leaves_machine(memory: &SharedMemory) -> Machine {
        machine(
            memory,
            &["tree", "n", "temp", "continue"],
            "
begin-count-leaves
  (assign continue (label count-leaves-done))
count-loop
  (test (op null?) (reg tree))
  (branch (label null-tree))
  (test (op pair?) (reg tree))
  (branch (label pair-tree))
  (assign n (const 1))
  (goto (reg continue))
null-tree
  (assign n (const 0))
  (goto (reg continue))
pair-tree
  (save continue)
  (save tree)
  (assign tree (op car) (reg tree))
  (assign continue (label after-car))
  (goto (label count-loop))
after-car
  (restore tree)
  (save n)
  (assign tree (op cdr) (reg tree))
  (assign continue (label after-cdr))
  (goto (label count-loop))
after-cdr
  (restore temp)
  (assign n (op +) (reg n) (reg temp))
  (restore continue)
  (goto (reg continue))
count-leaves-done",
        )
        .expect("the controller assembles")
    }

    /// The second machine: recursive count-leaves with the explicit
    /// counter of the book's `count-iter`. One `n` threads through
    /// the whole recursion: the car side counts starting from the
    /// running total, and the cdr side counts starting from the car
    /// side's answer.
    fn count_iter_machine(memory: &SharedMemory) -> Machine {
        machine(
            memory,
            &["tree", "n", "continue"],
            "
begin-count-leaves
  (assign n (const 0))
  (assign continue (label count-leaves-done))
count-loop
  (test (op null?) (reg tree))
  (branch (label null-tree))
  (test (op pair?) (reg tree))
  (branch (label pair-tree))
  (assign n (op +) (reg n) (const 1))
  (goto (reg continue))
null-tree
  (goto (reg continue))
pair-tree
  (save continue)
  (save tree)
  (assign tree (op car) (reg tree))
  (assign continue (label after-car))
  (goto (label count-loop))
after-car
  (restore tree)
  (assign tree (op cdr) (reg tree))
  (assign continue (label after-cdr))
  (goto (label count-loop))
after-cdr
  (restore continue)
  (goto (reg continue))
count-leaves-done",
        )
        .expect("the controller assembles")
    }

    /// Runs one machine over one tree and answers its `n` register.
    fn count(memory: &SharedMemory, build: fn(&SharedMemory) -> Word) -> i128 {
        for build_machine in [count_leaves_machine, count_iter_machine] {
            let tree = build(memory);
            let mut machine = build_machine(memory);
            machine
                .set_register("tree", Value::from(tree))
                .expect("tree is a declared register");
            machine.start().expect("the run completes");
            assert_eq!(
                machine.get_register("n").unwrap(),
                Value::Int(4),
                "both machines count the same four leaves"
            );
        }
        4
    }

    /// Both machines count the four leaves of `((1 2) 3 4)`, the
    /// list of Figure 5.14.
    #[test]
    fn ex_5_21_counts_the_figure_tree() {
        let memory: SharedMemory = Rc::new(RefCell::new(ch05::sec_5_3::Memory::new(16, 4, 1)));
        assert_eq!(count(&memory, figure_tree), 4);
    }

    /// Both machines count the four leaves of `(1 (2 (3 4)))`, whose
    /// nesting sits on the cdr side instead of the car side.
    #[test]
    fn ex_5_21_counts_the_deep_tree() {
        let memory: SharedMemory = Rc::new(RefCell::new(ch05::sec_5_3::Memory::new(16, 4, 1)));
        assert_eq!(count(&memory, deep_tree), 4);
    }

    /// The empty list has no leaves and a bare number is one leaf:
    /// the machines' base cases, driven directly.
    #[test]
    fn ex_5_21_base_cases() {
        let memory: SharedMemory = Rc::new(RefCell::new(ch05::sec_5_3::Memory::new(16, 4, 1)));
        for build_machine in [count_leaves_machine, count_iter_machine] {
            let mut machine = build_machine(&memory);
            machine
                .set_register("tree", Value::from(Word::Empty))
                .unwrap();
            machine.start().unwrap();
            assert_eq!(machine.get_register("n").unwrap(), Value::Int(0));

            let mut machine = build_machine(&memory);
            machine
                .set_register("tree", Value::from(Word::Num(7)))
                .unwrap();
            machine.start().unwrap();
            assert_eq!(machine.get_register("n").unwrap(), Value::Int(1));
        }
    }

    /// The first machine's cost on `((1 2) 3 4)`: five pair levels,
    /// each saving a continue, a tree, and then the car count, so
    /// fifteen pushes and six stack cells deep at most.
    #[test]
    fn ex_5_21_first_machine_stack_cost() {
        let memory: SharedMemory = Rc::new(RefCell::new(ch05::sec_5_3::Memory::new(16, 4, 1)));
        let mut machine = count_leaves_machine(&memory);
        machine
            .set_register("tree", Value::from(figure_tree(&memory)))
            .unwrap();
        machine.start().unwrap();
        assert_eq!(machine.stack_statistics(), (15, 6));
    }
}
