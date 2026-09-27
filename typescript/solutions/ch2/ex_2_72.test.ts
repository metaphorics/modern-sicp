// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { stepsAt } from "./ex_2_72.js";

describe("exercise 2.72", () => {
  it("measures linear cost for the most frequent symbol", () => {
    expect(stepsAt(5)[0]).toBe(5);
    expect(stepsAt(10)[0]).toBe(10);
    expect(stepsAt(20)[0]).toBe(20);
  });

  it("measures quadratic cost for the least frequent symbol", () => {
    // Subtree sizes along the skewed spine: n + (n-1) + ... + 2.
    expect(stepsAt(5)[1]).toBe(14);
    expect(stepsAt(10)[1]).toBe(54);
    expect(stepsAt(20)[1]).toBe(209);
    // Doubling n roughly quadruples the least-frequent count.
    expect(stepsAt(20)[1]).toBeGreaterThan(3.8 * stepsAt(10)[1]);
  });
});
