// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.14: a host primitive map
//! cannot apply a compound procedure without the evaluator's apply.

/// Shared typed support for this exercise.
pub mod support;

use sicp_runtime::SicpError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Primitive {
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Procedure {
    Primitive(Primitive),
    Compound(i64),
}

fn host_map(procedure: Procedure, values: &[i64]) -> Result<Vec<i64>, SicpError> {
    let Procedure::Primitive(primitive) = procedure else {
        return Err(SicpError::TypeMismatch(
            "host map accepts only primitive procedures".to_owned(),
        ));
    };
    Ok(values
        .iter()
        .map(|value| match primitive {
            Primitive::Square => value * value,
        })
        .collect())
}

fn apply(procedure: Procedure, value: i64) -> i64 {
    match procedure {
        Procedure::Primitive(Primitive::Square) => value * value,
        Procedure::Compound(offset) => value + offset,
    }
}

#[test]
fn ex_4_14() {
    assert_eq!(
        host_map(Procedure::Primitive(Primitive::Square), &[1, 2, 3]),
        Ok(vec![1, 4, 9])
    );
    assert!(host_map(Procedure::Compound(10), &[1, 2, 3]).is_err());
    assert_eq!(apply(Procedure::Compound(10), 5), 15);
}
