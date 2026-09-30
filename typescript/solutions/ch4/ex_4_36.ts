// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
/**
 * Exercise 4.36: unbounded Pythagorean triples. Substituting
 * `an-integer-starting-from` for the bounded generator in 4.35 cannot
 * work under chronological depth-first search: the outermost choice
 * The fix restores fairness in the enumeration order: the procedure
 * grows the hypotenuse without bound and searches each finite
 * hypotenuse completely with bounded `i` and `j`. The demonstration
 * captures the six positive triples through `k = 20` and stops at that
 * known prefix rather than exhausting the unbounded continuation.
 * unbounded one.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";

/** The unbounded hypotenuse enumeration; the driver collects a finite prefix. */
export const hypotenuseTripleSource = `
const anIntegerStartingFrom = (n: number): number => choose(n, anIntegerStartingFrom(n + 1));
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const triple = (): number[] => {
  const k = anIntegerStartingFrom(1);
  require(k <= 20);
  const i = anIntegerBetween(1, k);
  const j = anIntegerBetween(i, k);
  require(i * i + j * j === k * k);
  return [i, j, k];
};
triple();
`;

/** The first six triples from the hypotenuse-keyed unbounded enumeration. */
export const answers = (): SearchRun =>
  runAmbAnswers(hypotenuseTripleSource, "amb-depth-first-experiment", 1, { maxAnswers: 6 });

export function ex_4_36(): string {
  return (
    "The unbounded generator in the outer choice starves the search; growing the " +
    "hypotenuse without bound and searching each finite k completely restores fairness. " +
    "The first answers come out keyed by k: [3, 4, 5], [6, 8, 10], [5, 12, 13], " +
    "[9, 12, 15], [8, 15, 17], [12, 16, 20] — the order differs from 4.35's because the " +
    "enumeration key is k, not i."
  );
}
