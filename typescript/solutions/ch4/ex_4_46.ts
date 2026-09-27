// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.46: left-to-right operands. The section's `getArgs` evaluates
 * the operand execution procedures left to right by construction: an
 * explicit recursion over the operand list, each `aproc` handed a success
 * continuation that recurses on the rest, so the operand order is the walk
 * order and no inherited order is involved. The parsing programs need the
 * order: each `parse-word` consumes the head of `*unparsed*`, so the noun
 * phrase must consume `the professor` before the verb phrase looks at what
 * remains; a right-to-left operand order would make the verb phrase read
 * input the noun phrase has not yet isolated and every parse would fail.
 * The demonstration traces the moment each operand's choice is made with a
 * `note!` bookkeeping procedure that displays its tag before choosing.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

/** The demonstration: each operand announces itself before choosing. */
export const program = `
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(list (begin (note! 'left) (an-element-of '(1 2)))
      (begin (note! 'right) (an-element-of '(1 2))))
`;

export const noteDefinition = `(define (note! tag) (display tag) tag)`;

export interface OrderTrace {
  readonly answers: ReadonlyArray<string>;
  readonly displayed: string;
}

/** Runs the demonstration and collects the first `limit` answers plus the
 * display stream the two operands produced. */
export const trace = (limit: number): Effect.Effect<OrderTrace, EvaluationError> => {
  const displayed: string[] = [];
  return Effect.flatMap(
    setupAmbEnvironment((s) => displayed.push(s)),
    (env) =>
      Effect.map(
        runAmbText(ambEvaluator, [noteDefinition, program].join("\n"), env, limit),
        (run) => ({ answers: run.answers.map(format), displayed: displayed.join("") }),
      ),
  );
};

export function ex_4_46(): string {
  const observed = Effect.runSync(trace(3));
  return (
    "The amb evaluator evaluates an application's operands left to right: " +
    "getArgs walks the operand execution procedures from the front, handing " +
    "each a success continuation that recurses on the rest. The parser needs " +
    "exactly this order, because parse-word consumes the head of " +
    "*unparsed*: a right-to-left evaluator would match the last word of the " +
    "sentence first and every parse would fail. The trace shows the left " +
    `operand's choice made first: the display stream reads ${observed.displayed}, ` +
    "the left tag once for the first answer, the right tag once for it, and " +
    "once more when the third answer's try-again re-enters the left choice " +
    "and replays the right operand expression."
  );
}
