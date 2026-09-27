// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { divIntervalChecked } from "./ex_2_10.js";

describe("exercise 2.10", () => {
  it("a valid division still computes", () => {
    expect(divIntervalChecked({ lo: 6, hi: 8 }, { lo: 2, hi: 4 })).toStrictEqual({
      _tag: "Ok",
      value: { lo: 1.5, hi: 4 },
    });
  });

  it("dividing by an interval that spans zero reports ZeroSpan", () => {
    expect(divIntervalChecked({ lo: 6, hi: 8 }, { lo: -1, hi: 1 })).toStrictEqual({
      _tag: "Error",
      error: { _tag: "ZeroSpan", lo: -1, hi: 1 },
    });
  });

  it("touching zero at a bound is refused too", () => {
    const r = divIntervalChecked({ lo: 6, hi: 8 }, { lo: 0, hi: 1 });
    expect(r._tag).toBe("Error");
    const r2 = divIntervalChecked({ lo: 6, hi: 8 }, { lo: -1, hi: 0 });
    expect(r2._tag).toBe("Error");
  });
});
