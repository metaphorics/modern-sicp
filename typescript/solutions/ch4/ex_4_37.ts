// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
/**
 * Exercise 4.37: Ben's generator. Ben is right about the search size:
 * his `hsq` prune rejects a candidate `j` before the hypotenuse search
 * is ever entered, so fewer failures reach the choice handlers. The
 * guest has no floating `sqrt`, so the root is the floored integer
 * root and Ben's integrality requirement is the squaring-back check
 * `k * k === hsq`: the same rounding the float version performs
 * implicitly.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { triplesSource } from "./ex_4_35.js";

/** Ben's pruned generator: no search over k at all. */
export const benTripleSource = `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const isqrt = (n: number): number => {
  let r = 0;
  while ((r + 1) * (r + 1) <= n) {
    r = r + 1;
  }
  return r;
};
const benTriple = (): number[] => {
  const i = anIntegerBetween(1, 20);
  const j = anIntegerBetween(i, 20);
  const hsq = i * i + j * j;
  const k = isqrt(hsq);
  require(k <= 20);
  require(k * k === hsq);
  return [i, j, k];
};
benTriple();
`;

/** Ben's answers: the same triples as 4.35, reached with fewer failures. */
export const answers = (): SearchRun =>
  runAmbAnswers(benTripleSource, "amb-depth-first-experiment", 1);

/** The baseline program from Exercise 4.35. */
export const ordinaryAnswers = (): SearchRun =>
  runAmbAnswers(triplesSource, "amb-depth-first-experiment", 1);

/** Real search counters for the baseline and Ben's pruned generator. */
export const failureCounts = (): readonly [number, number] => {
  const ordinary = ordinaryAnswers();
  const ben = answers();
  return [ordinary.failures, ben.failures];
};

export function ex_4_37(): string {
  const [ordinary, ben] = failureCounts();
  return (
    "Ben is right about the search size: his hsq prune rejects a candidate j before the " +
    "hypotenuse search is ever entered, so fewer backtracking continuations are queued. " +
    "The integrality check is the squaring-back test k * k === hsq over the floored root. " +
    `Both programs produce the same six triples; measured deferred-backtrack counts are ${ordinary} ` +
    `for the ordinary generator and ${ben} for Ben's.`
  );
}
