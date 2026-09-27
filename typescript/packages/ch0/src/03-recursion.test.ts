// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.3

import { describe, expect, it } from "vitest";

import { factorial, factorialIter, fib, naturals, squaresOf, sumTo, take } from "./03-recursion.js";

describe("recursion and the call stack", () => {
  it("the recursive listings return the prose values", () => {
    expect(factorial(18)).toBe(6402373705728000);
    expect(fib(10)).toBe(55);
  });

  it("the loop carries the state instead of the stack", () => {
    expect(factorialIter(18)).toBe(6402373705728000);
    expect(sumTo(100)).toBe(5050);
  });

  it("a generator defers every step until it is driven", () => {
    expect(Array.from(take(squaresOf(naturals()), 5))).toEqual([1, 4, 9, 16, 25]);
    expect(Array.from(take(naturals(), 3))).toEqual([1, 2, 3]);
  });

  it("a fresh generator recomputes from the start", () => {
    const first = take(squaresOf(naturals()), 3);
    const second = take(squaresOf(naturals()), 3);
    expect(Array.from(first)).toEqual(Array.from(second));
  });
});
