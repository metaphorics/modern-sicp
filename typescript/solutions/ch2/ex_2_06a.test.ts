// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { churchAdd, churchToInt, one, two } from "./ex_2_06.js";
import { succ, three } from "./ex_2_06a.js";

describe("exercise 2.6a", () => {
  it("the direct three, succ(two), and churchAdd(two, one) all decode to 3", () => {
    expect(churchToInt(three)).toBe(3);
    expect(churchToInt(succ(two))).toBe(3);
    expect(churchToInt(churchAdd(two, one))).toBe(3);
  });

  it("the successor and addition decode consistently: 4 and 6", () => {
    expect(churchToInt(succ(three))).toBe(4);
    expect(churchToInt(churchAdd(three, three))).toBe(6);
  });

  it("succ(succ(one)) decodes the same as the direct three", () => {
    expect(churchToInt(succ(succ(one)))).toBe(churchToInt(three));
  });
});
