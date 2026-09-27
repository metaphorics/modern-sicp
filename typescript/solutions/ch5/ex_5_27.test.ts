// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { recursiveFactorialStack, recursiveFactorialTable } from "./ex_5_27.js";

describe("exercise 5.27 recursive factorial stack", () => {
  it("reproduces the book's session: 144 pushes at depth 28 for n = 5", () => {
    expect(recursiveFactorialStack(5)).toEqual({ n: 5, pushes: 144, maximumDepth: 28 });
  });
  it("fits 32n - 16 pushes and 5n + 3 depth across the table", () => {
    for (const row of recursiveFactorialTable()) {
      expect(row.pushes).toBe(32 * row.n - 16);
      expect(row.maximumDepth).toBe(5 * row.n + 3);
    }
  });
});
