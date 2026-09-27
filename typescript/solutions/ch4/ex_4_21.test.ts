// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { evenOddValue, ex_4_21, factorialValue, fibonacciValue } from "./ex_4_21.js";

describe("exercise 4.21: recursion without define", () => {
  it.effect("the book's self-application expression computes 10 factorial", () =>
    Effect.gen(function* () {
      expect(yield* factorialValue()).toStrictEqual({ _tag: "Number", n: 3628800 });
    }),
  );

  it.effect("the Fibonacci analog computes fib 10", () =>
    Effect.gen(function* () {
      expect(yield* fibonacciValue()).toStrictEqual({ _tag: "Number", n: 55 });
    }),
  );

  it.effect("the completed even?/odd? f answers parity without internal defines", () =>
    Effect.gen(function* () {
      expect(yield* evenOddValue(5)).toStrictEqual({ _tag: "Boolean", b: false });
      expect(yield* evenOddValue(6)).toStrictEqual({ _tag: "Boolean", b: true });
    }),
  );

  it("explains the self-application trick", () => {
    expect(ex_4_21()).toContain("self-application");
  });
});
