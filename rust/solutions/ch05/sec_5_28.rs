// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.28: the tail recursion
//! removed. The naive `ev-sequence` of the 5.4.2 footnote replaces
//! the tail-recursive one: every expression of a sequence is
//! evaluated across a saved `continue`, so no expression is in tail
//! position and a tail call pushes. The 5.26 and 5.27 experiments are
//! rerun on this evaluator, and both rows of the book's
//! demonstration hold: with the optimization removed, both versions
//! require space that grows linearly with their input.

use ch05::sec_5_2::Fault;
use ch05::sec_5_4::{compose_controller, make_evaluator};

mod ex_5_28 {
    //! Exercise 5.28: modify the evaluator by changing `ev-sequence`
    //! so that the evaluator is no longer tail-recursive, rerun the
    //! 5.26 and 5.27 experiments, and demonstrate that both versions
    //! of the factorial procedure now require space that grows
    //! linearly with their input.

    use super::*;

    /// The monitored driver of 5.4.4, the same variant 5.26 and 5.27
    /// measured under.
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

    /// The naive `ev-sequence` of the 5.4.2 footnote: the save and
    /// restore cycle for the last expression in a sequence as well as
    /// for the others.
    const NAIVE_EV_SEQUENCE: &str = "ev-sequence
  (test (op no-more-exps?) (reg unev))
  (branch (label ev-sequence-end))
  (assign exp (op first-exp) (reg unev))
  (save unev)
  (save env)
  (assign continue
          (label ev-sequence-continue))
  (goto (label eval-dispatch))
ev-sequence-continue
  (restore env)
  (restore unev)
  (assign unev (op rest-exps) (reg unev))
  (goto (label ev-sequence))
ev-sequence-end
  (restore continue)
  (goto (reg continue))";

    /// The iterative factorial of 1.2.1.
    const ITERATIVE_SOURCE: &str = "(define (factorial n) (define (iter product counter) (if (> counter n) product (iter (* counter product) (+ counter 1)))) (iter 1 1))";

    /// The recursive factorial.
    const RECURSIVE_SOURCE: &str =
        "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";

    fn controller() -> String {
        compose_controller(&[
            ("ev-sequence", NAIVE_EV_SEQUENCE),
            ("driver", MONITORED_DRIVER),
        ])
    }

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

    /// Runs the program's calls of `(factorial n)` for each `n` on a
    /// fresh naive machine and answers the counters of each call plus
    /// the printed value.
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
    /// point.
    fn fit_linear(ns: &[i128], ps: &[u64]) -> Option<(i64, i64)> {
        let (n0, n1) = (*ns.first()?, *ns.last()?);
        let p0 = i64::try_from(*ps.first()?).ok()?;
        let p1 = i64::try_from(*ps.last()?).ok()?;
        let slope = (p1 - p0) / i64::try_from(n1 - n0).ok()?;
        Some((slope, p0 - slope * i64::try_from(n0).ok()?))
    }

    /// The growth checks: every depth row now grows with n, and every
    /// measured table is verified against the line its endpoints
    /// determine.
    #[test]
    fn ex_5_28() -> Result<(), Fault> {
        let ns: Vec<i128> = (1..=5).collect();

        // The iterative factorial's depth, constant at 10 under the
        // book's evaluator, now grows three per n.
        let iterative = measure(ITERATIVE_SOURCE, &ns)?;
        let depths: Vec<u64> = iterative.iter().map(|(_, stats, _)| stats.depth).collect();
        let (a, b) = fit_linear(&ns, &depths).expect("iterative depths");
        assert_eq!((a, b), (3, 14));
        let iterative_pushes: Vec<u64> =
            iterative.iter().map(|(_, stats, _)| stats.pushes).collect();
        assert_eq!(fit_linear(&ns, &iterative_pushes), Some((37, 33)));

        // The recursive factorial's depth grows too, eight per n, and
        // its pushes keep the recursive shape.
        let recursive = measure(RECURSIVE_SOURCE, &ns)?;
        let depths: Vec<u64> = recursive.iter().map(|(_, stats, _)| stats.depth).collect();
        let (a, b) = fit_linear(&ns, &depths).expect("recursive depths");
        assert_eq!((a, b), (8, 3));
        let recursive_pushes: Vec<u64> =
            recursive.iter().map(|(_, stats, _)| stats.pushes).collect();
        assert_eq!(fit_linear(&ns, &recursive_pushes), Some((34, -16)));

        // The measured rows, each from its own run: both programs now
        // require space that grows linearly with their input.
        let naive_iterative: Vec<(i128, u64)> = iterative
            .iter()
            .map(|(n, stats, _)| (*n, stats.depth))
            .collect();
        assert_eq!(
            naive_iterative,
            vec![(1, 17), (2, 20), (3, 23), (4, 26), (5, 29)]
        );
        let naive_recursive: Vec<(i128, u64)> = recursive
            .iter()
            .map(|(n, stats, _)| (*n, stats.depth))
            .collect();
        assert_eq!(
            naive_recursive,
            vec![(1, 11), (2, 19), (3, 27), (4, 35), (5, 43)]
        );

        // The answers did not change with the evaluator.
        assert_eq!(iterative[4].2, "120");
        assert_eq!(recursive[4].2, "120");
        Ok(())
    }
}
