// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, nil, showList } from "../../packages/ch2/src/02-picture-language.js";

import { cc, exceptFirstDenomination, firstDenomination, noMore, usCoins } from "./ex_2_19.js";

describe("exercise 2.19", () => {
  it("counts 292 ways to change 100 US dollars", () => {
    expect(cc(100, usCoins)).toBe(292);
  });

  it("does not care about the order of the denominations", () => {
    expect(cc(100, list(1, 5, 10, 25, 50))).toBe(292);
  });

  it("counts 4 ways to make 11 from (50 25 10 5 1)", () => {
    expect(cc(11, usCoins)).toBe(4);
  });

  it("the selectors see the first denomination and the rest", () => {
    expect(firstDenomination(usCoins)).toBe(50);
    expect(showList(exceptFirstDenomination(usCoins))).toBe("(25 10 5 1)");
    expect(noMore(usCoins)).toBe(false);
    expect(noMore(nil)).toBe(true);
  });
});
