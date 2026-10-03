// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.10: new surface syntax is
//! desugared before dispatch, leaving the core evaluator unchanged.

/// Shared typed support for this exercise.
pub mod support;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Square,
    Increment,
}

enum Surface {
    Defun(&'static str, Operation),
    Fun(&'static str, Operation),
    Apply(&'static str, i64),
}

enum Core {
    Define(&'static str, Operation),
    Call(&'static str, i64),
}

fn desugar(form: &Surface) -> Core {
    match form {
        Surface::Defun(name, operation) | Surface::Fun(name, operation) => {
            Core::Define(name, *operation)
        }
        Surface::Apply(name, value) => Core::Call(name, *value),
    }
}

fn evaluate(core: &Core, functions: &mut HashMap<&'static str, Operation>) -> Option<i64> {
    match core {
        Core::Define(name, operation) => {
            functions.insert(name, *operation);
            Some(0)
        }
        Core::Call(name, value) => match functions.get(name)? {
            Operation::Square => Some(value * value),
            Operation::Increment => Some(value + 1),
        },
    }
}

#[test]
fn ex_4_10() {
    let mut functions = HashMap::new();
    assert_eq!(
        evaluate(
            &desugar(&Surface::Defun("square", Operation::Square)),
            &mut functions
        ),
        Some(0)
    );
    assert_eq!(
        evaluate(&desugar(&Surface::Apply("square", 7)), &mut functions),
        Some(49)
    );
    assert_eq!(evaluate(&Core::Call("defun", 7), &mut functions), None);
    assert_eq!(
        evaluate(
            &desugar(&Surface::Fun("increment", Operation::Increment)),
            &mut functions
        ),
        Some(0)
    );
    assert_eq!(
        evaluate(&desugar(&Surface::Apply("increment", 41)), &mut functions),
        Some(42)
    );
}
