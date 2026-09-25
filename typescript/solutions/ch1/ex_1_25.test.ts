// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { expmod, expmodSimplified, expmodSimplifiedBigint } from "./ex_1_25.js";

describe("exercise 1.25", () => {
  it("already disagrees with expmod once the product passes 2^53", () => {
    expect(expmod(3, 40, 101)).toBe(87);
    expect(expmodSimplified(3, 40, 101)).not.toBe(87);
  });

  it("returns NaN at prime-testing exponents where expmod stays correct", () => {
    expect(expmod(3, 1000003, 1000003)).toBe(3);
    expect(Number.isNaN(expmodSimplified(3, 1000003, 1000003))).toBe(true);
  });

  it("the bigint restatement is exact where number is not", () => {
    expect(expmodSimplifiedBigint(3n, 100003, 1000003n)).toBe(575610n);
    expect(expmodSimplifiedBigint(2n, 1015, 561n)).toBe(230n);
  });
});
