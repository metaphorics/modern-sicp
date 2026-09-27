// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.37: Ben Bitdiddle's triple generator. Ben is right about the
 * search size: computing hsq = high^2 up front, his `(require (>= hsq ksq))`
 * rejects a candidate j before the k loop is ever entered, so fewer choices
 * reach the require machinery on the way to the first triple. Both
 * generators answer the same first triple (3 4 5); the counts, not the
 * answer, are the difference. Because the edition's exact integer language
 * has no floating sqrt, `(sqrt ksq)` is the floored root and Ben's
 * `(require (integer? k))` becomes the equivalent squaring-back check
 * `(require (= ksq (* k k)))`, exactly the rounding the float version
 * performs implicitly.
 */
import { Effect } from "effect";

import {
  makeAmbEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import { format } from "../../packages/ch4/src/read.js";

import { library, tripleDefinition } from "./ex_4_35.js";

/** The floored integer square root the demonstration uses. */
export const isqrtDefinition = `
(define (isqrt n)
  (define (loop k) (if (> (* k k) n) (- k 1) (loop (+ k 1))))
  (loop 1))
`;

/** Ben's generator, with the integrality check squared back onto ksq. */
export const benDefinition = `
(define (a-pythagorean-triple-between low high)
  (let ((i (an-integer-between low high))
        (hsq (* high high)))
    (let ((j (an-integer-between i high)))
      (let ((ksq (+ (* i i) (* j j))))
        (require (>= hsq ksq))
        (let ((k (isqrt ksq)))
          (require (= ksq (* k k)))
          (list i j k))))))
`;

export const gteDefinition = `(define (>= x y) (not (< x y)))`;

/** The first answer of one generator together with the number of Fail
 * deliveries the choice frames saw on the way. */
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

export function ex_4_37(): string {
  const plain = Effect.runSync(
    firstWithFailures(
      [library, tripleDefinition, "(a-pythagorean-triple-between 1 20)"].join("\n"),
    ),
  );
  const ben = Effect.runSync(
    firstWithFailures(
      [
        library,
        gteDefinition,
        isqrtDefinition,
        benDefinition,
        "(a-pythagorean-triple-between 1 20)",
      ].join("\n"),
    ),
  );
  return (
    "Ben is correct about the search size. Between the bounds 1 and 20 his " +
    "hsq prune rejects a candidate j before the k loop is ever entered, so " +
    "fewer require failures reach the choice handlers. Measured with the " +
    "evaluator's failures knob on the way to the first triple, the 4.35 " +
    `order delivers ${plain.failuresToFirst} failures and Ben's version ` +
    `${ben.failuresToFirst}; both produce the same first triple ` +
    `${plain.answer}. The counts, not the answer, are the difference.`
  );
}
