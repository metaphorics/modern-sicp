// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.2

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  LAZY_PRINT_BUDGET,
  type LazyEvaluator,
  lazyDriverLoop,
  makeLazyEvaluator,
  renderLazyValue,
  setupLazyEnvironment,
} from "./02-lazy.js";
import type { EvaluationError } from "./errors.js";
import { format, read } from "./read.js";

const driverSession = (
  inputs: ReadonlyArray<string>,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupLazyEnvironment(), (env) => lazyDriverLoop(env, inputs));

const evaluateSession = (
  inputs: ReadonlyArray<string>,
  evaluator: LazyEvaluator,
): Effect.Effect<ReadonlyArray<string>, EvaluationError> =>
  Effect.flatMap(setupLazyEnvironment(), (env) =>
    Effect.forEach(inputs, (input) =>
      Effect.flatMap(evaluator.evaluate(read(input), env), (value) =>
        Effect.map(evaluator.force(value), (forced) => format(forced)),
      ),
    ),
  );

const failureOf = (
  effect: Effect.Effect<unknown, EvaluationError>,
): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(effect), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const valuesOf = (transcript: ReadonlyArray<string>): ReadonlyArray<string> =>
  transcript.filter((_, i) => i % 4 === 3);

describe("section 4.2: the lazy evaluator", () => {
  it.effect("runs the book's try under lazy evaluation", () =>
    Effect.gen(function* () {
      const env = yield* setupLazyEnvironment();
      const transcript = yield* lazyDriverLoop(env, [
        "(define (try a b) (if (= a 0) 1 b))",
        "(try 0 (/ 1 0))",
      ]);
      expect(transcript.slice(-2)).toStrictEqual([";;; L-Eval value:", "1"]);
      expect(transcript[0]).toBe(";;; L-Eval input:");
    }),
  );

  it.effect("keeps the memoized counter sequence of exercise 4.27", () =>
    Effect.gen(function* () {
      const transcript = valuesOf(
        yield* driverSession([
          "(define count 0)",
          "(define (id x) (set! count (+ count 1)) x)",
          "(define w (id (id 10)))",
          "count",
          "w",
          "count",
          "w",
          "count",
        ]),
      );
      expect(transcript).toStrictEqual(["ok", "ok", "ok", "1", "10", "2", "10", "2"]);
    }),
  );

  it.effect("forces the operator position, so id can deliver a primitive", () =>
    Effect.gen(function* () {
      const transcript = valuesOf(yield* driverSession(["(define (id x) x)", "((id +) 2 3)"]));
      expect(transcript).toStrictEqual(["ok", "5"]);
    }),
  );

  it.effect("memoization answers the stored value on re-forcing", () =>
    Effect.gen(function* () {
      const transcript = valuesOf(
        yield* driverSession([
          "(define count 0)",
          "(define (id x) (set! count (+ count 1)) x)",
          "(define (square x) (* x x))",
          "(square (id 10))",
          "count",
        ]),
      );
      expect(transcript.slice(-2)).toStrictEqual(["100", "1"]);
    }),
  );

  it.effect("the unmemoized twin recomputes at every demand", () =>
    Effect.gen(function* () {
      const unmemoized = makeLazyEvaluator({ delayOperand: () => false });
      const transcript = yield* evaluateSession(
        [
          "(define count 0)",
          "(define (id x) (set! count (+ count 1)) x)",
          "(define (cube x) (* x (* x x)))",
          "(cube (id 10))",
          "count",
        ],
        unmemoized,
      );
      expect(transcript.slice(-2)).toStrictEqual(["1000", "3"]);
    }),
  );

  it.effect("Cy's sequence forces the dormant set! of exercise 4.30", () =>
    Effect.gen(function* () {
      const cy = makeLazyEvaluator({ cySequence: true });
      const source = ["(define (p2 x) (define (p e) e x) (p (set! x (cons x '(2)))))", "(p2 1)"];
      const text = valuesOf(yield* driverSession(source));
      const cyTranscript = yield* evaluateSession(source, cy);
      expect(text[text.length - 1]).toBe("1");
      expect(cyTranscript[cyTranscript.length - 1]).toBe("(1 2)");
    }),
  );

  it.effect("lazier than streams: both slots of a procedural pair delay", () =>
    Effect.gen(function* () {
      const transcript = valuesOf(
        yield* driverSession([
          "(define (cons x y) (lambda (m) (m x y)))",
          "(define (car z) (z (lambda (p q) p)))",
          "(define (cdr z) (z (lambda (p q) q)))",
          "(define (list-ref items n) (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))",
          "(car (cons 7 (/ 1 0)))",
          "(define ones (cons 1 ones))",
          "(define (add-lists list1 list2) (cond ((null? list1) list2) ((null? list2) list1) (else (cons (+ (car list1) (car list2)) (add-lists (cdr list1) (cdr list2))))))",
          "(define integers (cons 1 (add-lists ones integers)))",
          "(list-ref integers 17)",
        ]),
      );
      expect(transcript[4]).toBe("7");
      expect(transcript[5]).toBe("ok");
      expect(transcript[6]).toBe("ok");
      expect(transcript[7]).toBe("ok");
      expect(transcript[8]).toBe("18");
    }),
  );

  it.effect("the printable driver budgets an infinite lazy list", () =>
    Effect.gen(function* () {
      const printable = makeLazyEvaluator({ printablePairs: true });
      const env = yield* setupLazyEnvironment();
      const render = (input: string): Effect.Effect<string, EvaluationError> =>
        Effect.flatMap(printable.evaluate(read(input), env), (value) =>
          renderLazyValue(printable, value),
        );
      expect(yield* render("(cons 1 (cons 2 '()))")).toBe("(1 2)");
      expect(yield* render("(define ones (cons 1 ones))")).toBe("ok");
      const prefix = Array.from({ length: LAZY_PRINT_BUDGET }, () => "1").join(" ");
      expect(yield* render("ones")).toBe(`(${prefix} ...)`);
      expect(yield* render("(car ones)")).toBe("1");
      expect(yield* render("(cons (cons 1 '()) (cons 2 '()))")).toBe("((1) 2)");
    }),
  );

  it.effect("a lifted quote builds true lazy pairs", () =>
    Effect.gen(function* () {
      const lifted = makeLazyEvaluator({ liftQuotedLists: true });
      const transcript = yield* evaluateSession(
        [
          "(define (cons x y) (lambda (m) (m x y)))",
          "(define (car z) (z (lambda (p q) p)))",
          "(define (cdr z) (z (lambda (p q) q)))",
          "(car '(a b c))",
          "(car (cdr '(a b c)))",
        ],
        lifted,
      );
      expect(transcript.slice(-2)).toStrictEqual(["a", "b"]);
    }),
  );

  it.effect("an unbound variable still fails on the error channel", () =>
    Effect.gen(function* () {
      const error = yield* failureOf(
        Effect.flatMap(setupLazyEnvironment(), (env) =>
          lazyDriverLoop(env, ["(undefined-variable)"]),
        ),
      );
      expect(error._tag).toBe("UnboundVariable");
    }),
  );
});
