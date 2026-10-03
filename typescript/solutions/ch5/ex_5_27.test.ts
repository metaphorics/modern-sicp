// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_26 } from "./ex_5_26.ts";
import { ex_5_27 } from "./ex_5_27.ts";

describe("exercise 5.27 recursive factorial stack behavior", () => {
  it("the recursive process pushes more per level than the iterative one", () => {
    const recursive = ex_5_27();
    const iterative = ex_5_26();
    const step = (rows: readonly { pushes: number }[]): number =>
      (rows[1]?.pushes ?? 0) - (rows[0]?.pushes ?? 0);
    expect(step(recursive)).toBeGreaterThanOrEqual(step(iterative));
    expect(
      new Set(recursive.slice(1).map((row, i) => row.pushes - (recursive[i]?.pushes ?? 0))).size,
    ).toBe(1);
  });
  it("the recursive branch adds a constant depth after the base case", () => {
    const recursive = ex_5_27();
    const depthSteps = recursive
      .slice(1)
      .map((row, i) => row.maxDepth - (recursive[i]?.maxDepth ?? 0));
    const recursiveSteps = depthSteps.slice(1);
    expect(new Set(recursiveSteps).size).toBe(1);
    expect(recursiveSteps[0]).toBeGreaterThan(0);
    expect(depthSteps[0]).toBeGreaterThanOrEqual(0);
  });
});
