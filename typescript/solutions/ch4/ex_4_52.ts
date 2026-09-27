// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.52: if-fail. The variant dispatch recognizes if-fail and
 * analyzes it as: the first expression runs against the continuations as
 * usual, and only its outermost failure is intercepted and replaced by the
 * alternative's value; a failure after the first expression has succeeded
 * propagates untouched, which is why the answered value cannot be asked
 * for twice.
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

const evenDefinition = `
(define (even? n) (= (remainder n 2) 0))
(define (remainder a b) (if (< a b) a (remainder (- a b) b)))
`;

/** The book's two if-fail programs. */
export const ifFailProgram = (items: string): string =>
  [
    library,
    evenDefinition,
    `(if-fail (let ((x (an-element-of '(${items}))))
  (require (even? x))
  x)
 'all-odd)`,
  ].join("\n");

/** The answers of one program until the search runs dry. */
export const run = (items: string): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(runAmbText(makeAmbEvaluator({ ifFail: true }), ifFailProgram(items), env), (run) =>
      run.answers.map(format),
    ),
  );

export function ex_4_52(): string {
  const odd = Effect.runSync(run("1 3 5"));
  const with8 = Effect.runSync(run("1 3 5 8"));
  return (
    "if-fail catches the first failure of its first expression once and " +
    "succeeds with its second expression in its place; after the catch the " +
    "handler stands down, so failures later in the branch propagate. " +
    "Without an even element the search runs dry at once and the value is " +
    `${odd.join(", ")}. With 8 in the list the first expression answers 8, ` +
    "the try-again-driven failure falls back to all-odd, and only then " +
    `reports exhaustion (${with8.join(", ")}): the book's transcript, ` +
    "pinned."
  );
}
