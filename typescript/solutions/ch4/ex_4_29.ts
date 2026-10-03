// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { RunResult } from "../../packages/ch4/src/01-metacircular.js";
/**
 * Exercise 4.29: memoization speed difference. The same counting
 * session runs under the section's memoized experiment and under the
 * recomputing twin. `squareL` feeds one thunk to two demand sites and
 * `cubeL` feeds one thunk to three; the counters measure each
 * discipline. The values agree because memoization changes how often
 * the body runs, not what it computes.
 */
import { runLazySource } from "../../packages/ch4/src/02-lazy.js";

/** The counting session: one thunk per call, demanded two and three times. */
export const countingSource = `
let count = 0;
const id = (n: number): number => {
  count = count + 1;
  return n;
};
const squareL = (t: number): number => force(t) * force(t);
const cubeL = (t: number): number => force(t) * force(t) * force(t);
console.log(squareL(delay(id(10))));
console.log(count);
console.log(cubeL(delay(id(10))));
console.log(count);
`;

/** The memoized discipline: each thunk's body runs once. */
export const runMemoized = (): RunResult =>
  runLazySource(countingSource, "lazy-memoized-experiment");

/** The recomputing discipline: every forcing re-evaluates. */
export const runRecompute = (): RunResult =>
  runLazySource(countingSource, "lazy-recompute-experiment");

export function ex_4_29(): string {
  return (
    "One thunk feeds the two demand sites of square and the three of cube. Memoized: " +
    "100 with count 1, then 1000 with count 2. Unmemoized: 100 with count 2, then 1000 " +
    "with count 5. The values agree because memoization changes how often the body runs, " +
    "not what it computes."
  );
}
