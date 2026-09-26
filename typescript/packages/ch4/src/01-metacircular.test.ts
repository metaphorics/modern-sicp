// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 4.1

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import {
  analyze,
  applyProcedure,
  driverLoop,
  evalAnalyzed,
  evalSequence,
  evalString,
  falseValue,
  isTrue,
  listOfValues,
  setupEnvironment,
  symbol,
} from "./01-metacircular.js";
import type { Env, Value } from "./core.js";
import {
  ArityMismatch,
  type EvaluationError,
  NotAProcedure,
  RuntimeError,
  UnboundVariable,
} from "./errors.js";
import { nil } from "./list.js";
import { format, read, readAll } from "./read.js";

const evalProgram = (source: string, env: Env): Effect.Effect<Value, EvaluationError> =>
  evalString(source, env);

const evalAll = (sources: ReadonlyArray<string>, env: Env): Effect.Effect<Value, EvaluationError> =>
  Effect.flatMap(
    Effect.forEach(sources, (source) => evalProgram(source, env)),
    (values) => Effect.succeed(values[values.length - 1] as Value),
  );

const failureOf = (source: string, env: Env): Effect.Effect<EvaluationError> =>
  Effect.flatMap(Effect.result(evalProgram(source, env)), (outcome) =>
    outcome._tag === "Failure"
      ? Effect.succeed(outcome.failure)
      : Effect.die(new Error("expected a failure")),
  );

const makeEnv = (): Effect.Effect<Env> => setupEnvironment();

