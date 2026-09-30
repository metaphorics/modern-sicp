// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.6: the Fibonacci machine's
//! redundant save and restore, removed and shown unnecessary.

use ch05::sec_5_1::{Instruction, MachineProgram, Register, fibonacci_machine};
use ch05::sec_5_2::{Fault, Machine, assemble};

/// The Fibonacci machine without its redundant save and restore: the
/// adjacent `restore continue` / `save continue` pair in the second
/// setup is dropped, so the second call reuses the copy the first
/// setup already saved.
fn fibonacci_without_redundant_pair() -> MachineProgram {
    let program = fibonacci_machine();
    let mut instructions = Vec::with_capacity(program.instructions.len());
    let mut index = 0;
    while index < program.instructions.len() {
        if is_redundant_pair(&program.instructions, index) {
            index += 2;
            continue;
        }
        instructions.push(program.instructions[index].clone());
        index += 1;
    }
    MachineProgram::new(program.registers.clone(), instructions)
}

/// Whether the row at `index` opens the redundant pair: a
/// `restore continue` immediately followed by a `save continue`.
fn is_redundant_pair(rows: &[(Option<ch05::sec_5_1::Label>, Instruction)], index: usize) -> bool {
    let Some((_, next)) = rows.get(index + 1) else {
        return false;
    };
    let restores_continue = matches!(
        &rows[index].1,
        Instruction::Restore(Register(name)) if name == "continue"
    );
    let saves_continue = matches!(next, Instruction::Save(Register(name)) if name == "continue");
    restores_continue && saves_continue
}

mod ex_5_06 {
    //! Exercise 5.6: find the Fibonacci machine's redundant save and
    //! restore.

    use super::*;

    /// The answer: the `(restore continue)` at `afterfibn-1` and the
    /// `(save continue)` in the second setup. The restore brings back
    /// the caller's return label only for the next save to push it
    /// straight back; removing both leaves the stack balanced,
    /// because the final `(restore continue)` at `afterfibn-2` then
    /// pops the copy saved at the first setup. The trace evidence is
    /// exercise 5.5a's steps 21/22 and 37/38.
    #[test]
    fn ex_5_06() {
        // Both machines compute Fibonacci for every n up to 10.
        for n in 0..=10 {
            let expected = fib_direct(n);
            let original = run(&fibonacci_machine(), n).expect("run");
            let pruned = run(&fibonacci_without_redundant_pair(), n).expect("run");
            assert_eq!(original.0, expected, "original on n = {n}");
            assert_eq!(pruned.0, expected, "pruned on n = {n}");
        }
        // At n = 6 the original machine executes 282 instructions
        // with 48 pushes; the pruned machine 258 and 36: one save and
        // one restore per internal call, twelve of them.
        let original = run(&fibonacci_machine(), 6).expect("run");
        let pruned = run(&fibonacci_without_redundant_pair(), 6).expect("run");
        assert_eq!((original.1, original.2), (282, 48));
        assert_eq!((pruned.1, pruned.2), (258, 36));
        assert_eq!(original.0, 8);
        assert_eq!(pruned.0, 8);
    }

    /// The pruned machine removes exactly one pair and leaves every
    /// other row, label, and register declaration in place.
    #[test]
    fn ex_5_06_pruning_removes_exactly_the_pair() {
        let original = fibonacci_machine();
        let pruned = fibonacci_without_redundant_pair();
        assert_eq!(original.instructions.len() - pruned.instructions.len(), 2);
        assert_eq!(original.registers, pruned.registers);
    }

    /// Runs one machine to completion and answers its value register,
    /// instruction count, and push count.
    fn run(program: &MachineProgram, n: i64) -> Result<(i64, u64, u64), Fault> {
        let mut machine = Machine::new(assemble(program)?);
        machine.set_register("n", n)?;
        let run = machine.run()?;
        let value = machine.get_register("val")?;
        Ok((value, run.steps, machine.stack_statistics().pushes))
    }

    /// The host's own Fibonacci, for the direct comparison.
    fn fib_direct(n: i64) -> i64 {
        if n < 2 {
            n
        } else {
            fib_direct(n - 1) + fib_direct(n - 2)
        }
    }
}
