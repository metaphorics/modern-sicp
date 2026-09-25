// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.59: Alyssa's `meeting-time` rule
//! over the weekly meetings. A person's meeting is either the
//! whole-company meeting or their division's meeting, and the two
//! disjuncts interleave, which pins the order of Alyssa's Wednesday
//! answers.

use ch04::sec_4_4::{Engine, microshaft};

mod ex_4_59 {
    //! Exercise 4.59: the meetings data base, the rule, and the two
    //! queries the exercise asks for.

    use super::*;

    /// One engine carrying the meetings and the rule.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&[
            "(meeting accounting (Monday 9am))",
            "(meeting administration (Monday 10am))",
            "(meeting computer (Wednesday 3pm))",
            "(meeting administration (Friday 1pm))",
            "(meeting whole-company (Wednesday 4pm))",
            "(rule (meeting-time ?person ?day-and-time) \
             (or (meeting whole-company ?day-and-time) \
             (and (meeting ?division ?day-and-time) \
             (job ?person (?division . ?type)))))",
        ]);
        engine
    }
}

#[test]
fn ex_4_59() {
    // a. Ben's Friday query: the one Friday meeting is the
    // administration's, so its attendees are Warbucks and Aull.
    assert_eq!(
        ex_4_59::engine().answers("(meeting-time ?who (Friday ?time))"),
        [
            "(meeting-time (Warbucks Oliver) (Friday 1pm))",
            "(meeting-time (Aull DeWitt) (Friday 1pm))",
        ]
    );
    // b. Alyssa's Wednesday query: the whole-company disjunct answers
    // 4pm first, and the interleaved computer disjunct answers 3pm
    // second -- the `or`'s interleave, not the data base's order.
    assert_eq!(
        ex_4_59::engine().answers("(meeting-time (Hacker Alyssa P) (Wednesday ?time))"),
        [
            "(meeting-time (Hacker Alyssa P) (Wednesday 4pm))",
            "(meeting-time (Hacker Alyssa P) (Wednesday 3pm))",
        ]
    );
}
