// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.26: the monitored stack
//! explores the evaluator's tail-recursive property with the
//! iterative factorial of 1.2.1.
//!
//! The monitored driver is the 5.4.4 variant: `print-result` performs
//! `print-stack-statistics` before announcing the value, and the
//! driver initializes the stack once per interaction, so every
//! interaction's counters are its own. The exercises that measure the
//! stack (5.26 to 5.29) share this module's controller and harness.

use ch05::sec_5_2::Fault;
use ch05::sec_5_4::{compose_controller, make_evaluator};

mod ex_5_26 {
    //! Exercise 5.26: record the maximum stack depth and the number
    //! of pushes required to compute the iterative factorial of n for
    //! a range of values of n, and determine formulas from the data.

    use super::*;

    /// The monitored driver of 5.4.4: the statistics printed before
    /// the value.
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

    /// The base controller with the monitored driver.
    fn controller() -> String {
        compose_controller(&[("driver", MONITORED_DRIVER)])
    }

    /// The iterative factorial of 1.2.1, the tail-recursive program
    /// the exercise measures.
    const ITERATIVE_SOURCE: &str = "(define (factorial n) (define (iter product counter) (if (> counter n) product (iter (* counter product) (+ counter 1)))) (iter 1 1))";

    /// The counters one interaction printed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Stats {
        pushes: u64,
        depth: u64,
    }

    /// Reads the counters out of a `(total-pushes = P maximum-depth = D)`
    /// line the machine printed.
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

    /// Runs the program's calls of `(factorial n)` for each `n` on a
    /// fresh machine and answers the counters of each call plus the
    /// printed value, the last line before the trailing prompt.
    fn measure(source: &str, ns: &[i128]) -> Result<Vec<(i128, Stats, String)>, Fault> {
        ns.iter()
            .map(|n| {
                let mut evaluator =
                    make_evaluator(&controller(), &[], &format!("{source}\n(factorial {n})"))?;
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
                Ok((*n, stats, transcript[at].clone()))
            })
            .collect()
    }

    /// The `a` and `b` of `p(n) = a*n + b` through the first and last
    /// point, or nothing for fewer than two points.
    fn fit_linear(ns: &[i128], ps: &[u64]) -> Option<(i64, i64)> {
        let (n0, n1) = (*ns.first()?, *ns.last()?);
        let p0 = i64::try_from(*ps.first()?).ok()?;
        let p1 = i64::try_from(*ps.last()?).ok()?;
        let slope = (p1 - p0) / i64::try_from(n1 - n0).ok()?;
        Some((slope, p0 - slope * i64::try_from(n0).ok()?))
    }

    /// Measures n = 1 to 6 and checks the two answers against the
    /// data: the maximum depth is `10` for every n, independent of n
    /// (part a), and the pushes fit `35n + 29` with the fitted
    /// constants verified on every measured point (part b).
    #[test]
    fn ex_5_26() -> Result<(), Fault> {
        let ns: Vec<i128> = (1..=6).collect();
        let measured = measure(ITERATIVE_SOURCE, &ns)?;
        let depths: Vec<u64> = measured.iter().map(|(_, stats, _)| stats.depth).collect();
        let pushes: Vec<u64> = measured.iter().map(|(_, stats, _)| stats.pushes).collect();

        // Part a: the depth is the same constant at every n.
        assert!(depths.iter().all(|depth| *depth == depths[0]));
        assert_eq!(depths[0], 10);

        // Part b: the pushes fit the line through the endpoints, and
        // the line holds on every measured point.
        let (a, b) = fit_linear(&ns, &pushes).expect("at least two measurements");
        assert_eq!((a, b), (35, 29));
        for (n, pushes) in ns.iter().zip(&pushes) {
            let model = i64::try_from(*n).expect("small n") * a + b;
            assert_eq!(u64::try_from(model).expect("positive"), *pushes, "n = {n}");
        }

        // The measured table, each row measured from its own run.
        let table: Vec<(i128, u64, u64)> = measured
            .iter()
            .map(|(n, stats, _)| (*n, stats.pushes, stats.depth))
            .collect();
        assert_eq!(
            table,
            vec![
                (1, 64, 10),
                (2, 99, 10),
                (3, 134, 10),
                (4, 169, 10),
                (5, 204, 10),
                (6, 239, 10),
            ]
        );

        // The evaluator still answers the right products.
        assert_eq!(measured[5].2, "720");
        Ok(())
    }
}
