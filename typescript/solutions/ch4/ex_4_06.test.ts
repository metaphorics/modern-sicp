// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect, test } from "vitest";

import { equalValue, setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { read } from "../../packages/ch4/src/read.js";
import { evalWithLet, letToCombination } from "./ex_4_06.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalWithLet(read(source), env);

describe("exercise 4.6: let as a derived expression", () => {
  it.effect("a let evaluates as its combination", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let ((x 3) (y 4)) (+ x y))", env)).toStrictEqual({
        _tag: "Number",
        n: 7,
      });
    }),
  );

  test("let->combination builds the manual lambda form exactly", () => {
    const letForm = read("(let ((x 3) (y 4)) (+ x y))");
    if (letForm._tag !== "Cons") {
      throw new Error("expected a let form");
    }
    const manual = read("((lambda (x y) (+ x y)) 3 4)");
    expect(equalValue(letToCombination(letForm), manual)).toBe(true);
  });

  it.effect("a let inside a defined body evaluates through the derived form", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define (f) (let ((x 2)) (* x 21)))", env);
      expect(yield* run("(f)", env)).toStrictEqual({ _tag: "Number", n: 42 });
    }),
  );

  it.effect("the let body runs as a sequence", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let ((x 1)) (set! x 10) x)", env)).toStrictEqual({
        _tag: "Number",
        n: 10,
      });
    }),
  );
});
