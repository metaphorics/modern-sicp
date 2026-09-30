// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_5_26 } from "./ex_5_26.ts";

describe("exercise 5.26 iterative factorial stack behavior", () => {
  it("pushes grow linearly while iterative stack depth stays constant", () => {
    const table = ex_5_26();
    const pushSteps = table.slice(1).map((row, i) => row.pushes - (table[i]?.pushes ?? 0));
    const depthSteps = table.slice(1).map((row, i) => row.maxDepth - (table[i]?.maxDepth ?? 0));
    expect(new Set(pushSteps).size).toBe(1);
    expect(pushSteps[0]).toBeGreaterThan(0);
    expect(new Set(table.map((row) => row.maxDepth)).size).toBe(1);
    expect(depthSteps.every((step) => step === 0)).toBe(true);
  });
  it("the rows cover n = 1 to 6 with the machine's own counters", () => {
    expect(ex_5_26().map((row) => row.n)).toEqual([1, 2, 3, 4, 5, 6]);
  });
});
