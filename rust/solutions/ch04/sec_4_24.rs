// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.24: analysis versus execution time, timed with the host
// clock after warmup on the same fib workload..

use ch04::eval_support::*;

mod ex_4_24 {
    use super::*;

    const DEFINITION: &str = "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))";
    const CALL: &str = "(fib 13)";
    const WARMUP: usize = 5;
    const RUNS: usize = 30;

    fn per_run<R>(runs: usize, job: impl Fn() -> R) -> u128 {
        for _ in 0..WARMUP {
            job();
        }
        let start = std::time::Instant::now();
        for _ in 0..runs {
            job();
        }
        start.elapsed().as_nanos() / runs as u128
    }

    /// Answers the base evaluator's per-call nanoseconds and the
    /// analyzed execution's per-call nanoseconds over the same
    /// workload, analysis paid once up front.
    ///
    /// # Errors
    /// Whatever the evaluation raises.
    pub fn benchmark() -> Result<(u128, u128), SchemeError> {
        let env = setup_environment();
        let definition = read(DEFINITION)?;
        let call = read(CALL)?;
        Base.eval(&definition, &env)?;
        let base_ns = per_run(RUNS, || Base.eval(&call, &env).expect("base runs"));

        let analyzer = AnalyzerBase::default();
        analyzer.eval_exp(&definition, &env)?;
        let exec = analyzer.analyze(&call)?;
        let analyzed_ns = per_run(RUNS, || exec(&env).expect("analyzed runs"));
        Ok((base_ns, analyzed_ns))
    }
}

#[test]
fn ex_4_24() {
    let (base_ns, analyzed_ns) = ex_4_24::benchmark().expect("runs");
    println!("base eval per (fib 13): {base_ns} ns; pre-analyzed: {analyzed_ns} ns");
    assert!(
        analyzed_ns < base_ns,
        "analysis amortizes: {analyzed_ns} ns must beat {base_ns} ns"
    );
}
