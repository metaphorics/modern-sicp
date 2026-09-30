// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.51: reversible set! versus permanent assignment. The
 * same counting program is run twice: choice-sensitive `=` restores
 * the prior value when a branch fails, while `permanentAssign`
 * leaves it for later choices. Rejected equal pairs therefore do
 * not affect the `set!` answers (all count 1), but they advance the
 * permanent count (2,3,4,6,7,8).
 */
import { runAmbAnswers, type SearchRun } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

export type AssignmentKind = "reversible" | "permanent";

/** The book's counter experiment, changing only its assignment form. */
export const countSource = (kind: AssignmentKind): string => `
let count = 0;
const x = choose("a", "b", "c");
const y = choose("a", "b", "c");
${kind === "reversible" ? "count = count + 1;" : "permanentAssign(count, count + 1);"}
require(!(x === y));
[x, y, count];
`;

/** The actual evaluator run for one assignment discipline. */
export const countRun = (kind: AssignmentKind): SearchRun =>
  runAmbAnswers(countSource(kind), "amb-depth-first-experiment", 1);

/** Rendered answers for one discipline. */
export const countAnswers = (kind: AssignmentKind): ReadonlyArray<string> =>
  countRun(kind).answers.map((value) => format(value));

export function ex_4_51(): string {
  const reversible = countRun("reversible");
  const permanent = countRun("permanent");
  const reversibleAnswers = reversible.answers.map((value) => format(value));
  const permanentAnswers = permanent.answers.map((value) => format(value));
  return (
    `Reversible set! answers: ${reversibleAnswers.join("; ")}. Permanent assignment: ` +
    `${permanentAnswers.join("; ")}. The runs record ${reversible.failures} and ` +
    `${permanent.failures} failed computations; rejected trials restore only ` +
    `the reversible write.`
  );
}
