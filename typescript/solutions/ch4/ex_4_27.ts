// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.27: lazy identity with set!. The sequence is argued from the
 * delay rules and then pinned with the driver transcript. Defining w
 * applies the outer id to the delayed (id 10): evaluating the outer body
 * runs its set! (count 1) and its last expression answers the inner thunk,
 * because delay-it does not evaluate. Asking for w is a demand site: the
 * thunk forces, the inner set! runs (count 2), and 10 fills the memo cell.
 * The re-display forces the same memoized cell and adds nothing, which is
 * the demonstration that the section's evaluator memoizes.
 */
import { Effect } from "effect";

import { lazyDriverWith, lazyEvaluator } from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";

/** The book's interaction, in order. */
export const session = [
  "(define count 0)",
  "(define (id x) (set! count (+ count 1)) x)",
  "(define w (id (id 10)))",
  "count",
  "w",
  "count",
  "w",
  "count",
];

/** The printed values of the session, driven by the lazy driver. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.map(lazyDriverWith(lazyEvaluator, session), (transcript) =>
    transcript.filter((_, i) => i % 4 === 3),
  );

export function ex_4_27(): string {
  const observed = Effect.runSync(answers());
  return (
    "The responses are " +
    "count " +
    `${observed[3]}, w ${observed[4]}, count ${observed[5]}, and after ` +
    `displaying w again: w ${observed[6]} and count ${observed[7]}. ` +
    "Defining w runs the outer id's body: the set! advances count to 1 and " +
    "the body's last expression answers the inner thunk, since delay-it " +
    "does not evaluate. The first demand on w forces the thunk: the inner " +
    "set! advances count to 2 and 10 fills the memo cell. The re-display " +
    "re-forces the evaluated thunk, which answers the stored 10 and runs " +
    "nothing, so count stays 2: the section's evaluator memoizes."
  );
}
