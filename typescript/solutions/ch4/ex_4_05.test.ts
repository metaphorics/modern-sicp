// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Env, Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { format, read } from "../../packages/ch4/src/read.js";
import { evalWithArrowCond } from "./ex_4_05.js";

const run = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalWithArrowCond(read(source), env);

const lookupFixture = (): ReadonlyArray<string> => [
  "(define (assoc key records)\n    (cond ((null? records) false)\n        ((eq? key (car (car records))) (car records))\n        (else (assoc key (cdr records)))))",
  "(define (lookup k)\n    (cond ((assoc k '((a 1) (b 2))) => (lambda (pair) (car (cdr pair))))\n        (else 'missing)))",
];

describe("exercise 4.5: cond arrow clauses", () => {
  it.effect("an arrow clause calls the recipient on the test value", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      for (const source of lookupFixture()) {
        yield* run(source, env);
      }
      expect(format(yield* run("(lookup 'b)", env))).toBe("2");
      expect(format(yield* run("(lookup 'a)", env))).toBe("1");
    }),
  );

  it.effect("a failed arrow test falls through to the else clause", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      for (const source of lookupFixture()) {
        yield* run(source, env);
      }
      expect(format(yield* run("(lookup 'c)", env))).toBe("missing");
    }),
  );

  it.effect("the arrow test is evaluated once and bound", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* run("(define count 0)", env);
      expect(
        format(
          yield* run(
            "(cond ((begin (set! count (+ count 1)) true) => (lambda (v) v)) (else 'no))",
            env,
          ),
        ),
      ).toBe("#t");
      expect(format(yield* run("count", env))).toBe("1");
    }),
  );
});
