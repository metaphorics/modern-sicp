// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import {
  falseOracle,
  makeDiagonalEnvironment,
  makeFueledEvaluator,
  RUN_FOREVER,
  runFueled,
  TRY,
  trueOracle,
} from "./ex_4_15.js";

const failureOf = (run: Effect.Effect<Value, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const fuelMessage = (error: EvaluationError): string =>
  error._tag === "RuntimeError" ? error.message : "<not a RuntimeError>";

const BOOK = [RUN_FOREVER, TRY];

describe("exercise 4.15: the halting diagonal", () => {
  it.effect("under the true oracle, (try try) runs forever and hits the fuel limit", () =>
    Effect.gen(function* () {
      const env = yield* makeDiagonalEnvironment(trueOracle);
      const fueled = yield* makeFueledEvaluator(500);
      const failure = yield* failureOf(runFueled(fueled, [...BOOK, "(try try)"], env));
      expect(failure._tag).toBe("RuntimeError");
      expect(fuelMessage(failure)).toBe("out of fuel");
      expect(yield* fueled.stepsUsed).toBe(501);
    }),
  );

  it.effect("under the false oracle, (try (lambda (u) u)) answers 'halted", () =>
    Effect.gen(function* () {
      const env = yield* makeDiagonalEnvironment(falseOracle);
      const fueled = yield* makeFueledEvaluator(500);
      const value = yield* runFueled(fueled, [...BOOK, "(try (lambda (u) u))"], env);
      expect(value).toStrictEqual({ _tag: "Symbol", name: "halted" });
      expect(yield* fueled.stepsUsed).toBeLessThan(500);
    }),
  );

  it.effect("under the true oracle the same lambda drives try into run-forever", () =>
    Effect.gen(function* () {
      const env = yield* makeDiagonalEnvironment(trueOracle);
      const fueled = yield* makeFueledEvaluator(500);
      const failure = yield* failureOf(runFueled(fueled, [...BOOK, "(try (lambda (u) u))"], env));
      expect(failure._tag).toBe("RuntimeError");
      expect(fuelMessage(failure)).toBe("out of fuel");
    }),
  );
});
