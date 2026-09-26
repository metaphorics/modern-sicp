// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.36: unbounded Pythagorean triples. Simply substituting
 * an-integer-starting-from for an-integer-between in the 4.35 procedure
 * cannot work under chronological depth-first search: the outermost choice
 * (i = 1) never exhausts, so the search never revisits it and no triple
 * ever emerges. The fix keeps the evaluator's search unchanged and restores
 * fairness in the enumeration order: grow the hypotenuse k without bound
 * and search each finite hypotenuse completely with bounded i and j, so
 * every triple with hypotenuse k is produced by the time k is reached.
 * The enumeration key is k, not i, so the order differs from 4.35's.
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

/** Why the naive substitution fails, as a program: i = 1 never exhausts. */
export const naiveDefinition = `
(define (a-pythagorean-triple)
  (let ((i (an-integer-starting-from 1)))
    (let ((j (an-integer-between i 1000000)))
      (let ((k (an-integer-between j 1000000)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
`;

/** The working procedure: the hypotenuse is the unbounded generator. */
export const keyedDefinition = `
(define (a-pythagorean-triple)
  (let ((k (an-integer-starting-from 1)))
    (let ((i (an-integer-between 1 k)))
      (let ((j (an-integer-between i k)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
`;

/** The first `limit` triples of the hypotenuse-keyed enumeration. */
export const firstTriples = (
  limit: number,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        ambEvaluator,
        [library, keyedDefinition, "(a-pythagorean-triple)"].join("\n"),
        env,
        limit,
      ),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_36(): string {
  const triples = Effect.runSync(firstTriples(6));
  return (
    "Simply replacing an-integer-between with an-integer-starting-from in " +
    "the 4.35 procedure cannot work under chronological backtracking: the " +
    "outermost choice i = 1 never exhausts, so the search never revisits it " +
    "and no triple is ever produced. Keeping the search unchanged, the fix " +
    "grows the hypotenuse k without bound and searches each finite " +
    "hypotenuse completely with bounded i and j, so every triple with " +
    "hypotenuse k comes out before k advances. The first six are " +
    `${triples.join(" ")}: the enumeration key is k, not i, which is why ` +
    "the order differs from 4.35's."
  );
}
