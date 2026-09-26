// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalDataDirected } from "./ex_4_03.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalDataDirected(read(source), env);

describe("exercise 4.3: data-directed dispatch", () => {
  it.effect("define and application share the evaluator with the table", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(define (square x) (* x x))", env))).toBe("ok");
      expect(format(yield* run("(* (square 6) (square 7))", env))).toBe("1764");
    }),
  );

  it.effect("quote runs its installed operation", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(quote (a b c))", env))).toBe("(a b c)");
    }),
  );

  it.effect("cond with an else clause runs its installed operation", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(cond ((= 1 2) 'yes) (else 'no))", env))).toBe("no");
    }),
  );

  it.effect("set! and define write through the installed operations", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define n 1)", env);
      expect(format(yield* run("(set! n 5)", env))).toBe("ok");
      expect(format(yield* run("n", env))).toBe("5");
    }),
  );

  it.effect("lambda builds a procedure and begin runs its installed operation", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("((lambda (x) (* x 2)) 21)", env))).toBe("42");
      expect(format(yield* run("(begin 1 2 3)", env))).toBe("3");
    }),
  );

  it.effect("a pair whose car is not installed is an application", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* run("(list 1 2 3)", env))).toBe("(1 2 3)");
    }),
  );

  it.effect("a cond nested in a defined body dispatches through the table", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define (classify n) (cond ((< n 0) 'neg) (else 'pos)))", env);
      expect(format(yield* run("(classify -5)", env))).toBe("neg");
    }),
  );
});
