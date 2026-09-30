// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.41 (adapted row, reworded): the statement asks for an
 * ordinary program in the reader's working language; this edition
 * reads that as TypeScript, so the solution is a plain host function —
 * enumeration of the 120 permutations of the five floors over the five
 * people, filtered by the puzzle's restrictions. No evaluator, no
 * search machinery.
 */
/** The puzzle's restrictions on one complete assignment. */
const acceptable = (
  baker: number,
  cooper: number,
  fletcher: number,
  miller: number,
  smith: number,
): boolean =>
  new Set([baker, cooper, fletcher, miller, smith]).size === 5 &&
  baker !== 5 &&
  cooper !== 1 &&
  fletcher !== 5 &&
  fletcher !== 1 &&
  miller > cooper &&
  Math.abs(smith - fletcher) !== 1 &&
  Math.abs(fletcher - cooper) !== 1;

/** The ordinary program: every permutation, filtered. */
export const solveDwelling = (): ReadonlyArray<Record<string, number>> => {
  const found: Array<Record<string, number>> = [];
  for (const baker of [1, 2, 3, 4, 5]) {
    for (const cooper of [1, 2, 3, 4, 5]) {
      for (const fletcher of [1, 2, 3, 4, 5]) {
        for (const miller of [1, 2, 3, 4, 5]) {
          for (const smith of [1, 2, 3, 4, 5]) {
            if (acceptable(baker, cooper, fletcher, miller, smith)) {
              found.push({ baker, cooper, fletcher, miller, smith });
            }
          }
        }
      }
    }
  }
  return found;
};

export function ex_4_41(): string {
  return (
    "The puzzle language is reworded into the working language: a plain host function " +
    "enumerates the 120 permutations of the five floors over the five people and filters " +
    "them by the puzzle's restrictions. It answers exactly one solution, baker 3, " +
    "cooper 2, fletcher 4, miller 5, smith 1 — the search program's answer too. The " +
    "exercise's point: the nondeterministic program buys notation, not power."
  );
}
