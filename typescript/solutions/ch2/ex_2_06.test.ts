// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { add1, churchAdd, churchToInt, churchZero, one, two } from "./ex_2_06.js";

describe("exercise 2.6", () => {
  it("the statement's pieces decode as the numbers they encode", () => {
    expect(churchToInt(churchZero)).toBe(0);
    expect(churchToInt(add1(churchZero))).toBe(1);
  });

  it("one and two are direct and decode to 1 and 2", () => {
    expect(churchToInt(one)).toBe(1);
    expect(churchToInt(two)).toBe(2);
    expect(churchToInt(add1(one))).toBe(2);
  });

  it("churchAdd is direct and adds without repeated add1", () => {
    expect(churchToInt(churchAdd(one, one))).toBe(2);
    expect(churchToInt(churchAdd(one, two))).toBe(3);
    expect(churchToInt(churchAdd(two, two))).toBe(4);
  });
});
