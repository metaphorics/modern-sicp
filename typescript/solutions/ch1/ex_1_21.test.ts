// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { smallestDivisor, smallestDivisorAnswers } from "./ex_1_21.js";

describe("exercise 1.21", () => {
  it("the three answers are 199, 1999, and 7", () => {
    expect(smallestDivisorAnswers()).toStrictEqual([199, 1999, 7]);
  });

  it("two of the three are therefore prime", () => {
    expect(smallestDivisor(199)).toBe(199);
    expect(smallestDivisor(1999)).toBe(1999);
    expect(smallestDivisor(19999)).not.toBe(19999);
    expect(19999 % 7).toBe(0);
  });
});
