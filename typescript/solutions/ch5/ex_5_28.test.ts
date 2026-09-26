// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  naiveFactorialValue,
  naiveIterativeFactorialStack,
  naiveRecursiveFactorialStack,
} from "./ex_5_28.js";

describe("exercise 5.28 the tail recursion removed", () => {
  it("measures the recursive factorial at 34n - 16 pushes, 8n + 3 depth", () => {
    expect(naiveRecursiveFactorialStack(5)).toEqual({ n: 5, pushes: 154, maximumDepth: 43 });
    for (const n of [1, 2, 3, 4, 6]) {
      expect(naiveRecursiveFactorialStack(n).pushes).toBe(34 * n - 16);
      expect(naiveRecursiveFactorialStack(n).maximumDepth).toBe(8 * n + 3);
    }
  });
  it("the iterative factorial loses constant space: 37n + 33 pushes, 3n + 14 depth", () => {
    expect(naiveIterativeFactorialStack(5)).toEqual({ n: 5, pushes: 218, maximumDepth: 29 });
    for (const n of [1, 2, 3, 4, 6]) {
      expect(naiveIterativeFactorialStack(n).pushes).toBe(37 * n + 33);
      expect(naiveIterativeFactorialStack(n).maximumDepth).toBe(3 * n + 14);
    }
  });
  it("both programs still answer 120 at n = 5", () => {
    expect(naiveFactorialValue(5)).toBe("120");
  });
});
