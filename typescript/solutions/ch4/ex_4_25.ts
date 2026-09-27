// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

/**
 * Exercise 4.25: unless breaks under applicative order. The same
 * unless-based factorial runs under two evaluators. The lazy evaluator
 * delays unless's arms: the condition forces, only the chosen arm is ever
 * demanded, and the recursion bottoms out, so (factorial 5) answers 120.
 * The strict 4.1 evaluator keeps the edition's strict-argument rule: an
 * ordinary procedure's operand evaluates before the call, so the armed
 * call (unless (= 1 1) (/ 1 0) 42) dies in the division, and the strict
 * factorial never reaches its base case. The descent is observed, not
 * hung on: the strict session runs with a step budget installed on the
 * subtraction primitive, which every level of the descent calls, so the
 * predicted non-termination answers as a typed budget error.
 */
import { Effect } from "effect";

import {
  evaluate,
  lookupVariableValue,
  setupEnvironment,
  setVariableValue,
  symbol,
} from "../../packages/ch4/src/01-metacircular.js";
import { lazyDriverLoop } from "../../packages/ch4/src/02-lazy.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import { type EvaluationError, RuntimeError } from "../../packages/ch4/src/errors.js";
import { read } from "../../packages/ch4/src/read.js";

/** The book's unless and the factorial defined with it. */
export const unlessDefinition =
  "(define (unless condition usual-value exceptional-value) (if condition exceptional-value usual-value))";

export const factorialDefinition =
  "(define (factorial n) (unless (= n 1) (* n (factorial (- n 1))) 1))";

/** The session's three inputs, in order. */
export const lazySession = [unlessDefinition, factorialDefinition, "(factorial 5)"];

/** The armed call: the exceptional arm would blow up if it ever ran. */
export const armedCall = "(unless (= 1 1) (/ 1 0) 42)";

/** A mutable step budget shared by one strict session. */
export interface Budget {
  steps: number;
}

export const budgetError = "the step budget ran out: factorial is still descending";

/** Installs the budget on the subtraction primitive: the strict descent
 * calls (- n 1) at every level, so the budget counts levels, and an
 * exhausted budget answers the typed error the host stands in with for
 * "runs forever". */
export const installBudget = (env: Env, budget: Budget): Effect.Effect<void, EvaluationError> =>
  Effect.flatMap(lookupVariableValue(symbol("-"), env), (subtraction) => {
    if (subtraction._tag !== "Primitive") {
      return Effect.die(new Error("the - binding is not the primitive"));
    }
    const inner = subtraction.fn;
    const counted: Value = {
      _tag: "Primitive",
      name: "-",
      fn: (args) => {
        if (budget.steps <= 0) {
          return Effect.fail(new RuntimeError({ message: budgetError, detail: "" }));
        }
        budget.steps -= 1;
        return inner(args);
      },
    };
    return setVariableValue(symbol("-"), counted, env);
  });

const runIn = (
  env: Env,
  sources: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.forEach(sources, (source) =>
    Effect.map(evaluate(read(source), env), (value) => `${value._tag}`),
  );

const strictRun = (
  sources: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => runIn(env, sources));

/** A strict session under the budget: the environment carries the
 * counted subtraction, so a program descending through (- n 1) answers
 * the budget error instead of running forever. */
const budgetedStrictRun = (
  sources: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(installBudget(env, { steps: 200 }), () => runIn(env, sources)),
  );

const strictFailure = (
  sources: ReadonlyArray<string>,
): Effect.Effect<RuntimeError, EvaluationError> =>
  Effect.flatMap(Effect.result(budgetedStrictRun(sources)), (outcome) => {
    if (outcome._tag === "Failure" && outcome.failure._tag === "RuntimeError") {
      return Effect.succeed(outcome.failure);
    }
    return Effect.die(new Error("expected a strict RuntimeError"));
  });

/** The lazy session: the definitions plus (factorial 5), driven by the
 * section's driver loop. */
export const lazyAnswers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) => lazyDriverLoop(env, lazySession));

/** The armed call under the strict evaluator, which answers the division
 * failure instead of a value. */
export const armedStrictError = (): Effect.Effect<RuntimeError, EvaluationError> =>
  Effect.flatMap(Effect.result(strictRun([unlessDefinition, armedCall])), (outcome) => {
    if (outcome._tag === "Failure" && outcome.failure._tag === "RuntimeError") {
      return Effect.succeed(outcome.failure);
    }
    return Effect.die(new Error("expected a strict RuntimeError"));
  });

/** The strict factorial under the budget, which answers the descent
 * error the exercise's "runs forever" predicts. */
export const descendingStrictError = (): Effect.Effect<RuntimeError, EvaluationError> =>
  strictFailure(lazySession);

/** The observed answers, one string per behavior. */
export const answers = (): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(lazyAnswers(), (lazy) =>
    Effect.flatMap(armedStrictError(), (armed) =>
      Effect.map(descendingStrictError(), (descending) => [
        lazy[lazy.length - 1] ?? "",
        armed.message,
        descending.message,
      ]),
    ),
  );

export function ex_4_25(): string {
  const observed = Effect.runSync(answers());
  return (
    "Under the lazy evaluator the definitions work unchanged: unless's arms " +
    "delay as thunks, the condition (= n 1) forces, and only the chosen arm " +
    "is ever demanded, so the recursion bottoms out and (factorial 5) " +
    `answers ${observed[0]}. Under the applicative-order evaluator the ` +
    "recursive operand (* n (factorial (- n 1))) evaluates before unless is " +
    `entered: the armed call (unless (= 1 1) (/ 1 0) 42) fails with ` +
    `"${observed[1]}", and the strict factorial never reaches its base ` +
    "case, so it only stops at the step budget installed on its descending " +
    `subtraction: "${observed[2]}". The definitions therefore do not work ` +
    "unchanged in a language without normal order."
  );
}
