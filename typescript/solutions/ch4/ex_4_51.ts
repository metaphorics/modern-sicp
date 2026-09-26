// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.51: permanent-set!. The variant dispatch recognizes
 * permanent-set! and analyzes it without the undo record, so the trial
 * that eventually fails still leaves its count behind: each answer's count
 * runs ahead of its position because the rejected (a a), (b b), (c c)
 * trials raised the counter too. Under plain set! the undo restores the
 * count before the next alternative, so every answer reads 1.
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

/** The book's counting example under one assignment kind. */
export const countProgram = (kind: string): string =>
  [
    library,
    `(define count 0)
(let ((x (an-element-of '(a b c)))
      (y (an-element-of '(a b c))))
  (${kind} count (+ count 1))
  (require (not (eq? x y)))
  (list x y count))`,
  ].join("\n");

/** All answers of the counting program for one assignment kind. */
export const answers = (kind: string): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(makeAmbEvaluator({ permanentSet: true }), countProgram(kind), env),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_51(): string {
  const permanent = Effect.runSync(answers("permanent-set!"));
  const plain = Effect.runSync(answers("set!"));
  return (
    "permanent-set! assigns without installing the undo record, so the " +
    "trials that fail still leave their count behind: the answers are " +
    `${permanent.join(" ")} -- each count runs ahead of its position ` +
    "because the rejected same-letter trials raised the counter too. Under " +
    "plain set! the undo restores the count before the next alternative, " +
    `so the same program answers ${plain.join(" ")}: every count reads 1.`
  );
}
