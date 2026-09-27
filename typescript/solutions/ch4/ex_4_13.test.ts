// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { nil } from "../../packages/ch4/src/list.js";
import { format } from "../../packages/ch4/src/read.js";
import { evalStringWithUnbound } from "./ex_4_13.js";

const unboundName = (error: EvaluationError): string =>
  error._tag === "UnboundVariable" ? error.name : "<not an UnboundVariable>";

const lastOf = (values: ReadonlyArray<Value>): Value => {
  const last = values[values.length - 1];
  return last === undefined ? nil : last;
};

const evalPrograms = (
  sources: ReadonlyArray<string>,
  env: Env,
): Effect.Effect<Value, EvaluationError> =>
  Effect.map(
    Effect.forEach(sources, (source) => evalStringWithUnbound(source, env)),
    lastOf,
  );

const failureOf = (run: Effect.Effect<Value, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

describe("exercise 4.13: make-unbound!", () => {
  it.effect("unbinding an inner binding exposes the outer one again", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const shadowed = yield* evalPrograms(["(define a 1)", "((lambda () (define a 2) a))"], env);
      expect(format(shadowed)).toBe("2");

      const restoredEnv = yield* setupEnvironment();
      const restored = yield* evalPrograms(
        ["(define a 1)", "((lambda () (define a 2) (make-unbound! a) a))"],
        restoredEnv,
      );
      expect(format(restored)).toBe("1");
      expect(yield* evalStringWithUnbound("a", restoredEnv)).toStrictEqual({
        _tag: "Number",
        n: 1,
      });
    }),
  );

  it.effect("set! after an unbind writes the outer frame", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const result = yield* evalPrograms(
        [
          "(define a 1)",
          "(define (retarget) (define a 2) (make-unbound! a) (set! a 50) a)",
          "(retarget)",
        ],
        env,
      );
      expect(format(result)).toBe("50");
      expect(yield* evalStringWithUnbound("a", env)).toStrictEqual({ _tag: "Number", n: 50 });
    }),
  );

  it.effect("unbinding a name bound nowhere is an error", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const missing = yield* failureOf(evalStringWithUnbound("(make-unbound! zz)", env));
      expect(missing._tag).toBe("UnboundVariable");
      expect(unboundName(missing)).toBe("zz");

      const twiceEnv = yield* setupEnvironment();
      yield* evalStringWithUnbound("(define a 1)", twiceEnv);
      yield* evalStringWithUnbound("(make-unbound! a)", twiceEnv);
      const gone = yield* failureOf(evalStringWithUnbound("a", twiceEnv));
      expect(gone._tag).toBe("UnboundVariable");
      const again = yield* failureOf(evalStringWithUnbound("(make-unbound! a)", twiceEnv));
      expect(again._tag).toBe("UnboundVariable");
      expect(unboundName(again)).toBe("a");
    }),
  );
});
