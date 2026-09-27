// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { subInterval, xMinusX } from "./ex_2_16.js";

describe("exercise 2.16", () => {
  it("x - x comes out wide because the occurrences are independent", () => {
    const r = { lo: 9.5, hi: 10.5 };
    expect(xMinusX(r)).toStrictEqual({ lo: -1, hi: 1 });
  });

  it("independent intervals subtract exactly as algebra expects", () => {
    expect(subInterval({ lo: 10, hi: 14 }, { lo: 2, hi: 6 })).toStrictEqual({ lo: 4, hi: 12 });
  });
});
