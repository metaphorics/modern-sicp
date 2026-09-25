// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { dampsRequired, nthRoot } from "./ex_1_45.js";

describe("exercise 1.45", () => {
  it("one damp covers square and cube roots, the doubling pattern follows", () => {
    expect(dampsRequired(2)).toBe(1);
    expect(dampsRequired(3)).toBe(1);
    expect(dampsRequired(4)).toBe(2);
    expect(dampsRequired(7)).toBe(2);
    expect(dampsRequired(8)).toBe(3);
    expect(dampsRequired(15)).toBe(3);
    expect(dampsRequired(16)).toBe(4);
  });

  it("the once-damped square root of 16 converges in six steps", () => {
    expect(nthRoot(16, 2, 1)).toStrictEqual({ root: 4.000000000000051, steps: 6 });
  });

  it("the twice-damped 4th root of 10 converges to the true root", () => {
    expect(nthRoot(10, 4, 2)).toStrictEqual({ root: 1.7782794100444472, steps: 7 });
    // the pinned root sits 5.5e-12 from the true fourth root of 10
    expect(Math.abs(1.7782794100444472 - 10 ** 0.25)).toBeLessThan(1e-5);
  });

  it("the thrice-damped 8th root and four-damped 16th root converge", () => {
    expect(nthRoot(10, 8, 3)).toStrictEqual({ root: 1.333521432163324, steps: 9 });
    expect(nthRoot(10, 16, 4)).toStrictEqual({ root: 1.154781984689469, steps: 10 });
  });

  it("an under-damped search reports failure at the cap instead of diverging", () => {
    expect(nthRoot(10, 4, 1)).toStrictEqual({ root: null, steps: 1000 });
    expect(nthRoot(10, 8, 2)).toStrictEqual({ root: null, steps: 1000 });
  });
});
