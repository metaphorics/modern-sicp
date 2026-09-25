// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { fIter, fRecursive } from "./ex_1_11.js";

describe("exercise 1.11", () => {
  it("the base cases are the identity", () => {
    expect(fRecursive(0)).toBe(0);
    expect(fRecursive(1)).toBe(1);
    expect(fRecursive(2)).toBe(2);
    expect(fIter(0)).toBe(0);
    expect(fIter(1)).toBe(1);
    expect(fIter(2)).toBe(2);
  });

  it("the recursion lands on the hand-computed ladder", () => {
    expect([3, 4, 5, 6].map(fIter)).toStrictEqual([4, 11, 25, 59]);
    expect(fIter(10)).toBe(1892);
    expect(fIter(15)).toBe(142717);
  });

  it("both spellings agree for every n up to 25", () => {
    for (let n = 0; n <= 25; n += 1) {
      expect(fIter(n)).toBe(fRecursive(n));
    }
  });
});
