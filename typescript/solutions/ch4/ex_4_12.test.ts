// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  defineVariableValue,
  extendEnvironment,
  lookupVariableValue,
  setupEnvironment,
  symbol,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { nil } from "../../packages/ch4/src/list.js";
import {
  defineVariableValueTraversing,
  forEachBinding,
  lookupVariableValueTraversing,
  setVariableValueTraversing,
} from "./ex_4_12.js";

const unboundName = (error: EvaluationError): string =>
  error._tag === "UnboundVariable" ? error.name : "<not an UnboundVariable>";

const number = (n: number): Value => ({ _tag: "Number", n });

const emptyFrame = (base: Env): Effect.Effect<Env, EvaluationError> =>
  extendEnvironment(nil, nil, base);

const makeChain = (): Effect.Effect<readonly [Env, Env, Env], EvaluationError> =>
  Effect.gen(function* () {
    const outer = yield* setupEnvironment();
    yield* defineVariableValue(symbol("a"), number(1), outer);
    const middle = yield* emptyFrame(outer);
    yield* defineVariableValue(symbol("b"), number(2), middle);
    const inner = yield* emptyFrame(middle);
    yield* defineVariableValue(symbol("c"), number(3), inner);
    return [outer, middle, inner] as const;
  });

describe("exercise 4.12: environment traversals", () => {
  it.effect("lookup traverses a three-frame chain outward", () =>
    Effect.gen(function* () {
      const [, , inner] = yield* makeChain();
      expect(yield* lookupVariableValueTraversing(symbol("c"), inner)).toStrictEqual(number(3));
      expect(yield* lookupVariableValueTraversing(symbol("b"), inner)).toStrictEqual(number(2));
      expect(yield* lookupVariableValueTraversing(symbol("a"), inner)).toStrictEqual(number(1));
      const missing = yield* Effect.flip(lookupVariableValueTraversing(symbol("zz"), inner));
      expect(missing._tag).toBe("UnboundVariable");
      expect(unboundName(missing)).toBe("zz");
    }),
  );

  it.effect("set! lands in the frame that defines the name, seen everywhere", () =>
    Effect.gen(function* () {
      const [, middle, inner] = yield* makeChain();
      yield* setVariableValueTraversing(symbol("b"), number(20), inner);
      expect(yield* lookupVariableValue(symbol("b"), inner)).toStrictEqual(number(20));
      expect(yield* lookupVariableValue(symbol("b"), middle)).toStrictEqual(number(20));
      expect(yield* lookupVariableValueTraversing(symbol("b"), inner)).toStrictEqual(number(20));
      const missing = yield* Effect.flip(
        setVariableValueTraversing(symbol("zz"), number(9), inner),
      );
      expect(missing._tag).toBe("UnboundVariable");
    }),
  );

  it.effect("define writes the first frame and needs no walk", () =>
    Effect.gen(function* () {
      const [outer, middle, inner] = yield* makeChain();
      yield* defineVariableValueTraversing(symbol("d"), number(4), inner);
      expect(yield* lookupVariableValueTraversing(symbol("d"), inner)).toStrictEqual(number(4));
      const fromMiddle = yield* Effect.flip(lookupVariableValueTraversing(symbol("d"), middle));
      expect(fromMiddle._tag).toBe("UnboundVariable");
      expect(unboundName(fromMiddle)).toBe("d");
      expect(yield* lookupVariableValueTraversing(symbol("a"), outer)).toStrictEqual(number(1));
    }),
  );

  it.effect("forEachBinding visits exactly the frame's own bindings", () =>
    Effect.gen(function* () {
      const [, , inner] = yield* makeChain();
      const seen: string[] = [];
      yield* forEachBinding(inner, (name) =>
        Effect.map(
          Effect.sync(() => seen.push(name)),
          () => undefined,
        ),
      );
      expect([...seen].sort()).toStrictEqual(["c"]);
    }),
  );
});
