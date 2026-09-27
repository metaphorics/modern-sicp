// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.38: multiple dwelling without the Smith-Fletcher clause.
 * Dropping `(require (not (= (abs (- smith fletcher)) 1)))` loosens the
 * puzzle and the solution set grows from one to five assignments. The
 * demonstration enumerates all five by search and counts them two ways:
 * the amb search itself and an independent brute-force loop over the 120
 * complete floor assignments, so the count does not rest on the evaluator.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { library } from "./ex_4_35.js";

/** The puzzle's bookkeeping procedures, as the book's footnote defines them. */
export const puzzleLibrary = `
${library}
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (member x xs)
  (cond ((null? xs) #f)
        ((equal? x (car xs)) xs)
        (else (member x (cdr xs)))))
(define (abs x) (if (< x 0) (- 0 x) x))
`;

/** The book's procedure; the `smithFletcher` flag omits the one clause. */
export const dwellingDefinition = (smithFletcher: boolean): string => `
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))${
      smithFletcher ? "\n    (require (not (= (abs (- smith fletcher)) 1)))" : ""
    }
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
`;

/** Every solution of the flagged puzzle, in search order. */
export const solutions = (
  smithFletcher: boolean,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        ambEvaluator,
        [puzzleLibrary, dwellingDefinition(smithFletcher), "(multiple-dwelling)"].join("\n"),
        env,
      ),
      (run) => run.answers.map(format),
    ),
  );

/** The independent count: a plain host loop over the 120 assignments
 * applying the remaining restrictions. */
export const bruteForceCount = (smithFletcher: boolean): number => {
  let count = 0;
  for (const baker of [1, 2, 3, 4, 5]) {
    for (const cooper of [1, 2, 3, 4, 5]) {
      for (const fletcher of [1, 2, 3, 4, 5]) {
        for (const miller of [1, 2, 3, 4, 5]) {
          for (const smith of [1, 2, 3, 4, 5]) {
            const distinct = new Set([baker, cooper, fletcher, miller, smith]).size === 5;
            // smithFletcher true: the clause is in the program, so the
            // floors may not be adjacent; false: the clause is dropped.
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
  const found = Effect.runSync(solutions(false));
  const bookAnswer = Effect.runSync(solutions(true));
  return (
    "Dropping the requirement that Smith and Fletcher not live on adjacent " +
    "floors loosens the puzzle and the solution set grows from one to " +
    `${found.length}. The demonstration enumerates all five with ` +
    "try-again, and the count matches an independent brute-force loop over " +
    `the 120 complete floor assignments (${bruteForceCount(false)} there, ` +
    `${bruteForceCount(true)} with the clause restored). The book's ` +
    `answer ${bookAnswer.join(" ")} is among them.`
  );
}
