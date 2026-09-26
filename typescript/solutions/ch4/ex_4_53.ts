// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.53: permanent-set! under if-fail. The trailing (amb) fails
 * every time, but each failure unwinds to the most recent choice point
 * (the prime-sum-pair choices), so each try-again-driven attempt
 * accumulates the next prime-sum pair into pairs under permanent-set!,
 * whose assignment survives. Only when the combinations are spent does the
 * failure reach if-fail, which succeeds with the accumulated list.
 */
import { Effect } from "effect";

import {
  makeAmbEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { library } from "./ex_4_35.js";

/** The prime? machinery the statement's program assumes defined. */
export const primeDefinitions = `
(define (remainder a b) (if (< a b) a (remainder (- a b) b)))
(define (prime? n)
  (define (smallest-divisor d)
    (cond ((> (* d d) n) n)
          ((= (remainder n d) 0) d)
          (else (smallest-divisor (+ d 1)))))
  (= (smallest-divisor 2) n))
(define (prime-sum-pair list1 list2)
  (let ((a (an-element-of list1))
        (b (an-element-of list2)))
    (require (prime? (+ a b)))
    (list a b)))
`;

/** The statement's program: prime-sum pairs accumulate under a failing amb. */
export const pairsProgram = `
(let ((pairs '()))
  (if-fail (let ((p (prime-sum-pair '(1 3 5 8) '(20 35 110))))
             (permanent-set! pairs (cons p pairs))
             (amb))
           pairs))
`;

/** The accumulated list and whether the search ran dry after it. */
export const result = (): Effect.Effect<{ value: string; exhausted: boolean }, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        makeAmbEvaluator({ permanentSet: true, ifFail: true }),
        library + primeDefinitions + pairsProgram,
        env,
      ),
      (run) => ({
        value: run.answers.map(format).join(""),
        exhausted: run.exhausted,
      }),
    ),
  );

export function ex_4_53(): string {
  const observed = Effect.runSync(result());
  return (
    "The trailing (amb) fails every time, but each failure unwinds to the " +
    "most recent choice point, so each attempt accumulates the next " +
    "prime-sum pair into pairs under permanent-set!, whose assignment " +
    "survives the backtrack. Only when the combinations are spent does the " +
    "failure reach if-fail, which succeeds with the accumulated list: " +
    `${observed.value}, and try-again reports exhaustion ` +
    `${observed.exhausted ? "as predicted" : "unexpectedly"}.`
  );
}
