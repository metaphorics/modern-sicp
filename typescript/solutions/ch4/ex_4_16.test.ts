// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { setupEnvironment } from "../../packages/ch4/src/01-metacircular.js";
import type { Value } from "../../packages/ch4/src/core.js";
import type { EvaluationError } from "../../packages/ch4/src/errors.js";
import { cons, type List, nil } from "../../packages/ch4/src/list.js";
import { format, readAll } from "../../packages/ch4/src/read.js";
import {
  evalStringScanned,
  ex_4_16,
  fDefinition,
  scanOutDefines,
  unassignedMarker,
} from "./ex_4_16.js";

const fromValues = (items: ReadonlyArray<Value>): List<Value> =>
  items.reduceRight<List<Value>>((tail, head) => cons(head, tail), nil);

const failureOf = (run: Effect.Effect<Value, EvaluationError>): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(run), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const runtimeDetail = (error: EvaluationError): string =>
  error._tag === "RuntimeError" ? error.detail : "<not a RuntimeError>";

describe("exercise 4.16: scan out internal defines", () => {
  it.effect("scan-out-defines produces the let-and-set! shape", () =>
    Effect.sync(() => {
      const body = fromValues(readAll("(define a 1) (define b 2) (+ a b)"));
      const scanned = scanOutDefines(body);
      expect(format(scanned)).toBe(
        "((let ((a (quote *unassigned*)) (b (quote *unassigned*))) (set! a 1) (set! b 2) (+ a b)))",
      );
    }),
  );

  it.effect("a body with no defines is unchanged", () =>
    Effect.sync(() => {
      const body = fromValues(readAll("(set! a 1) (+ a 1)"));
      expect(scanOutDefines(body)).toStrictEqual(body);
    }),
  );

  it.effect("the scanned marker is the *unassigned* symbol", () =>
    Effect.sync(() => {
      expect(unassignedMarker).toStrictEqual({ _tag: "Symbol", name: "*unassigned*" });
    }),
  );

  it.effect("the book's mutual even?/odd? f works under the scan", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* evalStringScanned(fDefinition, env);
      expect(yield* evalStringScanned("(f 7)", env)).toStrictEqual({ _tag: "Boolean", b: false });
      expect(yield* evalStringScanned("(f 8)", env)).toStrictEqual({ _tag: "Boolean", b: true });
    }),
  );

  it.effect("reading a name before its set! fails", () =>
    Effect.gen(function* () {
      const env = yield* setupEnvironment();
      yield* evalStringScanned("(define (g) (define a b) (define b 1) a)", env);
      const failure = yield* failureOf(evalStringScanned("(g)", env));
      expect(failure._tag).toBe("RuntimeError");
      expect(runtimeDetail(failure)).toBe("b");
    }),
  );

  it.effect("the answer names make-procedure as the installation point", () =>
    Effect.sync(() => {
      expect(ex_4_16()).toContain("make-procedure");
    }),
  );
});
