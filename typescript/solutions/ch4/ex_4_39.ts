// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.39: the order of the restrictions. In the book's procedure
 * every restriction follows every choice, so the search tree has the same
 * shape whichever `require` rejects a branch first: the same assignments
 * are rejected at the same depth, and the answer set cannot change. The
 * measurement makes that exact: both orders deliver the identical failure
 * count to the choice frames on the way to the same unique answer. Where
 * ordering does pay is interleaving the restrictions with the choices,
 * which is exercise 4.40's demonstration.
 */
import { Effect } from "effect";

import {
  makeAmbEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

import { puzzleLibrary } from "./ex_4_38.js";

/** The library plus the book's restriction order. */
export const bookOrder = `
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(multiple-dwelling)
`;

/** The same choices with the most selective restrictions first. */
export const reorderedOrder = `
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(multiple-dwelling)
`;

/** The first answer of one order with the failure count on the way. */
export const firstWithFailures = (
  program: string,
): Effect.Effect<{ answer: string; failuresToFirst: number }, never> => {
  const failures = { count: 0 };
  const evaluator = makeAmbEvaluator({ failures });
  return Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(Effect.result(runAmbText(evaluator, program, env, 1)), (outcome) => ({
      answer:
        outcome._tag === "Failure"
          ? `error ${outcome.failure._tag}`
          : outcome.success.answers.map(format).join(","),
      failuresToFirst: failures.count,
    })),
  );
};

const withLibrary = (program: string): string => [puzzleLibrary, program].join("\n");

export function ex_4_39(): string {
  const book = Effect.runSync(firstWithFailures(withLibrary(bookOrder)));
  const reordered = Effect.runSync(firstWithFailures(withLibrary(reorderedOrder)));
  return (
    "The order of the restrictions cannot change the answer: every " +
    "restriction is a predicate on complete assignments, and with all the " +
    "choices made first the search tree has the same shape in both orders, " +
    "so the same assignments are rejected at the same depth. The " +
    "measurement makes that exact: the book's order answers " +
    `${book.answer} after ${book.failuresToFirst} failures and the ` +
    "reordered program answers " +
    `${reordered.answer} after ${reordered.failuresToFirst} -- identical ` +
    "counts, identical answer. Reordering the requirements alone buys " +
    "nothing here; exercise 4.40 shows where interleaving the restrictions " +
    "with the choices does pay."
  );
}
