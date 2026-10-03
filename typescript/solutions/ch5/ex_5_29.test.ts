// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_29 } from "./ex_5_29.ts";

describe("exercise 5.29 tree-recursive Fibonacci stack behavior", () => {
  it("the pushes grow with the recursion tree while the depth steps uniformly", () => {
    const table = ex_5_29();
    const depthSteps = table.slice(1).map((row, i) => row.maxDepth - (table[i]?.maxDepth ?? 0));
    expect(new Set(depthSteps).size).toBe(1);
    const pushSteps = table.slice(1).map((row, i) => row.pushes - (table[i]?.pushes ?? 0));
    expect(pushSteps[pushSteps.length - 1]).toBeGreaterThan(pushSteps[0] ?? 0);
  });
});
