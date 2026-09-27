// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalWithAndOr } from "./ex_4_04.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalWithAndOr(read(source), env);

describe("exercise 4.4: and and or as special forms", () => {
  it.effect("empty and is true and empty or is false", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(and)", env))).toBe("#t");
      expect(format(yield* run("(or)", env))).toBe("#f");
    }),
  );

  it.effect("and returns the last value, or false on the first false conjunct", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(and 1 2 3)", env))).toBe("3");
      expect(format(yield* run("(and 1 false 3)", env))).toBe("#f");
    }),
  );

  it.effect("or returns the first non-false value, or false when all are false", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(or false false 4)", env))).toBe("4");
      expect(format(yield* run("(or false false)", env))).toBe("#f");
    }),
  );

  it.effect("and stops before a false conjunct's side effects", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define hits 0)", env);
      expect(format(yield* run("(and false (set! hits 99))", env))).toBe("#f");
      expect(format(yield* run("hits", env))).toBe("0");
    }),
  );

  it.effect("or stops before a true disjunct's side effects", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define hits 0)", env);
      expect(format(yield* run("(or 7 (set! hits 99))", env))).toBe("7");
      expect(format(yield* run("hits", env))).toBe("0");
    }),
  );

  it.effect("and evaluates the last conjunct when it is reached", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define hits 0)", env);
      expect(format(yield* run("(and (set! hits 99) hits)", env))).toBe("99");
    }),
  );

  it.effect("and and or work inside procedure bodies", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define (both? a b) (and (< 0 a) (< 0 b)))", env);
      yield* run("(define (fallback) (or false 'fallback))", env);
      expect(format(yield* run("(both? 2 3)", env))).toBe("#t");
      expect(format(yield* run("(both? 2 -3)", env))).toBe("#f");
      expect(format(yield* run("(fallback)", env))).toBe("fallback");
    }),
  );
});
