// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.40: pruning before the restrictions. The counting identities:
 * before the distinctness requirement there are 5^5 = 3125 sets of
 * assignments; after it, 5! = 120. The pruned procedure also moves each
 * restriction in front of the choices it does not mention: Cooper draws
 * from the four floors the bottom-floor rule allows, Fletcher from the
 * three neither top nor bottom, Miller draws above Cooper directly, and
 * the adjacency and distinctness requirements run as soon as the people
 * they mention have drawn. The answer is the book's either way; the pruned
 * search delivers a fraction of the failures on the way.
 */
import { Effect } from "effect";

import {
  ambEvaluator,
  makeAmbEvaluator,
  runAmbText,
  setupAmbEnvironment,
} from "../../packages/ch4/src/03-nondeterministic.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";

import { puzzleLibrary } from "./ex_4_38.js";

/** The naive procedure of the section (both adjacency clauses). */
export const naiveDefinition = `
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
`;

/** The pruned procedure: a nest of lets, each restriction where it binds. */
export const prunedDefinition = `
(define (multiple-dwelling-faster)
  (let ((cooper (amb 2 3 4 5)))
    (let ((fletcher (amb 2 3 4)))
      (require (not (= (abs (- fletcher cooper)) 1)))
      (let ((baker (amb 1 2 3 4)))
        (let ((miller (an-integer-between cooper 5)))
          (require (distinct? (list baker cooper fletcher miller)))
          (let ((smith (an-integer-between 1 5)))
            (require (not (= (abs (- smith fletcher)) 1)))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (list (list 'baker baker) (list 'cooper cooper)
                  (list 'fletcher fletcher) (list 'miller miller)
                  (list 'smith smith))))))))
`;

/** Host arithmetic: assignment sets before and after distinctness. */
export const assignmentCounts = (): readonly [number, number] => [5 ** 5, 5 * 4 * 3 * 2 * 1];

const firstWithFailures = (program: string): { answer: string; failuresToFirst: number } => {
  const failures = { count: 0 };
  const evaluator = makeAmbEvaluator({ failures });
  return Effect.runSync(
    Effect.flatMap(setupAmbEnvironment(), (env) =>
      Effect.map(Effect.result(runAmbText(evaluator, program, env, 1)), (outcome) => ({
        answer:
          outcome._tag === "Failure"
            ? `error ${outcome.failure._tag}`
            : outcome.success.answers.map(format).join(","),
        failuresToFirst: failures.count,
      })),
    ),
  );
};

/** Every solution of the pruned procedure: exactly one. */
export const prunedSolutions = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupAmbEnvironment(), (env) =>
    Effect.map(
      runAmbText(
        ambEvaluator,
        [puzzleLibrary, prunedDefinition, "(multiple-dwelling-faster)"].join("\n"),
        env,
      ),
      (run) => run.answers.map(format),
    ),
  );

export function ex_4_40(): string {
  const [before, after] = assignmentCounts();
  const naive = firstWithFailures(
    [puzzleLibrary, naiveDefinition, "(multiple-dwelling)"].join("\n"),
  );
  const pruned = firstWithFailures(
    [puzzleLibrary, prunedDefinition, "(multiple-dwelling-faster)"].join("\n"),
  );
  const all = Effect.runSync(prunedSolutions());
  return (
    `Before the distinctness requirement there are 5^5 = ${before} sets of ` +
    `assignments; after it, 5! = ${after}. It is inefficient to generate ` +
    "all the assignments and leave the eliminations to backtracking, so " +
    "the pruned procedure moves each restriction in front of the choices " +
    "it does not mention: Cooper draws from four floors, Fletcher from " +
    "three, Miller draws above Cooper directly, and the distinctness and " +
    "adjacency requirements run as soon as their people have drawn. Both " +
    `procedures answer ${naive.answer}; the naive order delivers ` +
    `${naive.failuresToFirst} failures to the first answer, the pruned ` +
    `order ${pruned.failuresToFirst}, and the pruned search finds no ` +
    `second solution (${all.length} in total).`
  );
}
