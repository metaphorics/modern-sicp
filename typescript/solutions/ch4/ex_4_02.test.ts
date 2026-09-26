// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { evalString, setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Evaluate, Value } from "../../packages/ch4/src/core.js";
import {
  type EvaluationError,
  UnboundVariable,
  UnknownSyntax,
} from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalApplicationsFirst, evalCallTagged } from "./ex_4_02.js";

const runWith = (ev: Evaluate, source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  ev(read(source), env);

describe("exercise 4.2", () => {
  it.effect("the ordinary evaluator defines x and answers (+ x 1)", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* evalString("(define x 3)", env))).toBe("ok");
      expect(format(yield* evalString("(+ x 1)", env))).toBe("4");
    }),
  );

  it.effect("applications-first dispatch applies the unbound symbol define", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const failure = yield* Effect.flip(runWith(evalApplicationsFirst, "(define x 3)", env));
      if (!(failure instanceof UnboundVariable)) {
        throw new Error(`expected UnboundVariable, got ${failure._tag}`);
      }
      expect(failure.name).toBe("define");
    }),
  );

  it.effect("call-tagged dispatch evaluates (call + 1 2) to 3", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(format(yield* runWith(evalCallTagged, "(call + 1 2)", env))).toBe("3");
    }),
  );

  it.effect("call-tagged dispatch rejects the old syntax with UnknownSyntax", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      const failure = yield* Effect.flip(runWith(evalCallTagged, "(+ 1 2)", env));
      if (!(failure instanceof UnknownSyntax)) {
        throw new Error(`expected UnknownSyntax, got ${failure._tag}`);
      }
      expect(failure.expr).toBe("(+ 1 2)");
    }),
  );

  it.effect("call-tagged dispatch runs a lambda whose body uses call", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      expect(
        format(yield* runWith(evalCallTagged, "(call (lambda (x) (call * x x)) 7)", env)),
      ).toBe("49");
    }),
  );
});
