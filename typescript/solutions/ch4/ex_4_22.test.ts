// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { evaluate, setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { read } from "../../packages/ch4/src/read.js";
import { analyzeLet, evalAnalyzedWithLet, ex_4_22, runAnalyzed } from "./ex_4_22.js";

const directValue = (sources: ReadonlyArray<string>): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(setupEnvironment(), (env) =>
    Effect.flatMap(
      Effect.forEach(sources, (source) => evaluate(read(source), env)),
      (values) => {
        const last = values[values.length - 1];
        return last !== undefined
          ? Effect.succeed(last)
          : Effect.die(new Error("no forms evaluated"));
      },
    ),
  );

describe("exercise 4.22: let in the analyzed evaluator", () => {
  it.effect("a let expression analyzes and evaluates through evalAnalyzed", () =>
    Effect.gen(function* () {
      expect(
        yield* evalAnalyzedWithLet(read("(let ((x 3)) (+ x 4))"), yield* setupEnvironment()),
      ).toStrictEqual({ _tag: "Number", n: 7 });
    }),
  );

  it.effect("a let nested in a lambda body analyzes once per closure", () =>
    Effect.gen(function* () {
      expect(
        yield* runAnalyzed(["(define (f y) (let ((x (* y y))) (+ x y)))", "(f 4)"]),
      ).toStrictEqual({ _tag: "Number", n: 20 });
    }),
  );

  it.effect("direct evaluation of the derived form agrees with analyzed let", () =>
    Effect.gen(function* () {
      const letSource = "(let ((x 3)) (let ((y (+ x 1))) (* x y)))";
      const derivedSource = "((lambda (x) ((lambda (y) (* x y)) (+ x 1))) 3)";
      expect(yield* directValue([derivedSource])).toStrictEqual({ _tag: "Number", n: 12 });
      expect(yield* evalAnalyzedWithLet(read(letSource), yield* setupEnvironment())).toStrictEqual({
        _tag: "Number",
        n: 12,
      });
    }),
  );

  it.effect("analyzeLet runs the combination's execution procedure", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const form = read("(let ((x 3)) (+ x 4))");
      if (form._tag !== "Cons") {
        return yield* Effect.die(new Error("expected a cons form"));
      }
      expect(yield* analyzeLet(form)(env)).toStrictEqual({
        _tag: "Number",
        n: 7,
      });
    }),
  );

  it("describes the derivation", () => {
    expect(ex_4_22()).toContain("derived expression");
  });
});
