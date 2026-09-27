// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { car, cdr, cons } from "./ex_2_05.js";

describe("exercise 2.5", () => {
  it("the encoding round-trips small pairs exactly", () => {
    expect(cons(3n, 2n)).toBe(72n);
    expect(car(cons(3n, 2n))).toBe(3n);
    expect(cdr(cons(3n, 2n))).toBe(2n);
    expect(car(cons(0n, 0n))).toBe(0n);
    expect(cdr(cons(5n, 0n))).toBe(0n);
  });

  it("the encoding stays exact far beyond the 53 exact bits of number", () => {
    // 2^53 * 3 has 55 significant bits; as a number the encoding would
    // already be inexact, and 6^100 would be hopeless. bigint is exact.
    const p = cons(53n, 1n);
    expect(p).toBe(2n ** 53n * 3n);
    expect(car(p)).toBe(53n);
    expect(cdr(p)).toBe(1n);
    const huge = cons(100n, 100n);
    expect(car(huge)).toBe(100n);
    expect(cdr(huge)).toBe(100n);
  });
});
