// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.29: memoization speed difference. The same counting session
 * runs under the section's memoized evaluator and under a twin whose
 * delayOperand always answers false, so every operand delays into a
 * recomputing wrapper and every forcing re-evaluates. (square (id 10))
 * feeds one thunk to the two demand sites of *, and (cube (id 10)) feeds
 * one thunk to three demand sites; the counters measure each discipline.
 * The values agree because memoization changes how often the body runs,
 * not what it computes.
 */
import { Effect } from "effect";

import {
  type LazyEvaluator,
  lazyDriverWith,
  lazyEvaluator,
  makeLazyEvaluator,
} from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

export const memoizedSession = [
  "(define count 0)",
  "(define (id x) (set! count (+ count 1)) x)",
  "(define (square x) (* x x))",
  "(square (id 10))",
  "count",
  "(define (cube x) (* x (* x x)))",
  "(cube (id 10))",
  "count",
];

/** The section's twin without memoization: every compound operand
 * delays into a wrapper that recomputes at each demand. */
export const unmemoizedEvaluator: LazyEvaluator = makeLazyEvaluator({
  delayOperand: () => false,
});

const valuesOf = (
  evaluator: LazyEvaluator,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.map(lazyDriverWith(evaluator, memoizedSession), (transcript) =>
    transcript.filter((_, i) => i % 4 === 3),
  );

/** The session's answers under each discipline: value, count, value,
 * count. */
export const answers = (): Effect.Effect<
  { readonly memoized: ReadonlyArray<string>; readonly unmemoized: ReadonlyArray<string> },
  EvaluationError
> =>
  Effect.flatMap(valuesOf(lazyEvaluator), (memoized) =>
    Effect.map(valuesOf(unmemoizedEvaluator), (unmemoized) => ({
      memoized: memoized.slice(3).filter((value) => value !== "ok"),
      unmemoized: unmemoized.slice(3).filter((value) => value !== "ok"),
    })),
  );

export function ex_4_29(): string {
  const observed = Effect.runSync(answers());
  const [squareMemo, countMemo, cubeMemo, countCubeMemo] = observed.memoized;
  const [squareRaw, countRaw, cubeRaw, countCubeRaw] = observed.unmemoized;
  return (
    "The program is the counting identity of exercise 4.27 fed to square " +
    "and cube, where each occurrence of the parameter is a separate demand " +
    "site: two for (* x x) and three for (* x (* x x)). Memoized, the " +
    `session answers ${squareMemo}, count ${countMemo}, then ${cubeMemo}, ` +
    `count ${countCubeMemo}: each forcing after the first reads the stored ` +
    "value. Without memoization the same session answers " +
    `${squareRaw}, count ${countRaw}, then ${cubeRaw}, count ${countCubeRaw}: ` +
    "every forcing re-runs the body, so the extra occurrences each advance " +
    "the counter again. The wall-clock analogue is the same slowdown: the " +
    "unmemoized evaluator recomputes what the memoized one remembers."
  );
}
