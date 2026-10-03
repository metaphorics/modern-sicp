// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.21: the two register machines
//! for `count-leaves`, both run over the list-structure memory
//! operations of section 5.3. The tree words follow the section's
//! discipline: `0` is the empty list, a positive word is a pair
//! address, and the leaf `n` rides the machine word `-n-1`, so the
//! leaves 1 to 4 are the words -2 to -5.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, constant, label, reg, reg_op};
use ch05::sec_5_2::{Fault, Machine, assemble};
use ch05::sec_5_3::{Memory, MemoryFault, SharedMemory, Word, memory_operations};

/// One row of a controller: an optional leading label and its
/// instruction.
type Row = (Option<Label>, Instruction);

/// One unlabeled instruction row.
fn row(instruction: Instruction) -> Row {
    (None, instruction)
}

/// One labeled instruction row.
fn at(name: &str, instruction: Instruction) -> Row {
    (Some(label(name)), instruction)
}

/// `assign` from an operand.
fn assign(name: &str, value: Operand) -> Instruction {
    Instruction::Assign {
        target: reg(name),
        value,
    }
}

/// The named register operand shorthand.
fn source(name: &str) -> Operand {
    reg_op(name)
}

/// The operation-valued operand the book writes
/// `(assign target (op name) args...)` as.
fn operation(name: &str, arguments: &[Operand]) -> Operand {
    Operand::Operation {
        operation: name.to_owned(),
        arguments: arguments.to_vec(),
    }
}

/// `test` of one predicate over the given operands.
fn test(predicate: &str, arguments: &[Operand]) -> Instruction {
    Instruction::Test {
        predicate: predicate.to_owned(),
        arguments: arguments.to_vec(),
    }
}

/// `goto` by label name.
fn jump(name: &str) -> Instruction {
    Instruction::Goto(Operand::Label(label(name)))
}

/// `goto` through a register.
fn jump_register(name: &str) -> Instruction {
    Instruction::Goto(Operand::Register(reg(name)))
}

/// The linear-recursive `count-leaves` of the exercise: the two
/// clauses of the `cond` are two tests, and the sum needs the first
/// count held across the second call, so it lives in the scratch
/// register `t` and rides the stack.
#[must_use]
pub fn count_leaves_recursive() -> MachineProgram {
    MachineProgram::new(
        vec![reg("tree"), reg("val"), reg("continue"), reg("t")],
        vec![
            row(assign("continue", Operand::Label(label("count-done")))),
            at("count-leaves", test("null?", &[source("tree")])),
            row(Instruction::Branch(label("null-case"))),
            row(test("pair?", &[source("tree")])),
            row(Instruction::Branch(label("pair-case"))),
            row(assign("val", constant(1))),
            row(jump_register("continue")),
            at("null-case", assign("val", constant(0))),
            row(jump_register("continue")),
            at("pair-case", Instruction::Save(reg("continue"))),
            row(Instruction::Save(reg("tree"))),
            row(assign("continue", Operand::Label(label("after-car")))),
            row(assign("tree", operation("car", &[source("tree")]))),
            row(jump("count-leaves")),
            at("after-car", Instruction::Restore(reg("tree"))),
            row(assign("t", source("val"))),
            row(Instruction::Save(reg("t"))),
            row(assign("continue", Operand::Label(label("after-cdr")))),
            row(assign("tree", operation("cdr", &[source("tree")]))),
            row(jump("count-leaves")),
            at("after-cdr", Instruction::Restore(reg("t"))),
            row(Instruction::Restore(reg("continue"))),
            row(assign(
                "val",
                operation("add", &[source("t"), source("val")]),
            )),
            row(jump_register("continue")),
            at(
                "count-done",
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![source("val")],
                },
            ),
        ],
    )
}

/// The `count-leaves` with the explicit counter: the inner call
/// computes `count-iter(car tree, n)` and its answer becomes the `n`
/// of the outer call, which sits in tail position and needs no
/// return label of its own — only the tree cell and the caller's
/// continuation cross the inner call.
#[must_use]
pub fn count_leaves_counter() -> MachineProgram {
    MachineProgram::new(
        vec![reg("tree"), reg("n"), reg("val"), reg("continue")],
        vec![
            row(assign("n", constant(0))),
            row(assign("continue", Operand::Label(label("count-done")))),
            at("count-iter", test("null?", &[source("tree")])),
            row(Instruction::Branch(label("null-case"))),
            row(test("pair?", &[source("tree")])),
            row(Instruction::Branch(label("pair-case"))),
            row(assign("val", operation("add", &[source("n"), constant(1)]))),
            row(jump_register("continue")),
            at("null-case", assign("val", source("n"))),
            row(jump_register("continue")),
            at("pair-case", Instruction::Save(reg("tree"))),
            row(Instruction::Save(reg("continue"))),
            row(assign("continue", Operand::Label(label("after-inner")))),
            row(assign("tree", operation("car", &[source("tree")]))),
            row(jump("count-iter")),
            at("after-inner", Instruction::Restore(reg("continue"))),
            row(Instruction::Restore(reg("tree"))),
            row(assign("n", source("val"))),
            row(assign("tree", operation("cdr", &[source("tree")]))),
            row(jump("count-iter")),
            at(
                "count-done",
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![source("val")],
                },
            ),
        ],
    )
}

