// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  applyProcedure,
  evaluate,
  setupEnvironment,
} from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, Value } from "../../packages/ch4/src/core.js";
import { nil } from "../../packages/ch4/src/list.js";
import { read } from "../../packages/ch4/src/read.js";
import { evaluateSameFrame, evaluateScanned, frameDepth } from "./ex_4_17.js";

const closureEnvOf = (value: Value): Env => {
  if (value._tag !== "Compound") {
    throw new Error(`expected a compound procedure, got ${value._tag}`);
  }
  return value.env;
};

const allEvaluators: ReadonlyArray<Evaluate> = [evaluate, evaluateScanned, evaluateSameFrame];

describe("exercise 4.17: the extra frame of scanned-out definitions", () => {
  it.effect("runs the same define-using program identically under all three evaluators", () =>
    Effect.gen(function* () {
      for (const evaluateVariant of allEvaluators) {
        const env = yield* setupEnvironment();
        yield* evaluateVariant(read("(define (f) (define a 1) (define b (+ a 1)) (+ a b))"), env);
        expect(yield* evaluateVariant(read("(f)"), env)).toStrictEqual({ _tag: "Number", n: 3 });
      }
    }),
  );

  it.effect("the scanned closure sees one more frame than sequential and same-frame", () =>
    Effect.gen(function* () {
      const depths: number[] = [];
      for (const evaluateVariant of allEvaluators) {
        const env = yield* setupEnvironment();
        yield* evaluateVariant(read("(define (g) (define a 1) (lambda () a))"), env);
        const closure = yield* evaluateVariant(read("(g)"), env);
        depths.push(frameDepth(closureEnvOf(closure)));
      }
      expect(depths).toStrictEqual([2, 3, 2]);
    }),
  );

  it.effect("the same-frame variant keeps the returned closure usable", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* evaluateSameFrame(read("(define (g) (define a 1) (lambda () a))"), env);
      const closure = yield* evaluateSameFrame(read("(g)"), env);
      expect(yield* applyProcedure(closure, nil)).toStrictEqual({ _tag: "Number", n: 1 });
    }),
  );
});
