// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { sevenFromFirst, sevenFromSecond, sevenFromThird } from "./ex_2_25.js";

describe("exercise 2.25", () => {
  it("7 sits at [2][1] of (1 3 (5 7) 9)", () => {
    expect(sevenFromFirst()).toBe(7);
  });

  it("7 sits at [0][0] of ((7))", () => {
    expect(sevenFromSecond()).toBe(7);
  });

  it("7 sits six cdr-steps down (1 (2 (3 (4 (5 (6 7))))))", () => {
    expect(sevenFromThird()).toBe(7);
  });
});
