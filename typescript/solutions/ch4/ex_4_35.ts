// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.35: an-integer-between and Pythagorean triples. The bounded
 * generator requires the range to be nonempty and then offers `low`
 * ambiguously against the rest; the triple procedure nests three of them so
 * the search walks i, then j from i, then k from j, and requires
 * i^2 + j^2 = k^2 at the innermost level. Under the edition's depth-first
 * search the triples between 1 and 20 come out ordered by ascending i, then
 * j, then k, and the order is a property of the search, not of the set.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

/** The library the section's programs share: require and the generators. */
export const library = `
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (an-integer-starting-from n)
  (amb n (an-integer-starting-from (+ n 1))))
`;

/** The book's triple procedure. */
export const tripleDefinition = `
(define (a-pythagorean-triple-between low high)
  (let ((i (an-integer-between low high)))
    (let ((j (an-integer-between i high)))
      (let ((k (an-integer-between j high)))
        (require (= (+ (* i i) (* j j)) (* k k)))
        (list i j k)))))
`;

/** Every triple between the bounds, in the search's order. */
export const triplesBetween = (
  low: number,
  high: number,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        ambEvaluator,
        [library, tripleDefinition, `(a-pythagorean-triple-between ${low} ${high})`].join("\n"),
        env,
      ),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_35(): string {
  const triples = Effect.runSync(triplesBetween(1, 20));
  return (
    "an-integer-between requires the range to be nonempty and offers low " +
    "ambiguously against the rest, so the nested generators walk i, then j " +
    "from i, then k from j under the depth-first search. Between 1 and 20 " +
    `the evaluator yields ${triples.length} triples in the order ` +
    `${triples.join(" ")}, ordered by ascending i, then j, then k: the ` +
    "order is a property of the search, not of the set."
  );
}
