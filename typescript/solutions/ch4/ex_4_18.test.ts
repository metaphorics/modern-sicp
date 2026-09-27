// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import type { Env, Evaluate, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { read } from "../../packages/ch4/src/read.js";
import { defineUnder, evaluateAlternative, evaluateText, ex_4_18 } from "./ex_4_18.js";

const siblingReader = "(define (g) (define a 1) (define b (+ a 1)) b)";

const deferredReader = "(define (h) (define (get) b) (define b 5) (get))";

const lastValue = (
  evaluate: Evaluate,
  env: Env,
  sources: ReadonlyArray<string>,
): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.forEach(sources, (source) => evaluate(read(source), env)),
    (values) => {
      const last = values[values.length - 1];
      return last !== undefined
        ? Effect.succeed(last)
        : Effect.die(new Error("no forms evaluated"));
    },
  );

describe("exercise 4.18: the alternative scan-out strategy", () => {
  it.effect("the text's scan reads an already assigned sibling; the alternative fails", () =>
    Effect.gen(function* () {
      const textEnv = yield* defineUnder(evaluateText, siblingReader);
      expect(yield* lastValue(evaluateText, textEnv, ["(g)"])).toStrictEqual({
        _tag: "Number",
        n: 2,
      });

      const alternativeEnv = yield* defineUnder(evaluateAlternative, siblingReader);
      const outcome = yield* Effect.result(lastValue(evaluateAlternative, alternativeEnv, ["(g)"]));
      expect(outcome._tag).toBe("Failure");
      if (outcome._tag === "Failure") {
        expect(outcome.failure._tag).toBe("RuntimeError");
      }
    }),
  );

  it.effect("a sibling read deferred through a closure works under both scans", () =>
    Effect.gen(function* () {
      const textEnv = yield* defineUnder(evaluateText, deferredReader);
      expect(yield* lastValue(evaluateText, textEnv, ["(h)"])).toStrictEqual({
        _tag: "Number",
        n: 5,
      });

      const alternativeEnv = yield* defineUnder(evaluateAlternative, deferredReader);
      expect(yield* lastValue(evaluateAlternative, alternativeEnv, ["(h)"])).toStrictEqual({
        _tag: "Number",
        n: 5,
      });
    }),
  );

  it("states the solve verdict", () => {
    expect(ex_4_18()).toContain("fails under the alternative scan");
  });
});
