// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.29: memoization changes the
//! counts. `(square (id 10))` feeds one thunk to two demand sites of
//! `*`: memoized, `id`'s body runs once; unmemoized, every forcing runs
//! it again. The session then extends the counts through `cube`, whose
//! body demands its parameter three times.

use ch04::eval_support::*;

mod ex_4_29 {
    use super::*;

    /// The unmemoized twin of the section evaluator: compound
    /// parameters delay into recomputing wrappers, so a demand site
    /// always re-evaluates.
    #[derive(Debug, Default)]
    pub struct LazyNoMemo;

    impl LazyEval for LazyNoMemo {
        fn force_value(&self, value: Value) -> EvalResult {
            force_recomputing(self, value)
        }

        fn delay_operand(&self, proc: &Rc<Closure>, _position: usize) -> Option<bool> {
            matches!(proc.as_ref(), Closure { .. }).then_some(false)
        }
    }

    impl Evaluator for LazyNoMemo {
        fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
            lazy_step(self, exp, env)
        }
    }

    /// The book's counting session: `square` demands its parameter
    /// twice, `cube` three times, and `count` answers after each.
    const SESSION: &[&str] = &[
        "(define count 0)",
        "(define (id x) (set! count (+ count 1)) x)",
        "(define (square x) (* x x))",
        "(define (cube x) (* x (* x x)))",
        "(square (id 10))",
        "count",
        "(cube (id 10))",
        "count",
    ];

    /// The printed answers of the session under both forcing
    /// disciplines.
    #[must_use]
    pub fn answers() -> (Vec<String>, Vec<String>) {
        let memo = run_values(&Lazy);
        let no_memo = run_values(&LazyNoMemo);
        (memo, no_memo)
    }

    fn run_values(ev: &impl LazyEval) -> Vec<String> {
        let (values, _) = run_lazy(ev, &SESSION.join("\n")).expect("the session runs");
        printed(&values)
    }
}

#[test]
fn ex_4_29() {
    let (memo, no_memo) = ex_4_29::answers();
    // Both evaluators answer 100 and 1000: memoization changes the
    // number of computations, not the values.
    assert_eq!(&memo[4..5], &["100"]);
    assert_eq!(&memo[6..7], &["1000"]);
    assert_eq!(&no_memo[4..5], &["100"]);
    assert_eq!(&no_memo[6..7], &["1000"]);
    // Memoized: the define runs nothing, `square` forces the one thunk
    // twice but computes it once (count 1), and `cube` forces three
    // times, still computing once more (count 2).
    assert_eq!(&memo[5..6], &["1"]);
    assert_eq!(&memo[7..], &["2"]);
    // Unmemoized: every one of the two, then the three, forcings runs
    // `id`'s body again.
    assert_eq!(&no_memo[5..6], &["2"]);
    assert_eq!(&no_memo[7..], &["5"]);
}
