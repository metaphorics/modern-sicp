// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { subInterval, width } from "./ex_2_08.js";

describe("exercise 2.8", () => {
  it("the difference covers every possible difference", () => {
    expect(subInterval({ lo: 2, hi: 6 }, { lo: 10, hi: 14 })).toStrictEqual({ lo: -12, hi: -4 });
    expect(subInterval({ lo: 10, hi: 14 }, { lo: 2, hi: 6 })).toStrictEqual({ lo: 4, hi: 12 });
  });

  it("the width of the difference is the sum of the widths", () => {
    const x = { lo: 2, hi: 6 };
    const y = { lo: 10, hi: 14 };
    expect(width(subInterval(x, y))).toBe(width(x) + width(y));
  });
});
