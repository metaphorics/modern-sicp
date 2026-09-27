// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { aOverA, aOverB, par1, par2, percent, resistorA, resistorB } from "./ex_2_14.js";

describe("exercise 2.14", () => {
  it("A / A is not the point [1, 1]: it carries a 9.975 percent tolerance", () => {
    const a1 = aOverA(resistorA);
    expect(percent(a1)).toBeCloseTo(9.975062344139651, 12);
    expect(a1.lo).toBeCloseTo(9.5 / 10.5, 12);
    expect(a1.hi).toBeCloseTo(10.5 / 9.5, 12);
  });

  it("A / B widens both operands' uncertainties", () => {
    const ab = aOverB(resistorA, resistorB);
    expect(ab.lo).toBeCloseTo(9.5 / 21, 12);
    expect(ab.hi).toBeCloseTo(10.5 / 19, 12);
    expect(percent(ab)).toBeCloseTo(9.975062344139651, 12);
  });

  it("par1 and par2 disagree on the same two resistors", () => {
    const p1 = par1(resistorA, resistorB);
    const p2 = par2(resistorA, resistorB);
    expect(percent(p1)).toBeCloseTo(14.900744416873446, 10);
    expect(percent(p2)).toBeCloseTo(5.000000000000005, 10);
    expect(percent(p1)).toBeGreaterThan(percent(p2));
  });
});
