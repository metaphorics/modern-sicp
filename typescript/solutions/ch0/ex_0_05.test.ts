// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { parseAmount, parseAmountEager } from "./ex_0_05.js";

describe("exercise 0.5", () => {
  it("digits with surrounding spaces parse to their value", () => {
    expect(parseAmount(" 42 ")).toEqual({ _tag: "Ok", value: 42 });
  });

  it("blank input raises Blank as a value", () => {
    expect(parseAmount("   ")).toEqual({ _tag: "Error", error: { _tag: "Blank" } });
  });

  it("a stray character raises NotDigits carrying the original input", () => {
    expect(parseAmount("12x3")).toEqual({
      _tag: "Error",
      error: { _tag: "NotDigits", input: "12x3" },
    });
  });

  it("the eager original still throws where the Result version returns", () => {
    expect(() => parseAmountEager("   ")).toThrow(RangeError);
    expect(() => parseAmountEager("12x3")).toThrow(TypeError);
    expect(parseAmount("   ")._tag).toBe("Error");
  });
});
