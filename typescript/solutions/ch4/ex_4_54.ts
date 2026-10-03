// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.54: require as a special form. A plain boolean result,
 * even computed correctly, changes nothing when ignored — all four
 * choices survive. Only the require node redirects the failure
 * continuation and kills the branch. Both halves run for real on
 * the delivered engine.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** Choices filtered by the require node. */
export const requiredSource = `
const x = choose(1, 2, 3, 4);
require(x > 2);
x;
`;

/** The same condition computed and ignored: no filtering. */
export const ignoredSource = `
const y = choose(1, 2, 3, 4);
const checked = y > 2;
y;
`;

/** The require-filtered answers, rendered. */
export const requiredAnswers = (): ReadonlyArray<string> =>
  runAmbAnswers(requiredSource, "amb-depth-first-experiment", 1).answers.map((value) =>
    format(value),
  );

/** The unfiltered answers when the boolean is ignored, rendered. */
export const ignoredAnswers = (): ReadonlyArray<string> =>
  runAmbAnswers(ignoredSource, "amb-depth-first-experiment", 1).answers.map((value) =>
    format(value),
  );

export function ex_4_54(): string {
  const kept = requiredAnswers();
  const all = ignoredAnswers();
  return (
    `Require keeps ${kept.length} of ${all.length} choices: a computed ` +
    "boolean changes nothing when ignored, because only the require " +
    "node reaches the failure continuation. That reach is why require " +
    "is a special form, not a procedure."
  );
}
