// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.24: benchmark analysis versus execution. One recursive
 * arithmetic workload — `fib(14)` — runs under the engine's direct
 * `evaluate` and under `analyze(...)(...)`, each over the same
 * environment. After two untimed warmups, seven calls are timed with
 * `performance.now()`; each path reports the median of its seven runs,
 * and the analysis fraction is `(direct - analyzed) / direct`, the
 * share of direct time attributable to re-analyzing syntax. The
 * benchmark reports its raw runs so the fraction is read with its
 * noise: on a fast evaluator the two paths can land in either order.
 */
import {
  analyze,
  evalAnalyzed,
  evaluate,
  Session,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import type { Expr } from "../../packages/ch4/src/syntax/ast.js";
import { call, ident, num } from "../../packages/ch4/src/syntax/ast.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";

/** One benchmark pair: raw runs, medians, and the analysis fraction. */
export interface BenchmarkResult {
  readonly directRuns: ReadonlyArray<number>;
  readonly analyzedRuns: ReadonlyArray<number>;
  readonly directMedian: number;
  readonly analyzedMedian: number;
  readonly fraction: number;
}

const median = (runs: ReadonlyArray<number>): number => {
  const ordered = [...runs].sort((left, right) => left - right);
  return ordered[Math.floor(ordered.length / 2)] ?? 0;
};

const timeRuns = (run: () => Outcome, runs: number): number[] => {
  run();
  run();
  const times: number[] = [];
  for (let i = 0; i < runs; i += 1) {
    const started = performance.now();
    run();
    times.push(performance.now() - started);
  }
  return times;
};

/** The benchmark workload: `fib(14)` over a recursive `fib`. */
export const fibWorkloadSource = `
const fib = (n: number): number => (n < 2 ? n : fib(n - 1) + fib(n - 2));
`;

/** The workload call the two paths time. */
export const fibWorkloadCall = (): Expr => call(ident("fib"), [num(14)]);

/** Runs the comparison: two warmups, seven timed calls per path. */
export const benchmarkAnalysis = (
  setup: string = fibWorkloadSource,
  workload: Expr = fibWorkloadCall(),
): BenchmarkResult => {
  const admission = admitSource(setup);
  const makeEnv = (): Env => {
    const session = new Session("core");
    const env = session.globalEnv();
    if (admission.ok) {
      session.execSequence(admission.program, env);
    }
    return env;
  };
  const directEnv = makeEnv();
  const analyzedEnv = makeEnv();
  const directRuns = timeRuns(() => evaluate(workload, directEnv), 7);
  const analyzedRuns = timeRuns(() => evalAnalyzed(workload, analyzedEnv), 7);
  const directMedian = median(directRuns);
  const analyzedMedian = median(analyzedRuns);
  return {
    directRuns,
    analyzedRuns,
    directMedian,
    analyzedMedian,
    fraction: (directMedian - analyzedMedian) / directMedian,
  };
};

export function ex_4_24(): string {
  return (
    "One recursive workload runs under direct evaluation and under analyze-once-run-many, " +
    "seven timed calls each after two warmups, medians reported with the raw runs. The " +
    "analysis fraction is (direct - analyzed) / direct: the share of direct time " +
    "attributable to re-analyzing syntax. For this workload the share is small and its " +
    "sign is not stable across runs — individual runs spread about as wide as the " +
    "medians' difference — because the engine's per-step work dominates both paths " +
    "equally, so separating analysis from execution buys little here."
  );
}
