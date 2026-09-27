// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.30: forcing in eval-sequence. The edition's rule: sequences
 * evaluate every expression and force none of them, because forcing
 * happens at the point of use, which is a primitive's operands, an if
 * predicate, an operator position, or the driver. Cy D. Fect's proposal
 * forces the non-final expressions instead; the tuning cySequence builds
 * his evaluator. The session runs the book's for-each under both
 * evaluators (part a and part c: the behavior is identical, because the
 * effects flow through an application whose primitive operands force),
 * then pins (p1 1) and (p2 1) under both (part b: the sequence rule is
 * the only thing that can differ), and answers part d by keeping the
 * text's rule for the section.
 */
import { Effect } from "effect";

import {
  type LazyEvaluator,
  lazyDriverWith,
  lazyEvaluator,
  makeLazyEvaluator,
} from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

export const forEachSession = [
  "(define (for-each proc items) (if (null? items) 'done (begin (proc (car items)) (for-each proc (cdr items)))))",
  "(for-each (lambda (x) (newline) (display x)) (list 57 321 88))",
];

export const p1Session = ["(define (p1 x) (set! x (cons x '(2))) x)", "(p1 1)"];

export const p2Session = [
  "(define (p2 x) (define (p e) e x) (p (set! x (cons x '(2)))))",
  "(p2 1)",
];

/** Cy's evaluator: the only change is the sequence rule. */
export const cyEvaluator: LazyEvaluator = makeLazyEvaluator({ cySequence: true });

/** One session under one evaluator: the printed display output and the
 * session's last value. */
const runSession = (
  evaluator: LazyEvaluator,
  session: ReadonlyArray<string>,
): Effect.Effect<{ readonly printed: string; readonly last: string }, EvaluationError> => {
  const printed: string[] = [];
  return Effect.map(
    lazyDriverWith(evaluator, session, (chunk) => printed.push(chunk)),
    (transcript) => ({
      printed: printed.join(""),
      last: transcript[transcript.length - 1] ?? "",
    }),
  );
};

/** The four observed runs, in the order the parts ask for them. */
export const answers = (): Effect.Effect<
  {
    readonly forEachText: string;
    readonly forEachCy: string;
    readonly p1Text: string;
    readonly p1Cy: string;
    readonly p2Text: string;
    readonly p2Cy: string;
  },
  EvaluationError
> =>
  Effect.flatMap(runSession(lazyEvaluator, forEachSession), (forEachText) =>
    Effect.flatMap(runSession(cyEvaluator, forEachSession), (forEachCy) =>
      Effect.flatMap(runSession(lazyEvaluator, p1Session), (p1Text) =>
        Effect.flatMap(runSession(cyEvaluator, p1Session), (p1Cy) =>
          Effect.flatMap(runSession(lazyEvaluator, p2Session), (p2Text) =>
            Effect.map(runSession(cyEvaluator, p2Session), (p2Cy) => ({
              forEachText: `${forEachText.printed}|${forEachText.last}`,
              forEachCy: `${forEachCy.printed}|${forEachCy.last}`,
              p1Text: p1Text.last,
              p1Cy: p1Cy.last,
              p2Text: p2Text.last,
              p2Cy: p2Cy.last,
            })),
          ),
        ),
      ),
    ),
  );

export function ex_4_30(): string {
  const observed = Effect.runSync(answers());
  return (
    "Ben is right about for-each: the mapped lambda is a compound procedure " +
    "whose application the evaluator's own rules drive, the effect " +
    "statements inside it are evaluated as the body sequence goes, and " +
    "display's operands force because primitives are strict, so the " +
    `session prints ${observed.forEachText} under the text's rule and ` +
    `${observed.forEachCy} under Cy's, which is part (c): his change only ` +
    "forces expressions the evaluator already evaluates, so an effect that " +
    "flows through an application behaves identically. The sequences that " +
    `differ are p1 and p2: (p1 1) answers ${observed.p1Text} under both, ` +
    "because the set! is evaluated either way; (p2 1) answers " +
    `${observed.p2Text} under the text's rule, where the set! hides in the ` +
    `delayed operand of p and never runs, and ${observed.p2Cy} under Cy's, ` +
    "where forcing the first expression of p's body runs it. The section " +
    "keeps the text's rule: sequence positions evaluate without forcing, " +
    "so a thunk in a discarded position is genuinely unused, while Cy's " +
    "variant silently runs computations the text's rule would have dropped."
  );
}
