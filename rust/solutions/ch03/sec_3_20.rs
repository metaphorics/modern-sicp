// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.20. The book's exercise asks
//! for an environment diagram of the procedural `cons`; that is a
//! question about Scheme frames, so this edition replaces it: the same
//! aliasing trace, run against the main text's dispatch closure, where
//! the "environment" of each pair is the state its closure captured.

use ch03::sec_3_3::{PairRequest, ProcPair, procedural_cons};
use sicp_runtime::Value;

mod ex_3_20 {
    use super::{PairRequest, ProcPair, Value, procedural_cons};

    /// Exercise 3.20: trace aliasing through the procedural pair
    ///
    /// Builds one procedural pair, takes a second name for it by
    /// cloning the dispatch handle, mutates the shared cell through the
    /// second name, and reads back through the first. It then builds a
    /// separate pair with the same starting contents to show what a
    /// non-aliased pair answers. The result is `(aliased, separate)`:
    /// the value the first name sees after the mutation, and the value
    /// the fresh pair still holds.
    #[must_use]
    pub fn ex_3_20() -> (i128, i128) {
        let x = procedural_cons(Value::int(1), Value::int(2));
        let alias = ProcPair::clone(&x);

        let _ = alias.send(PairRequest::SetCar(Value::int(17)));

        let aliased_answer = x.send(PairRequest::Car);
        let separate = procedural_cons(Value::int(1), Value::int(2));
        let separate_answer = separate.send(PairRequest::Car);

        let (Value::Int(answer), Value::Int(other)) = (aliased_answer, separate_answer) else {
            return (0, 0);
        };
        (answer, other)
    }
}

#[test]
fn ex_3_20() {
    // The mutation through `alias` is visible through `x`, because both
    // dispatch closures captured the same two cells.
    assert_eq!(ex_3_20::ex_3_20(), (17, 1));

    // The full trace: the second name shares the cells, a fresh pair
    // with the same contents does not.
    let x = procedural_cons(Value::int(1), Value::int(2));
    let alias = ProcPair::clone(&x);
    let separate = procedural_cons(Value::int(1), Value::int(2));

    let _ = alias.send(PairRequest::SetCdr(Value::sym("tail")));
    let through_x = x.send(PairRequest::Cdr);
    let through_separate = separate.send(PairRequest::Cdr);
    assert_eq!(through_x, Value::sym("tail"));
    assert_eq!(through_separate, Value::int(2));

    // The compiler cannot see any of this: `x`, `alias`, and
    // `separate` have the same type, `ProcPair`, and only the runtime
    // answers distinguish shared cells from equal ones.
}
