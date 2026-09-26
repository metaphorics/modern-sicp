// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.29: the stack of the
//! tree-recursive Fibonacci computation, monitored and reduced to
//! formulas checked against the measurements instead of asserted.

use ch05::sec_5_2::Fault;
use ch05::sec_5_4::{compose_controller, make_evaluator};

mod ex_5_29 {
    //! Exercise 5.29: monitor the stack operations in the
    //! tree-recursive Fibonacci computation, give a formula for the
    //! maximum depth of the stack in terms of n, a formula for the
    //! total number of pushes, the fixed overhead k of the recurrence
    //! S(n) = S(n-1) + S(n-2) + k, and the `a` and `b` of
    //! S(n) = a*Fib(n+1) + b.

    use super::*;

    /// The monitored driver of 5.4.4, the same variant the factorial
    /// exercises measured under.
    const MONITORED_DRIVER: &str = "read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input)
           (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output)
           (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))";

    /// The tree-recursive Fibonacci, the program the exercise
    /// measures.
    const FIB_SOURCE: &str = "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))";

    /// The counters one interaction printed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Stats {
        pushes: u64,
        depth: u64,
    }

    /// Reads the counters out of a stats line the machine printed.
    fn parse_stats(line: &str) -> Option<Stats> {
        let rest = line.strip_prefix("(total-pushes = ")?;
        let (pushes, rest) = rest.split_once(' ')?;
        let depth = rest.strip_prefix("maximum-depth = ")?.trim_end_matches(')');
        Some(Stats {
            pushes: pushes.parse().ok()?,
            depth: depth.parse().ok()?,
        })
    }

    /// The stats lines of a run, one per interaction, in order.
    fn stats_of(transcript: &[String]) -> Vec<Stats> {
        transcript
            .iter()
            .filter(|line| line.starts_with("(total-pushes"))
            .filter_map(|line| parse_stats(line))
            .collect()
    }

    /// Runs `(fib n)` on a fresh machine and answers the counters of
    /// the call plus the printed value.
    fn measure(n: i128) -> Result<(Stats, String), Fault> {
        let mut evaluator =
            make_evaluator(&controller(), &[], &format!("{FIB_SOURCE}\n(fib {n})"))?;
        evaluator.run()?;
        let transcript = evaluator.transcript();
        let stats = stats_of(&transcript)
            .into_iter()
            .last()
            .ok_or_else(|| Fault::Op {
                op: String::new(),
                message: "the call printed no stack statistics".to_owned(),
                step: 0,
            })?;
        let at = transcript.len().saturating_sub(2);
        Ok((stats, transcript[at].clone()))
    }

    fn controller() -> String {
        compose_controller(&[("driver", MONITORED_DRIVER)])
    }

    /// The Fibonacci numbers, for the closed form's coefficient.
    fn fib(n: i128) -> i128 {
        let (mut a, mut b) = (0_i128, 1);
        for _ in 0..n {
            let next = a.checked_add(b).expect("small n");
            a = b;
            b = next;
        }
        a
    }

    /// Measures fib for n = 2 to 9 and checks the two formulas
    /// against the data: the maximum depth grows by exactly `5` per
    /// n, so depth = `5n + 3`, and the pushes satisfy
    /// `S(n) = S(n-1) + S(n-2) + 40` at every n, with the closed form
    /// `S(n) = 56 * Fib(n+1) - 40` computed from the measurements.
    #[test]
    fn ex_5_29() -> Result<(), Fault> {
        let ns: Vec<i128> = (2..=9).collect();
        let mut runs = Vec::new();
        for n in &ns {
            runs.push((*n, measure(*n)?));
        }

        // Part a: the depth steps by exactly 5 per n, and the fitted
        // line is 5n + 3.
        let depths: Vec<u64> = runs.iter().map(|(_, (stats, _))| stats.depth).collect();
        for pair in depths.windows(2) {
            assert_eq!(pair[1] - pair[0], 5, "the depth step");
        }
        let depth_n0 = i64::try_from(ns[0]).expect("small n");
        assert_eq!(
            i64::try_from(depths[0]).expect("small depth"),
            5 * depth_n0 + 3
        );

        // The recurrence: S(n) = S(n-1) + S(n-2) + k with one k at
        // every n from 4 up.
        let pushes: Vec<i128> = runs
            .iter()
            .map(|(_, (stats, _))| i128::from(stats.pushes))
            .collect();
        let ks: Vec<i128> = pushes
            .windows(3)
            .map(|window| window[2] - window[1] - window[0])
            .collect();
        assert!(ks.iter().all(|k| *k == ks[0]), "one constant k: {ks:?}");
        let k = ks[0];
        assert_eq!(k, 40);

        // The closed form: a and b computed from the first two
        // points, then verified on every measured n.
        let fib0 = fib(ns[0] + 1);
        let fib1 = fib(ns[1] + 1);
        let a = (pushes[1] - pushes[0]) / (fib1 - fib0);
        let b = pushes[0] - a * fib0;
        assert_eq!((a, b), (56, -40));
        for (n, pushes) in ns.iter().zip(&pushes) {
            let model = a * fib(*n + 1) + b;
            assert_eq!(model, *pushes, "n = {n}");
        }

        // The values the machine computed are the Fibonacci numbers.
        for (n, (_, value)) in &runs {
            assert_eq!(*value, fib(*n).to_string(), "n = {n}");
        }

        // The measured table, each row from its own run.
        let table: Vec<(i128, u64, u64)> = runs
            .iter()
            .map(|(n, (stats, _))| (*n, stats.pushes, stats.depth))
            .collect();
        assert_eq!(
            table,
            vec![
                (2, 72, 13),
                (3, 128, 18),
                (4, 240, 23),
                (5, 408, 28),
                (6, 688, 33),
                (7, 1136, 38),
                (8, 1864, 43),
                (9, 3040, 48),
            ]
        );
        Ok(())
    }
}
