// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.42: the Liars puzzle. Each girl makes one true and one untrue
 * statement, so each pair of claims is constrained by `exactly-one`: the
 * two statements of a pair must disagree. Betty's "Kitty was second, I was
 * third" becomes `(require (exactly-one (= kitty 2) (= betty 3)))`, and so
 * on for the four other pairs; the five positions are drawn with
 * an-integer-between under a distinctness requirement.
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

/** The puzzle program: exactly-one over each girl's pair of claims. */
export const liarsProgram = `
(define (exactly-one a b) (if a (not b) b))
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (member x xs)
  (cond ((null? xs) #f)
        ((equal? x (car xs)) xs)
        (else (member x (cdr xs)))))
(define (solve-liars)
  (let ((betty (an-integer-between 1 5)) (ethel (an-integer-between 1 5))
        (joan (an-integer-between 1 5)) (kitty (an-integer-between 1 5))
        (mary (an-integer-between 1 5)))
    (require (distinct? (list betty ethel joan kitty mary)))
    (require (exactly-one (= kitty 2) (= betty 3)))
    (require (exactly-one (= ethel 1) (= joan 2)))
    (require (exactly-one (= joan 3) (= ethel 5)))
    (require (exactly-one (= kitty 2) (= mary 4)))
    (require (exactly-one (= mary 4) (= betty 1)))
    (list (list 'betty betty) (list 'ethel ethel) (list 'joan joan)
          (list 'kitty kitty) (list 'mary mary))))
`;

/** Every solution of the puzzle, in search order. */
export const solutions = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(ambEvaluator, [library, liarsProgram, "(solve-liars)"].join("\n"), env),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_42(): string {
  const found = Effect.runSync(solutions());
  if (found.length !== 1) {
    throw new Error(`expected exactly one solution, got ${found.length}`);
  }
  return (
    "exactly-one requires the pair of statements each girl made to " +
    "disagree: `(if a (not b) b)`. The five positions are chosen with " +
    "an-integer-between under a distinctness requirement, and each girl's " +
    "pair of claims becomes one require of exactly-one: Betty's statement " +
    "is `(exactly-one (= kitty 2) (= betty 3))`, and so on for the other " +
    "four. The evaluator's first answer is " +
    `${found[0]}: Kitty first, Joan second, Betty third, Mary fourth, ` +
    "Ethel fifth, and try-again reports exhaustion, so the assignment is " +
    "the unique one."
  );
}
