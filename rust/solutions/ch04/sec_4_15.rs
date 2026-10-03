// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.15: the halting-problem
//! diagonal over a bounded typed machine.

/// Shared typed support for this exercise.
pub mod support;

use sicp_runtime::SicpError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Program {
    Halt,
    Diverge,
}

fn bounded_run(program: Program, budget: usize) -> Result<(), SicpError> {
    match program {
        Program::Halt => Ok(()),
        Program::Diverge => {
            if budget == 0 {
                Err(SicpError::TypeMismatch(
                    "the step budget ran out".to_owned(),
                ))
            } else {
                bounded_run(program, budget - 1)
            }
        }
    }
}

fn verdict_says_halts() -> Result<(), SicpError> {
    bounded_run(Program::Diverge, 3)
}

fn verdict_says_diverges() -> Result<(), SicpError> {
    bounded_run(Program::Halt, 3)
}

#[test]
fn ex_4_15() {
    assert!(verdict_says_halts().is_err());
    assert!(verdict_says_diverges().is_ok());
}
