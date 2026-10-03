// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { list, showList } from "../../packages/ch2/src/02-picture-language.js";
import { accumulateN } from "./ex_2_36.js";

const s = list(list(1, 2, 3), list(4, 5, 6), list(7, 8, 9), list(10, 11, 12));

describe("exercise 2.36", () => {
  it("accumulates the columns of the 4-by-3 matrix", () => {
    expect(showList(accumulateN((a, b) => a + b, 0, s))).toBe("[22, 26, 30]");
  });
});
