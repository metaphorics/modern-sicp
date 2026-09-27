// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.32: streams versus lazy lists. The 4.2.3 procedural pairs
 * delay both slots, so a pair constructs without computing and a dormant
 * armed slot forces only when it is the one demanded, which is lazier
 * than the chapter 3 streams whose cons-stream evaluates the car at
 * construction. The demonstrations: (car (cons 7 (/ 1 0))) answers 7
 * under the lazy evaluator and the armed (/ 1 0) answers only when cdr is
 * demanded; the same procedural cons under the strict base evaluator
 * dies in the arm at construction, which is the eager discipline the
 * chapter 3 car has; and the self-referential ones defines in one step
 * because nothing is forced.
 */
import { Effect } from "effect";

import { evaluate, setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import { lazyDriverWith, lazyEvaluator } from "../../packages/ch4/src/02-lazy.js";
import type { EvaluationError, RuntimeError } from "../../packages/ch4/src/errors.js";
import { read } from "../../packages/ch4/src/read.js";

export const proceduralPairs = [
  "(define (cons x y) (lambda (m) (m x y)))",
  "(define (car z) (z (lambda (p q) p)))",
  "(define (cdr z) (z (lambda (p q) q)))",
];

export const armedPair = "(car (cons 7 (/ 1 0)))";
export const armedTail = "(cdr (cons 7 (/ 1 0)))";
export const eagerConstruction = "(car (cons (/ 1 0) 7))";
export const selfReference = "(define ones (cons 1 ones))";

/** The lazy run: the armed slot stays dormant until demanded. */
export const lazyCarOfArmed = (): Effect.Effect<string, EvaluationError> =>
  Effect.map(
    lazyDriverWith(lazyEvaluator, [...proceduralPairs, armedPair]),
    (transcript) => transcript[transcript.length - 1] ?? "",
  );

const lastFailureMessage = (
  effect: Effect.Effect<ReadonlyArray<string>, EvaluationError>,
): Effect.Effect<string, EvaluationError> =>
  Effect.flatMap(Effect.result(effect), (outcome) => {
    if (outcome._tag === "Failure" && outcome.failure._tag === "RuntimeError") {
      const error: RuntimeError = outcome.failure;
      return Effect.succeed(error.message);
    }
    return Effect.die(new Error("expected a RuntimeError"));
  });

/** The lazy run of the armed tail: the division answers on demand. */
export const lazyCdrOfArmed = (): Effect.Effect<string, EvaluationError> =>
  lastFailureMessage(lazyDriverWith(lazyEvaluator, [...proceduralPairs, armedTail]));

/** The same procedural cons under the strict base evaluator: the armed
 * arm evaluates at construction, before car is ever applied. */
export const strictEagerConstruction = (): Effect.Effect<string, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => {
    const run = Effect.forEach([...proceduralPairs, eagerConstruction], (source) =>
      Effect.map(evaluate(read(source), env), () => "done"),
    );
    return lastFailureMessage(run);
  });

/** The self-referential definition under the lazy evaluator: ok, with
 * nothing forced. */
export const lazySelfReference = (): Effect.Effect<string, EvaluationError> =>
  Effect.map(
    lazyDriverWith(lazyEvaluator, [...proceduralPairs, selfReference]),
    (transcript) => transcript[transcript.length - 1] ?? "",
  );

/** The four observed answers. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(lazyCarOfArmed(), (car) =>
    Effect.flatMap(lazyCdrOfArmed(), (cdr) =>
      Effect.flatMap(strictEagerConstruction(), (eager) =>
        Effect.map(lazySelfReference(), (ones) => [car, cdr, eager, ones]),
      ),
    ),
  );

export function ex_4_32(): string {
  const observed = Effect.runSync(answers());
  return (
    "The extra laziness is in the car slot. Under the lazy evaluator the " +
    `procedural pair constructs without computing: (car (cons 7 (/ 1 0))) ` +
    `answers ${observed[0]} because the armed arm is never demanded, while ` +
    `(cdr (cons 7 (/ 1 0))) answers the division on demand (${observed[1]}). ` +
    "The chapter 3 stream discipline delays only the cdr: the same " +
    "procedural cons under the strict base evaluator dies in the arm at " +
    `construction (${observed[2]}), which is exactly how cons-stream would ` +
    "treat the car. The lazier pair also builds self-referential data in " +
    `one step: (define ones (cons 1 ones)) answers ${observed[3]} with ` +
    "nothing forced, and selective forcing walks any path of the structure " +
    "without computing the rest, which is the tool the lazy tree of the " +
    "chapter's footnote generalizes."
  );
}
