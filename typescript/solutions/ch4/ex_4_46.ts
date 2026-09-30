// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.46: left-to-right operands. The operand order is the walk
 * order: each operand's announcement lands before its choice, so the
 * display stream records which operand the search resumed from. The
 * parsing programs need the order: each parse-word consumes the head
 * of the input, so the noun phrase must consume "the professor" before
 * the verb phrase looks at the rest; a right-to-left operand order
 * would make the verb phrase read input the noun phrase has not yet
 * isolated, and every parse would fail.
 */
import { runAmbAnswers } from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

/** The order demonstration: two choices, each announcing before choosing. */
export const operandOrderSource = `
const demo = (): number[] => {
  console.log("left");
  const x = choose(1, 2);
  console.log("right");
  const y = choose(1, 2);
  return [x, y];
};
demo();
`;

/** The run: the answers and the announcement stream over them. */
export const answers = (): {
  readonly values: ReadonlyArray<string>;
  readonly stream: ReadonlyArray<string>;
} => {
  const run = runAmbAnswers(operandOrderSource, "amb-depth-first-experiment", 1);
  return {
    values: run.answers.map((value) => format(value)),
    stream: run.transcript,
  };
};
export function ex_4_46(): string {
  return (
    "The answers enumerate left-major: [1, 1], [1, 2], [2, 1], [2, 2]. The announcement " +
    "stream reads left, right, right: the left operand announces once, the " +
    "right operand announces once per visit to its choice point — on the " +
    "first answer and again when resumption passes back through the left " +
    "choice. Resuming the right choice alone re-announces nothing, so the " +
    "second and fourth answers add no log lines — the resumable-frame " +
    "property the book's failure continuations give."
  );
}
