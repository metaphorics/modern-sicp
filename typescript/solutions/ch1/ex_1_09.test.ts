// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { plusIterative, plusIterativeTrace, plusRecursive, plusRecursiveTrace } from "./ex_1_09.js";

describe("exercise 1.9", () => {
  it("both additions agree at the statement's (4, 5)", () => {
    expect(plusRecursive(4, 5)).toBe(9);
    expect(plusIterative(4, 5)).toBe(9);
  });

  it("the recursive trace holds b and grows the deferred chain", () => {
    const { value, states } = plusRecursiveTrace(4, 5);
    expect(value).toBe(9);
    expect(states).toStrictEqual([
      [4, 5],
      [3, 5],
      [2, 5],
      [1, 5],
      [0, 5],
    ]);
  });

  it("the iterative trace walks the same states without deferred work", () => {
    const { value, states } = plusIterativeTrace(4, 5);
    expect(value).toBe(9);
    expect(states).toStrictEqual([
      [4, 5],
      [3, 6],
      [2, 7],
      [1, 8],
      [0, 9],
    ]);
  });

  it("the loop survives a depth the call chain cannot", () => {
    expect(plusIterative(100000, 1)).toBe(100001);
    expect(() => plusRecursive(100000, 1)).toThrow(RangeError);
  });
});
