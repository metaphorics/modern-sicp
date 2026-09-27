// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 3.2

import { it } from "@effect/vitest";
import { Effect } from "effect";
import { describe, expect } from "vitest";

import { makeWithdraw } from "./01-assignment.js";
import { f, sqrt, square, squareFn, sumOfSquares } from "./02-environment.js";

describe("section 3.2: the environment model", () => {
  it("square answers 25 and the arrow spelling is one procedure object among many", () => {
    expect(square(5)).toBe(25);
    expect(squareFn(5)).toBe(25);
    expect(squareFn).not.toBe(square);
  });

  it("f(5) evaluates to 136, each call frame keeping its own x", () => {
    expect(sumOfSquares(6, 10)).toBe(136);
    expect(f(5)).toBe(136);
    // The two square calls cannot interfere: each built its own frame.
    expect(sumOfSquares(3, 4)).toBe(25);
  });

  it.effect("w1 keeps its balance between calls and w2 never sees it", () =>
    Effect.gen(function* () {
      const w1 = makeWithdraw(100);
      expect(yield* w1(50)).toBe(50);
      expect(yield* w1(20)).toBe(30);
      const w2 = makeWithdraw(100);
      expect(yield* w2(10)).toBe(90);
    }),
  );

  it("sqrt(2) with internal definitions converges on 1.4142...", () => {
    expect(sqrt(2)).toBe(1.4142156862745097);
    expect(sqrt(9)).toBe(3.00009155413138);
  });
});
