// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect, test } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalWithNamedLet, namedLetToCombination } from "./ex_4_08.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalWithNamedLet(read(source), env);

const fib = (n: number): string =>
  `(let fib ((k ${n})) (if (< k 2) k (+ (fib (- k 1)) (fib (- k 2)))))`;

describe("exercise 4.8: named let", () => {
  it.effect("Fibonacci 10 by named let is 55", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run(fib(10), env)).toStrictEqual({ _tag: "Number", n: 55 });
    }),
  );

  it.effect("a factorial loop by named let is 120", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(
        yield* run("(let fact ((n 5) (acc 1)) (if (= n 0) acc (fact (- n 1) (* acc n))))", env),
      ).toStrictEqual({ _tag: "Number", n: 120 });
    }),
  );

  test("the transformation follows the book's set! hint", () => {
    const namedForm = read("(let loop ((n 0)) n)");
    if (namedForm._tag !== "Cons") {
      throw new Error("expected a named let form");
    }
    const expected = format(read("((lambda (loop) (set! loop (lambda (n) n)) (loop 0)) 'ok)"));
    expect(format(namedLetToCombination(namedForm))).toBe(expected);
  });

  it.effect("plain let still evaluates in the same evaluator", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(yield* run("(let ((x 3) (y 4)) (+ x y))", env)).toStrictEqual({
        _tag: "Number",
        n: 7,
      });
    }),
  );

  it.effect("a named let inside a defined body recurs through its name", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run(`(define (fib10) ${fib(10)})`, env);
      expect(yield* run("(fib10)", env)).toStrictEqual({ _tag: "Number", n: 55 });
    }),
  );
});
