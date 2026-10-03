// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercises 5.15 and 5.15a: instruction
//! counting with a print-and-reset message, and this edition's
//! instruction budget.

use std::cell::Cell;
use std::rc::Rc;

use ch05::sec_5_1::{Instruction, MachineProgram, Operand, fibonacci_machine};
use ch05::sec_5_2::{Fault, Machine, OpHandler, Run, assemble};

/// The typed budget fault the exercise asks for: a run due to
/// execute an instruction past the budget halts, carrying the count
/// and the program counter of the instruction that would have run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetExceeded {
    /// The instructions already executed.
    pub count: u64,
    /// The program counter of the refused instruction.
    pub pc: usize,
}

/// How a counted run can end: at the budget, or at a machine fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Halted {
    /// The budget refused the next instruction.
    Budget(BudgetExceeded),
    /// The machine raised a fault.
    Fault(Fault),
}

/// Steps one machine one instruction at a time, counting, and stops
/// at the budget with the step that would have run. The machine is
/// halted at its final instruction, so the resumed run only reports
/// the outcome — it never re-executes the controller.
fn step_counted(machine: &mut Machine, budget: u64) -> Result<Run, Halted> {
    let mut count = 0;
    while machine.pc() < machine.assembled().instructions.len() {
        if count >= budget {
            return Err(Halted::Budget(BudgetExceeded {
                count,
                pc: machine.pc(),
            }));
        }
        machine.step().map_err(Halted::Fault)?;
        count += 1;
    }
    machine.resume().map_err(Halted::Fault)
}

/// Runs one machine under an instruction budget.
fn run_bounded(
    program: &MachineProgram,
    inputs: &[(&str, i64)],
    budget: u64,
) -> Result<Run, Halted> {
    let assembled = assemble(program).map_err(Halted::Fault)?;
    let mut machine = Machine::new(assembled);
    for (name, value) in inputs {
        machine.set_register(name, *value).map_err(Halted::Fault)?;
    }
    step_counted(&mut machine, budget)
}

/// Runs one machine with no budget.
fn run_counted(program: &MachineProgram, inputs: &[(&str, i64)]) -> Result<Run, Halted> {
    run_bounded(program, inputs, u64::MAX)
}

/// The Fibonacci machine whose final `perform` prints its own
/// instruction count and resets the counter: the message of the
/// exercise, visible from the controller.
fn fibonacci_counting() -> MachineProgram {
    let mut program = fibonacci_machine();
    if let Some((_, instruction)) = program.instructions.last_mut() {
        *instruction = Instruction::Perform {
            operation: "print".to_owned(),
            arguments: vec![Operand::Operation {
                operation: "instruction-count".to_owned(),
                arguments: vec![],
            }],
        };
    }
    program
}

/// Runs the counting Fibonacci machine: the driver bumps the shared
/// counter before every step, and the controller's `instruction-count`
/// operation reads and resets it.
fn run_with_message(n: i64) -> Result<Run, Halted> {
    let counter: Rc<Cell<u64>> = Rc::new(Cell::new(0));
    let reported = Rc::clone(&counter);
    let operation: OpHandler = Rc::new(move |_args: &[i64]| {
        let count = reported.get();
        reported.set(0);
        i64::try_from(count).map_err(|_| Fault::Overflow("instruction count"))
    });
    let assembled = assemble(&fibonacci_counting()).map_err(Halted::Fault)?;
    let mut machine = Machine::new(assembled);
    machine.install_operation("instruction-count", operation);
    machine.set_register("n", n).map_err(Halted::Fault)?;
    while machine.pc() < machine.assembled().instructions.len() {
        counter.set(counter.get() + 1);
        machine.step().map_err(Halted::Fault)?;
    }
    machine.resume().map_err(Halted::Fault)
}

mod ex_5_15 {
    //! Exercise 5.15: have the machine keep track of the number of
    //! instructions executed and accept a message that prints the
    //! count and resets it to zero.

    use super::*;

    /// Every executed instruction counts, transfers included. The
    /// base case runs six instructions (the initial assign, the
    /// test, the branch, the base assign, the return goto, and the
    /// final print); fib(2) runs 29 and the whole computation of
    /// fib(6) runs 282.
    #[test]
    fn ex_5_15_counts() {
        let zero = run_counted(&fibonacci_machine(), &[("n", 0)]).expect("run");
        let one = run_counted(&fibonacci_machine(), &[("n", 1)]).expect("run");
        let two = run_counted(&fibonacci_machine(), &[("n", 2)]).expect("run");
        let six = run_counted(&fibonacci_machine(), &[("n", 6)]).expect("run");
        assert_eq!(zero.steps, 6);
        assert_eq!(one.steps, 6);
        assert_eq!(two.steps, 29);
        assert_eq!(six.steps, 282);
    }

    /// The message: the count is printed through the controller's
    /// `print (instruction-count)` operand, and the counter is back
    /// to zero afterwards — the same run's message reports its own
    /// whole instruction count, and it restarts at the first
    /// instruction of the next run.
    #[test]
    fn ex_5_15_print_and_reset_message() {
        let counted = run_with_message(3).expect("run");
        assert_eq!(counted.output, ["52".to_owned()]);
        let again = run_with_message(3).expect("run");
        assert_eq!(again.output, ["52".to_owned()]);
        let zero = run_with_message(0).expect("run");
        assert_eq!(zero.output, ["6".to_owned()]);
    }
}

mod ex_5_15a {
    //! Exercise 5.15a (this edition): the counting machine accepts a
    //! budget; a run due to execute an instruction past it halts
    //! with a typed fault carrying the count and the program
    //! counter.

    use super::*;

    /// A budget far below the demand halts the run at the count
    /// itself, naming the instruction that would have run: fib(6)
    /// stopped after ten instructions at pc 3, the first save.
    #[test]
    fn ex_5_15a_run_halts_at_the_budget() {
        let fault = run_bounded(&fibonacci_machine(), &[("n", 6)], 10).expect_err("budget");
        assert_eq!(fault, Halted::Budget(BudgetExceeded { count: 10, pc: 3 }));
    }

    /// One instruction short of the demand still faults; the exact
    /// demand finishes cleanly. fib(6) needs 282 instructions.
    #[test]
    fn ex_5_15a_exact_demand_is_the_boundary() {
        let fault = run_bounded(&fibonacci_machine(), &[("n", 6)], 281).expect_err("budget");
        assert_eq!(fault, Halted::Budget(BudgetExceeded { count: 281, pc: 22 }));
        let done = run_bounded(&fibonacci_machine(), &[("n", 6)], 282).expect("run");
        assert_eq!(done.steps, 282);
        assert_eq!(done.registers["val"], 8);
    }

    /// The halt carries exactly what a resumed run would continue
    /// from: the counted progress and the halted program counter.
    #[test]
    fn ex_5_15a_halt_is_inspectable() {
        let fault = run_bounded(&fibonacci_machine(), &[("n", 6)], 100).expect_err("budget");
        assert_eq!(fault, Halted::Budget(BudgetExceeded { count: 100, pc: 21 }));
    }
}
