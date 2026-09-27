// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { par1, par2, par2IsTighter, percent } from "./ex_2_15.js";

const r1 = { lo: 9.5, hi: 10.5 };
const r2 = { lo: 19, hi: 21 };

describe("exercise 2.15", () => {
  it("par2 gives the tighter percentage tolerance", () => {
    expect(par2IsTighter(r1, r2)).toBe(true);
    expect(percent(par1(r1, r2))).toBeCloseTo(14.900744416873446, 10);
    expect(percent(par2(r1, r2))).toBeCloseTo(5.000000000000005, 10);
  });

  it("par2's bounds sit inside par1's", () => {
    const p1 = par1(r1, r2);
    const p2 = par2(r1, r2);
    expect(p2.lo).toBeGreaterThanOrEqual(p1.lo);
    expect(p2.hi).toBeLessThanOrEqual(p1.hi);
  });
});