describe("section 4.1: the metacircular evaluator", () => {
  it.effect("evaluates self-evaluating expressions and quoted data", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      expect(yield* evalProgram("(+ 2 3)", env)).toStrictEqual({ _tag: "Number", n: 5 });
      expect(format(yield* evalProgram("42", env))).toBe("42");
      expect(format(yield* evalProgram('"hello"', env))).toBe("hello");
      expect(format(yield* evalProgram("'(a b c)", env))).toBe("(a b c)");
      expect(format(yield* evalProgram("''a", env))).toBe("(quote a)");
      expect(format(yield* evalProgram("#t", env))).toBe("#t");
    }),
  );

  it.effect("runs the book's define-lambda-apply cycle", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      expect(format(yield* evalProgram("(define (square x) (* x x))", env))).toBe("ok");
      expect(yield* evalProgram("(square 7)", env)).toStrictEqual({ _tag: "Number", n: 49 });
      expect(format(yield* evalProgram("(define x 3)", env))).toBe("ok");
      expect(yield* evalProgram("(+ (square x) 1)", env)).toStrictEqual({ _tag: "Number", n: 10 });
    }),
  );

  it.effect("prints the book's 4.1.4 sample session", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      const transcript = yield* driverLoop(env, [
        "(define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))",
        "(append '(a b c) '(d e f))",
        "(car '(a b))",
      ]);
      expect(transcript).toStrictEqual([
        ";;; M-Eval input:",
        "(define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))",
        ";;; M-Eval value:",
        "ok",
        ";;; M-Eval input:",
        "(append '(a b c) '(d e f))",
        ";;; M-Eval value:",
        "(a b c d e f)",
        ";;; M-Eval input:",
        "(car '(a b))",
        ";;; M-Eval value:",
        "a",
      ]);
    }),
  );

  it.effect("set! writes the frame that defines the name, seen through closures", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      yield* evalAll(
        [
          "(define (make-counter) (define n 0) (lambda () (set! n (+ n 1)) n))",
          "(define c1 (make-counter))",
          "(define c2 (make-counter))",
        ],
        env,
      );
      expect(yield* evalProgram("(c1)", env)).toStrictEqual({ _tag: "Number", n: 1 });
      expect(yield* evalProgram("(c1)", env)).toStrictEqual({ _tag: "Number", n: 2 });
      expect(yield* evalProgram("(c2)", env)).toStrictEqual({ _tag: "Number", n: 1 });
    }),
  );

  it.effect("cond and begin evaluate as the book defines them", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      yield* evalProgram(
        "(define (rank n) (cond ((< n 10) 'small) ((< n 100) 'medium) (else 'large)))",
        env,
      );
      expect(format(yield* evalProgram("(rank 5)", env))).toBe("small");
      expect(format(yield* evalProgram("(rank 50)", env))).toBe("medium");
      expect(format(yield* evalProgram("(rank 500)", env))).toBe("large");
      expect(
        yield* evalProgram("(begin (define a 1) (define b 2) (set! a (+ a b)) a)", env),
      ).toStrictEqual({
        _tag: "Number",
        n: 3,
      });
    }),
  );

  it.effect("factorial of 10 comes out right through the driver loop", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      const transcript = yield* driverLoop(env, [
        "(define (factorial n) (if (= n 1) 1 (* n (factorial (- n 1)))))",
        "(factorial 10)",
      ]);
      expect(transcript[7]).toBe("3628800");
    }),
  );

  it.effect("the evaluator stays stack-safe on deep object recursion", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      yield* evalProgram("(define (countdown n) (if (= n 0) 0 (countdown (- n 1))))", env);
      const value = yield* evalProgram("(countdown 20000)", env);
      expect(value).toStrictEqual({ _tag: "Number", n: 0 });
    }),
  );

  it.effect("failures land on the checked error channel with the right variant", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      expect(yield* failureOf("(some-unbound-name)", env)).toBeInstanceOf(UnboundVariable);
      expect(yield* failureOf("(1 2 3)", env)).toBeInstanceOf(NotAProcedure);
      expect(yield* failureOf("(car 5)", env)).toBeInstanceOf(RuntimeError);
      expect(yield* failureOf("((lambda (x y) x) 1)", env)).toBeInstanceOf(ArityMismatch);
    }),
  );

  it.effect("the error primitive carries its message and irritants", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      const failure = yield* failureOf('(error "signal: bad input" 23)', env);
      expect(failure._tag).toBe("RuntimeError");
      if (failure._tag === "RuntimeError") {
        expect(failure.message).toBe("signal: bad input");
        expect(failure.detail).toBe("23");
      }
    }),
  );

  it.effect("apply and list-of-values behave as standalone book procedures", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      const plus = yield* evalProgram("+", env);
      const operandForm = read("(1 2 3)");
      if (operandForm._tag !== "Cons" && operandForm._tag !== "Nil") {
        return yield* Effect.die(new Error("expected an operand list"));
      }
      const args = yield* listOfValues(operandForm, env);
      const sum = yield* applyProcedure(plus, args);
      expect(sum).toStrictEqual({ _tag: "Number", n: 6 });
      expect(isTrue(falseValue)).toBe(false);
      expect(isTrue(nil)).toBe(true);
      expect(isTrue(symbol("false"))).toBe(true);
    }),
  );

  it.effect("eval-sequence demands a nonempty body", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      const outcome = yield* Effect.result(evalSequence(nil, env));
      expect(outcome._tag).toBe("Failure");
      if (outcome._tag === "Failure") {
        const failure = outcome.failure;
        expect(failure._tag).toBe("RuntimeError");
        if (failure._tag === "RuntimeError") {
          expect(failure.message).toBe("Empty sequence: EVAL");
        }
      }
    }),
  );

  it.effect("the analyzed evaluator runs the same programs", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      yield* evalAll(
        [
          "(define (square x) (* x x))",
          "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))",
        ],
        env,
      );
      let value: Value = nil;
      for (const exp of readAll("(fib 10)")) {
        value = yield* evalAnalyzed(exp, env);
      }
      expect(value).toStrictEqual({ _tag: "Number", n: 55 });
      const execution = analyze(read("(square 12)"));
      expect(yield* execution(env)).toStrictEqual({ _tag: "Number", n: 144 });
    }),
  );

  it.effect("malformed programs fail on the error channel instead of throwing", () =>
    Effect.gen(function* () {
      const env = yield* makeEnv();
      expect(yield* failureOf("(if)", env)).toBeInstanceOf(UnboundVariable);
      expect(yield* failureOf("(moose 1)", env)).toBeInstanceOf(UnboundVariable);
    }),
  );
});
