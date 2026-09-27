// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { p, test, testThunk } from "./ex_1_05.js";

describe("exercise 1.5", () => {
  it("test answers 0 when x is 0 and otherwise its second argument", () => {
    expect(test(0, 5)).toBe(0);
    expect(test(1, 9)).toBe(9);
  });

  it("applicative order: evaluating the argument p() diverges before the call", () => {
    expect(() => test(0, p())).toThrow(RangeError);
  });

  it("normal order: the thunk stand-in returns 0 without running its argument", () => {
    let called = false;
    const thunk = (): number => {
      called = true;
      return 5;
    };
    expect(testThunk(0, thunk)).toBe(0);
    expect(called).toBe(false);
  });

  it("the thunk stand-in does evaluate the argument when x is not 0", () => {
    expect(testThunk(2, () => 9)).toBe(9);
  });
});
