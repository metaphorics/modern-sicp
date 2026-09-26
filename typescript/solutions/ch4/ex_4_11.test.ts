// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { symbol } from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format } from "../../packages/ch4/src/read.js";
import {
  defineVariableValueAssoc,
  frameVariables,
  lookupVariableValueAssoc,
  makeAssocEnv,
  makeGlobalAssocEnv,
  setVariableValueAssoc,
} from "./ex_4_11.js";

const unboundName = (error: EvaluationError): string =>
  error._tag === "UnboundVariable" ? error.name : "<not an UnboundVariable>";

const failureOf = <A>(run: Effect.Effect<A, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const number = (n: number): Value => ({ _tag: "Number", n });

describe("exercise 4.11: association-list frames", () => {
  it.effect("lookup walks the entries and then the chain", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalAssocEnv();
      yield* defineVariableValueAssoc(symbol("a"), number(1), global);
      const inner = yield* makeAssocEnv(global);
      yield* defineVariableValueAssoc(symbol("b"), number(2), inner);

      expect(yield* lookupVariableValueAssoc(symbol("b"), inner)).toStrictEqual(number(2));
      expect(yield* lookupVariableValueAssoc(symbol("a"), inner)).toStrictEqual(number(1));
      expect(yield* lookupVariableValueAssoc(symbol("a"), global)).toStrictEqual(number(1));
      const missing = yield* failureOf(lookupVariableValueAssoc(symbol("zz"), inner));
      expect(missing._tag).toBe("UnboundVariable");
    }),
  );

  it.effect("set! writes the shared frame the chain points at", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalAssocEnv();
      yield* defineVariableValueAssoc(symbol("a"), number(1), global);
      const inner = yield* makeAssocEnv(global);

      yield* setVariableValueAssoc(symbol("a"), number(10), inner);
      expect(yield* lookupVariableValueAssoc(symbol("a"), global)).toStrictEqual(number(10));
      expect(yield* lookupVariableValueAssoc(symbol("a"), inner)).toStrictEqual(number(10));
    }),
  );

  it.effect("define conses onto the current frame, shadowing outer names", () =>
    Effect.gen(function* () {
      const global = yield* makeGlobalAssocEnv();
      yield* defineVariableValueAssoc(symbol("a"), number(1), global);
      const inner = yield* makeAssocEnv(global);
      yield* defineVariableValueAssoc(symbol("b"), number(2), inner);
      yield* defineVariableValueAssoc(symbol("c"), number(3), inner);

      expect(yield* lookupVariableValueAssoc(symbol("c"), inner)).toStrictEqual(number(3));
      const missing = yield* failureOf(lookupVariableValueAssoc(symbol("c"), global));
      expect(missing._tag).toBe("UnboundVariable");

      expect(format(yield* frameVariables(inner.frame))).toBe("(c b)");
      expect(format(yield* frameVariables(global.frame))).toBe("(a)");

      const unbound = yield* failureOf(setVariableValueAssoc(symbol("zz"), number(9), inner));
      expect(unbound._tag).toBe("UnboundVariable");
      expect(unboundName(unbound)).toBe("zz");
    }),
  );
});
