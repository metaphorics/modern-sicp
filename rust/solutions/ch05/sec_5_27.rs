// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.27: the recursive factorial
//! on the monitored stack, for comparison with 5.26. The measurements
//! fill the book's table, and the n = 5 row is the very session the
//! 5.4.4 prose quotes.

use ch05::sec_5_2::Fault;
use ch05::sec_5_4::{compose_controller, make_evaluator};

mod ex_5_27 {
    //! Exercise 5.27: for the recursive factorial, determine as a
    //! function of n the maximum depth of the stack and the total
    //! number of pushes used in computing n! for the range of values
    //! of n in the 5.26 table.

    use super::*;

    /// The monitored driver of 5.4.4, the same variant 5.26 measured
    /// under.
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

    /// The recursive factorial, the program the exercise measures.
    const RECURSIVE_SOURCE: &str =
        "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

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

    /// Runs the program's calls of `(factorial n)` for each `n` on a
    /// fresh machine and answers the counters of each call plus the
    /// printed value.
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

    /// The stats lines of a run, one per interaction, in order.
    fn stats_of(transcript: &[String]) -> Vec<Stats> {
        transcript
            .iter()
            .filter(|line| line.starts_with("(total-pushes"))
            .filter_map(|line| parse_stats(line))
            .collect()
    }

    fn controller() -> String {
        compose_controller(&[("driver", MONITORED_DRIVER)])
    }

    /// The `a` and `b` of `p(n) = a*n + b` through the first and last
    /// point.
    fn fit_linear(ns: &[i128], ps: &[u64]) -> Option<(i64, i64)> {
        let (n0, n1) = (*ns.first()?, *ns.last()?);
        let p0 = i64::try_from(*ps.first()?).ok()?;
        let p1 = i64::try_from(*ps.last()?).ok()?;
        let slope = (p1 - p0) / i64::try_from(n1 - n0).ok()?;
        Some((slope, p0 - slope * i64::try_from(n0).ok()?))
    }

    /// Measures n = 1 to 6: the maximum depth is `5n + 3` and the
    /// pushes are `32n - 16`, each formula computed from the data's
    /// endpoints and verified on every measured point. The n = 5 row,
    /// 144 pushes at depth 28, is the book's quoted session.
    #[test]
    fn ex_5_27() -> Result<(), Fault> {
        let ns: Vec<i128> = (1..=6).collect();
        let measured = measure(RECURSIVE_SOURCE, &ns)?;
        let depths: Vec<u64> = measured.iter().map(|(_, stats, _)| stats.depth).collect();
        let pushes: Vec<u64> = measured.iter().map(|(_, stats, _)| stats.pushes).collect();

        let (depth_a, depth_b) = fit_linear(&ns, &depths).expect("at least two depths");
        assert_eq!((depth_a, depth_b), (5, 3));
        let (push_a, push_b) = fit_linear(&ns, &pushes).expect("at least two pushes");
        assert_eq!((push_a, push_b), (32, -16));

        let table: Vec<(i128, u64, u64)> = measured
            .iter()
            .map(|(n, stats, _)| (*n, stats.pushes, stats.depth))
            .collect();
        assert_eq!(
            table,
            vec![
                (1, 16, 8),
                (2, 48, 13),
                (3, 80, 18),
                (4, 112, 23),
                (5, 144, 28),
                (6, 176, 33),
            ]
        );

        // The comparison with 5.26 is the point: the iterative
        // version's depth is the constant 10, the recursive version's
        // grows by 5 per n, and the per-level push overhead is
        // visible in the slopes 35 against 32.
        assert_eq!(measured[4].2, "120", "the book's session value");
        Ok(())
    }
}
