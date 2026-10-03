// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.27: memoization. The memoized
//! `memo-fib` answers each argument once and stores it in the table, so
//! the exponential process becomes linear; the environment diagram of
//! the book's question becomes a trace of the table's growth here.

use std::cell::Cell;

use ch03::sec_3_3::MemoTable;
use sicp_runtime::{Key, SicpError, Value};

mod ex_3_27 {
    use super::{memo_fib_traced, naive_calls};

    /// Exercise 3.27: memoized fib turns exponential into linear
    ///
    /// Runs the memoized Fibonacci for 25 steps and reports the value,
    /// how many times the wrapped computation actually ran, how many
    /// distinct arguments the table ended up holding, and how many
    /// calls the unwrapped procedure of the same shape would have made.
    #[must_use]
    pub fn ex_3_27() -> (i128, u64, u64, u64) {
        let (value, computed, stored) = memo_fib_traced(25);
        (value, computed, stored, naive_calls(25))
    }
}

/// The book's `memoize` applied to `fib`: one `MemoTable` shared by
/// every recursive level, because the closures all capture it.
///
/// # Panics
/// When `n` is large enough that a Fibonacci value overflows `i128`.
#[must_use]
pub fn memo_fib_traced(n: u32) -> (i128, u64, u64) {
    let table = MemoTable::new();
    let computed = Cell::new(0u64);
    let value = memo_go(n, &table, &computed).expect("fib of a small count is exact");
    (value, computed.get(), table.len() as u64)
}

fn memo_go(n: u32, table: &MemoTable, computed: &Cell<u64>) -> Result<i128, SicpError> {
    let answer = table.lookup_insert(Key::int(i128::from(n)), || {
        computed.set(computed.get() + 1);
        match n {
            0 => Ok(Value::int(0)),
            1 => Ok(Value::int(1)),
            _ => {
                let smaller = memo_go(n - 1, table, computed)?;
                let smallest = memo_go(n - 2, table, computed)?;
                let total = smaller.checked_add(smallest).ok_or(SicpError::Overflow)?;
                Ok(Value::int(total))
            }
        }
    })?;
    match answer {
        Value::Int(total) => Ok(total),
        _ => Err(SicpError::TypeMismatch(
            "memo-fib stored a non-number".into(),
        )),
    }
}

/// How many calls the unwrapped `fib` makes for `n`: the count the
/// memoized version reduces to `n + 1` computed steps.
#[must_use]
pub fn naive_calls(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => 1 + naive_calls(n - 1) + naive_calls(n - 2),
    }
}

#[test]
fn ex_3_27() {
    // memo-fib computes each argument exactly once: 26 computed steps
    // and 26 stored values for fib(25), against 242,785 naive calls.
    assert_eq!(ex_3_27::ex_3_27(), (75_025, 26, 26, 242_785));

    // The table is why: a second query answers from storage and runs
    // nothing.
    let table = MemoTable::new();
    let computed = Cell::new(0u64);
    let _ = memo_go(10, &table, &computed).expect("small fib");
    assert_eq!(computed.get(), 11);
    assert_eq!(table.len(), 11);

    // Explaining the book's question in this shape: (memo-fib 10)
    // drives the memoized procedure 177 calls, but only 11 of them
    // compute; the rest are table lookups that hit.
    assert_eq!(naive_calls(10), 177);
}
