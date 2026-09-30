// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.8: Errors and tests.

use thiserror::Error;

/// A small arithmetic error, derived with `thiserror` the same way the
/// rest of the book derives its one error type,
/// [`SicpError`](sicp_runtime::SicpError).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ArithError {
    /// Division or remainder by zero.
    #[error("division by zero")]
    DivideByZero,
}

/// Checked integer division: an `Err` instead of a panic on a zero
/// divisor, propagated with `?` at the call site.
///
/// # Errors
/// [`ArithError::DivideByZero`] when `divisor` is zero.
pub fn safe_div(dividend: i128, divisor: i128) -> Result<i128, ArithError> {
    if divisor == 0 {
        Err(ArithError::DivideByZero)
    } else {
        Ok(dividend / divisor)
    }
}
