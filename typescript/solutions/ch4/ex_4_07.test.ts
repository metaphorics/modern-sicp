// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect, test } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalWithLetStar, letStarToNestedLets } from "./ex_4_07.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalWithLetStar(read(source), env);

describe("exercise 4.7: let* as nested lets", () => {
  it.effect("the book's example returns 39", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let* ((x 3) (y (+ x 2)) (z (+ x y 5))) (* x z))", env)).toStrictEqual({
        _tag: "Number",
        n: 39,
      });
    }),
  );

  test("let*->nested-lets rewrites into one-binding lets", () => {
    const starForm = read("(let* ((x 3) (y (+ x 2))) (* x y))");
    if (starForm._tag !== "Cons") {
      throw new Error("expected a let* form");
    }
    const expected = format(read("(let ((x 3)) (let ((y (+ x 2))) (* x y)))"));
    expect(format(letStarToNestedLets(starForm))).toBe(expected);
  });

  it.effect("each initializer sees the previous bindings", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let* ((x 3) (y (+ x 1))) (+ x y))", env)).toStrictEqual({
        _tag: "Number",
        n: 7,
      });
    }),
  );

  it.effect("empty bindings leave only the body", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let* () 42)", env)).toStrictEqual({ _tag: "Number", n: 42 });
    }),
  );

  it.effect("a let* inside a defined body evaluates through the rewrite", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define (g) (let* ((a 2) (b (* a 3))) (+ a b)))", env);
      expect(yield* run("(g)", env)).toStrictEqual({ _tag: "Number", n: 8 });
    }),
  );
});
