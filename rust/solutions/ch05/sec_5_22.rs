// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.22: the register machines for
//! `append` and `append!`, both run over the list-structure memory
//! operations of section 5.3. The words follow the section's
//! discipline: `0` is the empty list, a positive word is a pair
//! address, and the integer datum `n` rides the word `-n-1`.

use std::cell::RefCell;
use std::rc::Rc;

use ch05::sec_5_1::{Instruction, Label, MachineProgram, Operand, label, reg, reg_op};
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

/// The `append` machine: the answer is `(cons (car x) (append (cdr
/// x) y))`, so the pending `cons` — which needs the first cell of
/// `x` — crosses the recursive call in the saved `x`, and the
/// accumulated list comes back in `val`.
#[must_use]
pub fn append_machine() -> MachineProgram {
    MachineProgram::new(
        vec![reg("x"), reg("y"), reg("val"), reg("continue")],
        vec![
            row(assign("continue", Operand::Label(label("append-done")))),
            at("append", test("null?", &[source("x")])),
            row(Instruction::Branch(label("base-case"))),
            row(Instruction::Save(reg("continue"))),
            row(assign("continue", Operand::Label(label("after-append")))),
            row(Instruction::Save(reg("x"))),
            row(assign("x", operation("cdr", &[source("x")]))),
            row(jump("append")),
            at("after-append", Instruction::Restore(reg("x"))),
            row(Instruction::Restore(reg("continue"))),
            row(assign(
                "val",
                operation("cons", &[operation("car", &[source("x")]), source("val")]),
            )),
            row(jump_register("continue")),
            at("base-case", assign("val", source("y"))),
            row(jump_register("continue")),
            at(
                "append-done",
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![source("val")],
                },
            ),
        ],
    )
}

/// The `append!` machine: the answer is the original `x`, so it is
/// copied into `val` before the walk to `last-pair` reuses the `x`
/// register, and the splice is the effect-only `set-cdr!`.
#[must_use]
pub fn append_bang_machine() -> MachineProgram {
    MachineProgram::new(
        vec![reg("x"), reg("y"), reg("val"), reg("continue")],
        vec![
            row(assign("val", source("x"))),
            row(assign("continue", Operand::Label(label("append-done")))),
            at(
                "last-pair",
                test("null?", &[operation("cdr", &[source("x")])]),
            ),
            row(Instruction::Branch(label("splice"))),
            row(assign("x", operation("cdr", &[source("x")]))),
            row(jump("last-pair")),
            at(
                "splice",
                Instruction::Perform {
                    operation: "set-cdr".to_owned(),
                    arguments: vec![source("x"), source("y")],
                },
            ),
            row(jump_register("continue")),
            at(
                "append-done",
                Instruction::Perform {
                    operation: "print".to_owned(),
                    arguments: vec![source("val")],
                },
            ),
        ],
    )
}

/// The heap word a machine value names: an address as itself, the
/// datum `n` as its integer word.
fn heap_word(value: i64) -> Word {
    if value <= -1 {
        Word::Int(-value - 1)
    } else {
        Word::Addr(usize::try_from(value).expect("non-negative word"))
    }
}

/// Builds one list cell per leaf, the leaves as integer data.
fn build_list(memory: &SharedMemory, leaves: &[i64]) -> Result<Word, MemoryFault> {
    let mut list = Word::Addr(0);
    for leaf in leaves.iter().rev() {
        list = memory.borrow_mut().cons(Word::Int(*leaf), list)?;
    }
    Ok(list)
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

/// One machine run over a fresh heap holding `x` and `y`, answering
/// the machine's `val` and the heap.
fn run_pair(
    machine_program: &MachineProgram,
    x_leaves: &[i64],
    y_leaves: &[i64],
) -> Result<(i64, SharedMemory, Word, Word), Box<dyn std::error::Error>> {
    let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
    let x = build_list(&memory, x_leaves)?;
    let y = build_list(&memory, y_leaves)?;
    let mut machine = Machine::new(assemble(machine_program)?);
    for (name, op) in memory_operations(&memory) {
        machine.install_operation(&name, op);
    }
    machine.set_register("x", word_value(x))?;
    machine.set_register("y", word_value(y))?;
    machine.run()?;
    let answer = machine.get_register("val")?;
    Ok((answer, memory, x, y))
}

/// The machine word a heap word travels as: an address as itself, an
/// integer datum `n` as `-n-1`.
fn word_value(word: Word) -> i64 {
    match word {
        Word::Addr(address) => i64::try_from(address).expect("small heap"),
        Word::Int(value) => -value - 1,
    }
}

mod ex_5_22 {
    //! Exercise 5.22: design a register machine to implement `append`
    //! and one to implement `append!`.

    use super::*;

    /// `append` builds a fresh list: the answer is `(1 2 3 4)`,
    /// neither input is disturbed, and the answer shares no cell
    /// with `x`.
    #[test]
    fn ex_5_22_append_builds_a_fresh_list() -> Result<(), Box<dyn std::error::Error>> {
        let (answer, memory, x, y) = run_pair(&append_machine(), &[1, 2], &[3, 4])?;
        let list = heap_word(answer);
        assert_eq!(read_list(&memory, list)?, [1, 2, 3, 4]);
        assert_ne!(list, x);
        assert_eq!(read_list(&memory, x)?, [1, 2]);
        assert_eq!(read_list(&memory, y)?, [3, 4]);
        Ok(())
    }

    /// The base case answers `y` itself: appending to the empty list
    /// allocates nothing and the answer is the second pointer.
    #[test]
    fn ex_5_22_append_of_the_empty_list() -> Result<(), Box<dyn std::error::Error>> {
        let (answer, memory, _, y) = run_pair(&append_machine(), &[], &[3, 4])?;
        assert_eq!(heap_word(answer), y);
        assert_eq!(read_list(&memory, heap_word(answer))?, [3, 4]);
        Ok(())
    }

    /// `append!` splices: the answer is the original `x`, now
    /// reading `(1 2 3 4)`, and its last cell names `y`'s first cell
    /// rather than the empty list.
    #[test]
    fn ex_5_22_append_bang_splices() -> Result<(), Box<dyn std::error::Error>> {
        let (answer, memory, x, y) = run_pair(&append_bang_machine(), &[1, 2], &[3, 4])?;
        assert_eq!(heap_word(answer), x);
        assert_eq!(read_list(&memory, x)?, [1, 2, 3, 4]);
        assert_eq!(read_list(&memory, y)?, [3, 4]);
        let spliced = memory.borrow().cdr(x)?;
        assert_eq!(memory.borrow().cdr(spliced)?, y);
        Ok(())
    }

    /// Splicing into the empty list is the machine's one boundary
    /// case: `last-pair` has no cell to visit, so the run faults
    /// rather than inventing a head.
    #[test]
    fn ex_5_22_append_bang_of_the_empty_list_faults() -> Result<(), Box<dyn std::error::Error>> {
        let memory: SharedMemory = Rc::new(RefCell::new(Memory::new(64, 1)));
        let y = build_list(&memory, &[3, 4])?;
        let mut machine = Machine::new(assemble(&append_bang_machine())?);
        for (name, op) in memory_operations(&memory) {
            machine.install_operation(&name, op);
        }
        machine.set_register("x", word_value(Word::Addr(0)))?;
        machine.set_register("y", word_value(y))?;
        let fault = machine.run().expect_err("no cell to splice into");
        assert!(matches!(fault, Fault::UndefinedOperation(_)));
        Ok(())
    }
}
