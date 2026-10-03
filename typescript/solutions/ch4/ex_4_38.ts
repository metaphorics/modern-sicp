// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.38: multiple dwelling without the Smith-Fletcher clause.
 * Dropping the requirement that Smith and Fletcher not live on
 * adjacent floors loosens the puzzle and the solution set grows from
 * one to five assignments. The demonstration enumerates all five by
 * search and counts them two ways: the search itself and an
 * independent brute-force loop over the 120 complete floor
 * assignments, so the count does not rest on the evaluator.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** The book's procedure; the flag omits the one dropped clause. */
export const dwellingSource = (smithFletcher: boolean): string => `
const anIntegerBetween = (low: number, high: number): number => {
  require(low <= high);
  return choose(low, anIntegerBetween(low + 1, high));
};
const dwelling = (): Record<string, number> => {
  const baker = anIntegerBetween(1, 5);
  const cooper = anIntegerBetween(1, 5);
  const fletcher = anIntegerBetween(1, 5);
  const miller = anIntegerBetween(1, 5);
  const smith = anIntegerBetween(1, 5);
  require(
    baker !== cooper &&
      baker !== fletcher &&
      baker !== miller &&
      baker !== smith &&
      cooper !== fletcher &&
      cooper !== miller &&
      cooper !== smith &&
      fletcher !== miller &&
      fletcher !== smith &&
      miller !== smith,
  );
  require(baker !== 5);
  require(cooper !== 1);
  require(fletcher !== 5);
  require(fletcher !== 1);
  require(miller > cooper);
  ${smithFletcher ? "require(Math.abs(smith - fletcher) !== 1);" : ""}
  require(Math.abs(fletcher - cooper) !== 1);
  return { baker, cooper, fletcher, miller, smith };
};
dwelling();
`;

/** Every solution of the flagged puzzle, in search order. */
export const solutions = (smithFletcher: boolean): ReadonlyArray<string> =>
  runAmbAnswers(dwellingSource(smithFletcher), "amb-depth-first-experiment", 1).answers.map(
    (value) => format(value),
  );

/** The independent count: a plain host loop over the 120 assignments. */
export const bruteForceCount = (smithFletcher: boolean): number => {
  let count = 0;
  for (const baker of [1, 2, 3, 4, 5]) {
    for (const cooper of [1, 2, 3, 4, 5]) {
      for (const fletcher of [1, 2, 3, 4, 5]) {
        for (const miller of [1, 2, 3, 4, 5]) {
          for (const smith of [1, 2, 3, 4, 5]) {
            const distinct = new Set([baker, cooper, fletcher, miller, smith]).size === 5;
            const smithClause = !smithFletcher || Math.abs(smith - fletcher) !== 1;
            if (
              distinct &&
              baker !== 5 &&
              cooper !== 1 &&
              fletcher !== 5 &&
              fletcher !== 1 &&
              miller > cooper &&
              smithClause &&
              Math.abs(fletcher - cooper) !== 1
            ) {
              count += 1;
            }
          }
        }
      }
    }
  }
  return count;
};

export function ex_4_38(): string {
  return (
    "Dropping the Smith-Fletcher adjacency clause loosens the puzzle and the solution set " +
    "grows from one to five assignments: baker 1/cooper 2/fletcher 4/miller 3/smith 5, " +
    "then miller 5/smith 3, then cooper 4/fletcher 2, then the book's baker 3/cooper " +
    "2/fletcher 4/miller 5/smith 1, then baker 3/cooper 4/fletcher 2/miller 5/smith 1. " +
    "The search count matches an independent brute-force loop over the 120 complete " +
    "assignments: 5 without the clause, 1 with it."
  );
}
