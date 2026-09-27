// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { ackermann, ackermannF, ackermannG, ackermannH, ackermannK } from "./ex_1_10.js";

describe("exercise 1.10", () => {
  it("the three printed values", () => {
    expect(ackermann(1, 10)).toBe(1024);
    expect(ackermann(2, 4)).toBe(65536);
    expect(ackermann(3, 3)).toBe(65536);
  });

  it("the wrappers follow 2n, 2^n, and the tower of twos", () => {
    expect(ackermannF(5)).toBe(10);
    expect(ackermannG(5)).toBe(32);
    expect(ackermannG(10)).toBe(1024);
    expect(ackermannH(3)).toBe(16);
    expect(ackermannH(4)).toBe(65536);
    expect(ackermannK(3)).toBe(45);
  });
});
