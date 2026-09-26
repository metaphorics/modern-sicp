// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.54: require as a special form. The edition keeps the
 * section's require as the user-level procedure definable in the object
 * language, exactly as the book presents it; this solution shows the
 * special-form alternative the statement sketches, completing the two
 * holes of analyze-require: the first blank is the truth test on the
 * predicate's value, the second is the succeed that answers ok with the
 * intercepted failure continuation, so the predicate's own alternatives
 * are tried before the failure propagates.
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

/** The variant evaluator: require recognized in the dispatch. */
export const requireEvaluator = makeAmbEvaluator({ requireForm: true });

/** The even? predicate the pruning demonstration uses. */
export const evenDefinition = `
(define (even? n) (= (remainder n 2) 0))
(define (remainder a b) (if (< a b) a (remainder (- a b) b)))
`;

/** A true require answers ok. */
export const requireTrue = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(requireEvaluator, `${library}(require 1)`, env, 1), (run) =>
      run.answers.map(format),
    ),
  );

/** A false require exhausts: the special form rejects the branch. */
export const requireFalse = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(requireEvaluator, `${library}(require #f)`, env), (run) =>
      run.answers.map(format),
    ),
  );

/** Inside a choice the special form prunes a draw: the evens of (1 2 3 4). */
export const prunedEvens = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        requireEvaluator,
        [
          library,
          evenDefinition,
          `(define (pick-even)
  (let ((x (an-element-of '(1 2 3 4))))
    (require (even? x))
    x))
(pick-even)`,
        ].join("\n"),
        env,
      ),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_54(): string {
  const ok = Effect.runSync(requireTrue());
  const evens = Effect.runSync(prunedEvens());
  return (
    "The edition keeps require as the user-level procedure of 4.3.1; the " +
    "special-form alternative of the statement completes analyze-require " +
    "by testing the predicate's value with isTrue and answering ok with " +
    "the intercepted failure continuation when it holds, so the " +
    "predicate's own alternatives are tried before the failure " +
    `propagates. A true require answers ${ok.join("")}; a false one ` +
    "exhausts the branch outright. Inside a choice the special form " +
    `prunes the draws of (an-element-of (1 2 3 4)) to ${evens.join(" and ")}, ` +
    "indistinguishable to the search from the procedure version, which is " +
    "the statement's point."
  );
}
