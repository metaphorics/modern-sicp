// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercises 5.5 and 5.5a: hand-simulated
//! traces of the factorial and Fibonacci machines, and the edition's
//! restore annotations.

use ch05::sec_5_1::{
    Event, Value, factorial_recursive, fibonacci, render_annotated_trace, render_trace,
};

/// The number of restore-continue events whose value is pushed back
/// unchanged by the next save of the same register: the pure
/// round-trips exercise 5.5a's annotations expose.
fn round_trip_pairs(events: &[Event]) -> usize {
    let mut count = 0;
    for (index, event) in events.iter().enumerate() {
        let Event::Restore {
            reg: "continue",
            value,
            ..
        } = event
        else {
            continue;
        };
        let pushed_back = events[index + 1..]
            .iter()
            .find(|event| {
                matches!(
                    event,
                    Event::Save {
                        reg: "continue",
                        ..
                    }
                )
            })
            .is_some_and(|next| matches!(next, Event::Save { value: back, .. } if back == value));
        count += usize::from(pushed_back);
    }
    count
}

mod ex_5_05 {
    //! Exercise 5.5: hand-simulate the factorial and Fibonacci
    //! machines on a nontrivial input, showing the stack at each
    //! significant point.

    use super::*;

    /// The factorial machine (the book's Figure 5.11) on n = 3: four
    /// saves and four restores, the stack peaking at depth 4.
    #[test]
    fn ex_5_05_factorial() {
        let run = factorial_recursive()
            .run(&[("n", Value::Int(3))])
            .expect("run");
        assert_eq!(run.value_of("val"), Value::Int(6));
        assert_eq!(run.instructions, 27);
        assert_eq!(run.pushes, 4);
        assert_eq!(run.pops, 4);
        assert_eq!(run.max_depth, 4);
        assert_eq!(render_trace(&run.events), FACT_N3_TRACE);
    }

    /// The Fibonacci machine (the book's Figure 5.12) on n = 3: both
    /// recursive calls execute, eight saves against eight restores.
    #[test]
    fn ex_5_05_fibonacci() {
        let run = fibonacci().run(&[("n", Value::Int(3))]).expect("run");
        assert_eq!(run.value_of("val"), Value::Int(2));
        assert_eq!(run.instructions, 51);
        assert_eq!(run.pushes, 8);
        assert_eq!(run.pops, 8);
        assert_eq!(run.max_depth, 4);
        assert_eq!(render_trace(&run.events), FIB_N3_TRACE);
    }

    const FACT_N3_TRACE: &str = "   3 fact-loop     save continue=fact-done  stack [continue=fact-done] depth 1
   4 fact-loop     save n=3  stack [n=3, continue=fact-done] depth 2
  10 fact-loop     save continue=after-fact  stack [continue=after-fact, n=3, continue=fact-done] depth 3
  11 fact-loop     save n=2  stack [n=2, continue=after-fact, n=3, continue=fact-done] depth 4
  19 after-fact    restore n=2  stack [continue=after-fact, n=3, continue=fact-done] depth 3
  20 after-fact    restore continue=after-fact  stack [n=3, continue=fact-done] depth 2
  23 after-fact    restore n=3  stack [continue=fact-done] depth 1
  24 after-fact    restore continue=fact-done  stack [] depth 0
";

    const FIB_N3_TRACE: &str = "   3 fib-loop      save continue=fib-done  stack [continue=fib-done] depth 1
   5 fib-loop      save n=3  stack [n=3, continue=fib-done] depth 2
  10 fib-loop      save continue=afterfib-n-1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  12 fib-loop      save n=2  stack [n=2, continue=afterfib-n-1, n=3, continue=fib-done] depth 4
  19 afterfib-n-1  restore n=2  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  20 afterfib-n-1  restore continue=afterfib-n-1  stack [n=3, continue=fib-done] depth 2
  22 afterfib-n-1  save continue=afterfib-n-1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  24 afterfib-n-1  save val=1  stack [val=1, continue=afterfib-n-1, n=3, continue=fib-done] depth 4
  31 afterfib-n-2  restore val=1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  32 afterfib-n-2  restore continue=afterfib-n-1  stack [n=3, continue=fib-done] depth 2
  35 afterfib-n-1  restore n=3  stack [continue=fib-done] depth 1
  36 afterfib-n-1  restore continue=fib-done  stack [] depth 0
  38 afterfib-n-1  save continue=fib-done  stack [continue=fib-done] depth 1
  40 afterfib-n-1  save val=1  stack [val=1, continue=fib-done] depth 2
  47 afterfib-n-2  restore val=1  stack [continue=fib-done] depth 1
  48 afterfib-n-2  restore continue=fib-done  stack [] depth 0
";
}

mod ex_5_05a {
    //! Exercise 5.5a (this edition): annotate every restore in the
    //! Fibonacci trace with the save it matches and the age of the
    //! value it returns, and read the redundant pair of exercise 5.6
    //! off the annotations.

    use super::*;

    /// The annotated trace of the n = 3 Fibonacci run. The
    /// annotations show the pattern: at step 20 the restore of
    /// `continue` returns the value `afterfib-n-1`, and at step 22
    /// the very next save of `continue` pushes that same value back;
    /// steps 36 and 38 repeat the pair at the outer level. A restore
    /// whose value is pushed back unchanged, with no use in between,
    /// can be removed together with that save.
    #[test]
    fn ex_5_05a() {
        let run = fibonacci().run(&[("n", Value::Int(3))]).expect("run");
        assert_eq!(render_annotated_trace(&run.events), FIB_N3_ANNOTATED);
        // One pure round-trip per internal call: the two levels of
        // the n = 3 computation. These pairs are exactly the
        // redundant save and restore of exercise 5.6.
        assert_eq!(round_trip_pairs(&run.events), 2);
    }

    const FIB_N3_ANNOTATED: &str = "   3 fib-loop      save continue=fib-done  stack [continue=fib-done] depth 1
   5 fib-loop      save n=3  stack [n=3, continue=fib-done] depth 2
  10 fib-loop      save continue=afterfib-n-1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  12 fib-loop      save n=2  stack [n=2, continue=afterfib-n-1, n=3, continue=fib-done] depth 4
  19 afterfib-n-1  restore n=2  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3 matches save #12, age 7, 1 older save of n beneath
  20 afterfib-n-1  restore continue=afterfib-n-1  stack [n=3, continue=fib-done] depth 2 matches save #10, age 10, 1 older save of continue beneath
  22 afterfib-n-1  save continue=afterfib-n-1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3
  24 afterfib-n-1  save val=1  stack [val=1, continue=afterfib-n-1, n=3, continue=fib-done] depth 4
  31 afterfib-n-2  restore val=1  stack [continue=afterfib-n-1, n=3, continue=fib-done] depth 3 matches save #24, age 7
  32 afterfib-n-2  restore continue=afterfib-n-1  stack [n=3, continue=fib-done] depth 2 matches save #22, age 10, 1 older save of continue beneath
  35 afterfib-n-1  restore n=3  stack [continue=fib-done] depth 1 matches save #5, age 30
  36 afterfib-n-1  restore continue=fib-done  stack [] depth 0 matches save #3, age 33
  38 afterfib-n-1  save continue=fib-done  stack [continue=fib-done] depth 1
  40 afterfib-n-1  save val=1  stack [val=1, continue=fib-done] depth 2
  47 afterfib-n-2  restore val=1  stack [continue=fib-done] depth 1 matches save #40, age 7
  48 afterfib-n-2  restore continue=fib-done  stack [] depth 0 matches save #38, age 10
";
}
