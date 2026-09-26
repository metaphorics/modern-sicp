// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { iterativeFactorialStack, iterativeFactorialTable } from "./ex_5_26.js";

describe("exercise 5.26 iterative factorial stack", () => {
  it("measures 204 pushes at the constant depth 10 for n = 5", () => {
    expect(iterativeFactorialStack(5)).toEqual({ n: 5, pushes: 204, maximumDepth: 10 });
  });
  it("fits 35n + 29 pushes with depth independent of n across the table", () => {
    for (const row of iterativeFactorialTable()) {
      expect(row.pushes).toBe(35 * row.n + 29);
      expect(row.maximumDepth).toBe(10);
    }
  });
});
