// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { evalString } from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";
import { evaMapDefinition, makeEvaEnvironment, makeLouisEnvironment } from "./ex_4_14.js";

const runtimeMessage = (error: EvaluationError): string =>
  error._tag === "RuntimeError" ? error.message : "<not a RuntimeError>";

const failureOf = (run: Effect.Effect<Value, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

describe("exercise 4.14: map as a primitive", () => {
  it.effect("Louis's host map handles a primitive procedure", () =>
    Effect.gen(function* () {
      const env = yield* makeLouisEnvironment();
      const value = yield* evalString("(map car '((1 2) (3 4)))", env);
      expect(format(value)).toBe("(1 3)");
    }),
  );

  it.effect("Louis's host map dies on an evaluator closure", () =>
    Effect.gen(function* () {
      const env = yield* makeLouisEnvironment();
      const failure = yield* failureOf(evalString("(map (lambda (p) p) '((9)))", env));
      expect(failure._tag).toBe("RuntimeError");
      expect(runtimeMessage(failure)).toBe("map: the host could not call this procedure");
    }),
  );

  it.effect("Eva's object-language map works for both calls", () =>
    Effect.gen(function* () {
      const env = yield* makeEvaEnvironment();
      const pairs = yield* evalString("(map car '((1 2) (3 4)))", env);
      expect(format(pairs)).toBe("(1 3)");
      const squares = yield* evalString("(map (lambda (n) (* n n)) '(1 2 3))", env);
      expect(format(squares)).toBe("(1 4 9)");
      const identity = yield* evalString("(map (lambda (p) p) '((9)))", env);
      expect(format(identity)).toBe("((9))");
    }),
  );

  it.effect("Eva's map is an ordinary compound definition", () =>
    Effect.sync(() => {
      expect(evaMapDefinition.startsWith("(define (map p x)")).toBe(true);
    }),
  );
});
