// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { add, div, ex_1_02, mul, sub } from "./ex_1_02.js";

describe("exercise 1.2", () => {
  it("the nested application evaluates to -37/150 in float form", () => {
    expect(ex_1_02()).toBe(-0.24666666666666667);
  });

  it("the nested calls and the infix expression agree exactly", () => {
    expect(ex_1_02()).toBe((5 + 4 + (2 - (3 - (6 + 4 / 5)))) / (3 * (6 - 2) * (2 - 7)));
  });

  it("the four primitives behave as the operators they wrap", () => {
    expect(add(2, 3)).toBe(5);
    expect(sub(2, 3)).toBe(-1);
    expect(mul(2, 3)).toBe(6);
    expect(div(6, 4)).toBe(1.5);
  });
});
