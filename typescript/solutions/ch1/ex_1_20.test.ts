// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { tracedGcd } from "./ex_1_20.js";

describe("exercise 1.20", () => {
  it("the eager gcd computes the remainder four times for (206, 40)", () => {
    expect(tracedGcd(206, 40)).toStrictEqual({ value: 2, remainderCalls: 4 });
  });

  it("the reduction chain matches the section's trace", () => {
    // (206, 40) -> (40, 6) -> (6, 4) -> (4, 2) -> (2, 0): four remainders.
    expect(tracedGcd(40, 6).remainderCalls).toBe(3);
    expect(tracedGcd(6, 4).remainderCalls).toBe(2);
    expect(tracedGcd(4, 2).remainderCalls).toBe(1);
    expect(tracedGcd(2, 0).remainderCalls).toBe(0);
  });
});
