// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { double, puzzleAnswer } from "./ex_1_41.js";

describe("exercise 1.41", () => {
  it("double(inc) adds 2", () => {
    const inc = (x: number): number => x + 1;
    expect(double(inc)(5)).toBe(7);
    expect(double(double(inc))(5)).toBe(9);
  });

  it("the puzzle's value is 21", () => {
    expect(puzzleAnswer()).toBe(21);
  });

  it("double doubles any procedure's effect", () => {
    expect(double((x: number): number => x * 3)(2)).toBe(18);
    expect(double((x: number): number => x)(7)).toBe(7);
  });
});
