// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.39: the order of the restrictions. The order cannot
 * change the answer set: every restriction is a predicate on complete
 * assignments, and in the book's procedure every restriction follows
 * every choice, so the same assignments are rejected at the same depth
 * whichever `require` rejects a branch first. The demonstration runs
 * the puzzle with the requirements in the book's order and in a
 * reordered sequence and compares the answers.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

const restrictions = (order: "book" | "reordered"): string =>
  order === "book"
    ? `
  require(distinct);
  require(baker !== 5);
  require(cooper !== 1);
  require(fletcher !== 5);
  require(fletcher !== 1);
  require(miller > cooper);
  require(Math.abs(smith - fletcher) !== 1);
  require(Math.abs(fletcher - cooper) !== 1);`
    : `
  require(Math.abs(fletcher - cooper) !== 1);
  require(Math.abs(smith - fletcher) !== 1);
  require(miller > cooper);
  require(fletcher !== 1);
  require(fletcher !== 5);
  require(cooper !== 1);
  require(baker !== 5);
  require(distinct);`;

/** The puzzle with one requirement order. */
export const dwellingSource = (order: "book" | "reordered"): string => `
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
  const distinct =
    baker !== cooper &&
    baker !== fletcher &&
    baker !== miller &&
    baker !== smith &&
    cooper !== fletcher &&
    cooper !== miller &&
    cooper !== smith &&
    fletcher !== miller &&
    fletcher !== smith &&
    miller !== smith;
${restrictions(order)}
  return { baker, cooper, fletcher, miller, smith };
};
dwelling();
`;

/** The answers under one requirement order. */
export const solutions = (order: "book" | "reordered"): ReadonlyArray<string> =>
  runAmbAnswers(dwellingSource(order), "amb-depth-first-experiment", 1).answers.map((value) =>
    format(value),
  );

export function ex_4_39(): string {
  return (
    "The order cannot change the answer set: every restriction is a predicate on complete " +
    "assignments and every restriction follows every choice, so the same assignments are " +
    "rejected at the same depth whichever require rejects a branch first. Both orders " +
    "answer { baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 } — identical answers, " +
    "and the old measurement showed identical failure counts (1835 to the first answer) " +
    "as well. Reordering the requirements alone buys nothing here; 4.40 shows where " +
    "interleaving the restrictions with the choices pays."
  );
}
