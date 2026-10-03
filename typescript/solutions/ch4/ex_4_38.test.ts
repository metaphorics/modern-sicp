// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { bruteForceCount, solutions } from "./ex_4_38.js";

describe("exercise 4.38: multiple dwelling without Smith-Fletcher", () => {
  it("the five solutions come out in search order", () => {
    expect(solutions(false)).toEqual([
      "{ baker: 1, cooper: 2, fletcher: 4, miller: 3, smith: 5 }",
      "{ baker: 1, cooper: 2, fletcher: 4, miller: 5, smith: 3 }",
      "{ baker: 1, cooper: 4, fletcher: 2, miller: 5, smith: 3 }",
      "{ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 }",
      "{ baker: 3, cooper: 4, fletcher: 2, miller: 5, smith: 1 }",
    ]);
  });

  it("the brute force agrees: five without the clause, one with it", () => {
    expect(bruteForceCount(false)).toBe(5);
    expect(bruteForceCount(true)).toBe(1);
    expect(solutions(true)).toEqual(["{ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 }"]);
  });
});
