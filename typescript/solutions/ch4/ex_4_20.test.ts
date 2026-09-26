// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";
import { format, read } from "../../packages/ch4/src/read.js";
import { ex_4_20, letrecToLet, runProgramsWithLetrec, runWithLetrec } from "./ex_4_20.js";

const evenOddLetrec =
  "(letrec ((even? (lambda (n) (if (= n 0) true (odd? (- n 1))))) (odd? (lambda (n) (if (= n 0) false (even? (- n 1)))))) (even? 5))";

const factorialLetrec =
  "(letrec ((fact (lambda (n) (if (= n 1) 1 (* n (fact (- n 1))))))) (fact 10))";

const louisLet = "(let ((a 1) (b (+ a 1))) b)";

const louisLetrec = "(letrec ((a 1) (b (+ a 1))) b)";

describe("exercise 4.20: letrec as a derived expression", () => {
  it.effect("mutually recursive letrec bindings answer the book's even?", () =>
    Effect.gen(function* () {
      expect(yield* runWithLetrec(evenOddLetrec)).toStrictEqual({ _tag: "Boolean", b: false });
    }),
  );

  it.effect("a recursive fact binding evaluates 10 factorial", () =>
    Effect.gen(function* () {
      expect(yield* runWithLetrec(factorialLetrec)).toStrictEqual({ _tag: "Number", n: 3628800 });
    }),
  );

  it.effect("letrec works nested inside a lambda body", () =>
    Effect.gen(function* () {
      expect(
        yield* runProgramsWithLetrec([
          "(define (f x) (letrec ((double (lambda (n) (* 2 n)))) (double x)))",
          "(f 21)",
        ]),
      ).toStrictEqual({ _tag: "Number", n: 42 });
    }),
  );

  it.effect("Louis's plain let fails where the letrec analog works", () =>
    Effect.gen(function* () {
      const outcome = yield* Effect.result(runWithLetrec(louisLet));
      expect(outcome._tag).toBe("Failure");
      if (outcome._tag === "Failure") {
        expect(outcome.failure._tag).toBe("UnboundVariable");
      }
      expect(yield* runWithLetrec(louisLetrec)).toStrictEqual({ _tag: "Number", n: 2 });
    }),
  );

  it("letrecToLet emits the let-and-set! shape the book asks for", () => {
    const form = read("(letrec ((a 1) (b (+ a 1))) (+ a b))");
    if (form._tag !== "Cons") {
      throw new Error("expected a cons form");
    }
    const transformed = letrecToLet(form);
    expect(format(transformed)).toBe(
      "(let ((a (quote *unassigned*)) (b (quote *unassigned*))) (set! a 1) (set! b (+ a 1)) (+ a b))",
    );
  });

  it("explains what is loose about Louis's reasoning", () => {
    expect(ex_4_20()).toContain("evaluates its inits outside the new bindings");
  });
});