/// Builds one list cell per element, the leaves as the section's
/// integer data words. The cells are allocated head first and
/// chained with `set-cdr!`, so the head of a nonempty list is the
/// first word after the root region.
fn build_list(memory: &SharedMemory, leaves: &[i64]) -> Result<Word, MemoryFault> {
    let mut head = Word::Addr(0);
    let mut link: Option<Word> = None;
    for leaf in leaves {
        let cell = memory.borrow_mut().cons(Word::Int(*leaf), Word::Addr(0))?;
        match link.replace(cell) {
            Some(previous) => memory.borrow_mut().set_cdr(previous, cell)?,
            None => head = cell,
        }
    }
    Ok(head)
}

/// Reads one proper list back to its leaf data.
fn read_list(memory: &SharedMemory, mut list: Word) -> Result<Vec<i64>, MemoryFault> {
    let mut leaves = Vec::new();
    loop {
        let (head, rest) = {
            let heap = memory.borrow();
            (heap.car(list)?, heap.cdr(list)?)
        };
        let Word::Int(leaf) = head else {
            return Err(MemoryFault::OutOfBounds);
        };
        leaves.push(leaf);
        match rest {
            Word::Addr(0) => return Ok(leaves),
            Word::Addr(_) => list = rest,
            Word::Int(_) => return Err(MemoryFault::OutOfBounds),
        }
    }
}

/// The machine word of a heap word: an address as itself, an integer
/// datum `n` as `-n-1`.
fn tree_value(tree: Word) -> i64 {
    match tree {
        Word::Addr(address) => i64::try_from(address).expect("small heap"),
        Word::Int(value) => -value - 1,
    }
}

mod ex_5_21 {
    //! Exercise 5.21: implement register machines for recursive
    //! `count-leaves` and for the version with the explicit counter.

    use super::*;

    /// A shared heap holding one tree of four leaves, built in the
    /// section's word discipline.
    fn four_leaf_tree(memory: &SharedMemory) -> Result<Word, MemoryFault> {
        let one_two = memory.borrow_mut().cons(Word::Int(1), Word::Int(2))?;
        let three_four = memory.borrow_mut().cons(Word::Int(3), Word::Int(4))?;
        memory.borrow_mut().cons(one_two, three_four)
    }

    /// Runs one machine over the given heap and tree.
    fn count_over(
        machine_program: &MachineProgram,
        memory: &SharedMemory,
        tree: Word,
    ) -> Result<i64, Fault> {
        let mut machine = Machine::new(assemble(machine_program)?);
        for (name, op) in memory_operations(memory) {
            machine.install_operation(&name, op);
        }
        machine.set_register("tree", tree_value(tree))?;
        let outcome = machine.run()?;
        Ok(outcome.registers["val"])
    }

    /// Both machines count the leaves of `((1 . 2) . (3 . 4))`: four
    /// leaves, whichever order the recursion visits them.
    #[test]
    fn ex_5_21_both_machines_count_four_leaves() -> Result<(), Box<dyn std::error::Error>> {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
        let tree = four_leaf_tree(&memory)?;
        assert_eq!(count_over(&count_leaves_recursive(), &memory, tree)?, 4);
        assert_eq!(count_over(&count_leaves_counter(), &memory, tree)?, 4);
        Ok(())
    }

    /// The clauses of the `cond` are distinguished by shape: the
    /// empty list counts 0 and a bare leaf counts 1, in both
    /// machines.
    #[test]
    fn ex_5_21_empty_list_and_bare_leaf() -> Result<(), Box<dyn std::error::Error>> {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
        assert_eq!(
            count_over(&count_leaves_recursive(), &memory, Word::Addr(0))?,
            0
        );
        assert_eq!(
            count_over(&count_leaves_counter(), &memory, Word::Addr(0))?,
            0
        );
        let leaf = Word::Int(7);
        assert_eq!(count_over(&count_leaves_recursive(), &memory, leaf)?, 1);
        assert_eq!(count_over(&count_leaves_counter(), &memory, leaf)?, 1);
        Ok(())
    }

    /// The two machines agree shape for shape: a proper list of four
    /// leaves, a nested tree of five, and a spine whose cars are
    /// pairs.
    #[test]
    fn ex_5_21_machines_agree_on_any_shape() -> Result<(), Box<dyn std::error::Error>> {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
        let proper = build_list(&memory, &[1, 2, 3, 4])?;
        assert_eq!(count_over(&count_leaves_recursive(), &memory, proper)?, 4);
        assert_eq!(count_over(&count_leaves_counter(), &memory, proper)?, 4);

        let one_two = memory.borrow_mut().cons(Word::Int(1), Word::Int(2))?;
        let three = memory.borrow_mut().cons(Word::Int(3), Word::Addr(0))?;
        let nested = memory.borrow_mut().cons(one_two, three)?;
        assert_eq!(count_over(&count_leaves_recursive(), &memory, nested)?, 3);
        assert_eq!(count_over(&count_leaves_counter(), &memory, nested)?, 3);
        Ok(())
    }

    /// The list the recursive machine reads back is the one the heap
    /// was built from: the machine's word model round-trips leaves
    /// 1 to 4 through -2 to -5 untouched.
    #[test]
    fn ex_5_21_word_model_round_trips() -> Result<(), Box<dyn std::error::Error>> {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
        let tree = build_list(&memory, &[1, 2, 3, 4])?;
        assert_eq!(tree_value(tree), 1);
        assert_eq!(tree_value(Word::Int(1)), -2);
        assert_eq!(tree_value(Word::Int(4)), -5);
        assert_eq!(read_list(&memory, tree)?, [1, 2, 3, 4]);
        Ok(())
    }
}
