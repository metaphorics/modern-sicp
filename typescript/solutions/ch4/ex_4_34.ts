// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.34: printing lazy pairs. The representation changes so the
 * evaluator can identify a lazy pair: under the printablePairs tuning,
 * (cons a b) builds the tagged two-thunk list (lazy-pair <thunk> <thunk>)
 * instead of running the strict constructor, car and cdr address its
 * forced slots, and the driver prints with the budgeted lazy renderer.
 * The print rule: a lazy list prints its first ten elements, each forced
 * once, and the unprinted tail prints as the ellipsis, so an infinite
 * list renders finitely and the printer never forces past the budget nor
 * the tail of an unprinted element. Nested lazy pairs print inside their
 * parent's parentheses with their own budget.
 */
import { Effect } from "effect";
import { makeLazyEvaluator, printableDriverLoop } from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

/** The lazy evaluator whose cons builds printable lazy pairs. */
export const printableEvaluator = makeLazyEvaluator({ printablePairs: true });

/** The book's session: a finite pair, the infinite ones, a demand on it,
 * and a nested pair. */
export const session = [
  "(cons 1 (cons 2 '()))",
  "(define ones (cons 1 ones))",
  "ones",
  "(car ones)",
  "(cons (cons 1 '()) (cons 2 '()))",
];

/** The printed values of the session. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.map(printableDriverLoop(printableEvaluator, session), (transcript) =>
    transcript.filter((_, i) => i % 4 === 3),
  );

export function ex_4_34(): string {
  const observed = Effect.runSync(answers());
  return (
    "The representation is tagged, the move the statement suggests for " +
    "making lazy pairs identifiable: (cons a b) builds the two-thunk list " +
    "(lazy-pair <thunk> <thunk>), car and cdr address its forced slots, " +
    "and the driver prints with a budgeted renderer. The session prints " +
    `${observed[0]} for the finite pair, ok for the self-referential ` +
    `definition, ${observed[2]} for the infinite ones (ten forced elements ` +
    "and the ellipsis, which is the answer to the parenthetical question), " +
    `${observed[3]} for a demand on the list, and ${observed[4]} for the ` +
    "nested pair, whose inner pairs print inside the parent's parentheses " +
    "with their own budget. Printing forces only what it prints, so it is " +
    "itself a forcing site of the language."
  );
}
