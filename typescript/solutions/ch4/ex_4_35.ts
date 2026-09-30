// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import type { SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
/**
 * Exercise 4.35: an-integer-between and triples. The statement's
 * `an-integer-between` is the bounded generator the search language
 * needs for the triples program: it fails on the empty range and
 * otherwise offers `low` ambiguously against the rest. The choice
 * points sit in the order i, then j from i, then k from j, with the
 * requirement checked at the innermost level.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";

/** The bounded generator: fails on the empty range, else `low` against the rest. */
export const integerBetweenSource = `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
`;

/** The triples program: i, then j from i, then k from j, then the requirement. */
export const triplesSource = `${integerBetweenSource}
const triple = (): number[] => {
  const i = anIntegerBetween(1, 20);
  const j = anIntegerBetween(i, 20);
  const k = anIntegerBetween(j, 20);
  require(i * i + j * j === k * k);
  return [i, j, k];
};
triple();
`;

/** The triples between 1 and 20 in depth-first search order. */
export const answers = (): SearchRun =>
  runAmbAnswers(triplesSource, "amb-depth-first-experiment", 1);

export function ex_4_35(): string {
  return (
    "an-integer-between fails on the empty range and otherwise offers low against the " +
    "rest; the choice points sit in the order i, then j from i, then k from j, with the " +
    "requirement checked at the innermost level. Under depth-first search the triples " +
    "between 1 and 20 come out ordered by ascending i, then j, then k: [3, 4, 5], " +
    "[5, 12, 13], [6, 8, 10], [8, 15, 17], [9, 12, 15], [12, 16, 20]. The order is a " +
    "property of the search, not of the set."
  );
}
