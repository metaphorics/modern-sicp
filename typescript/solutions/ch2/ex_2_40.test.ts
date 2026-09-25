// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { primeSumPairs, showList } from "../../packages/ch2/src/02-picture-language.js";
import { primeSumPairsSimplified, uniquePairs } from "./ex_2_40.js";

describe("exercise 2.40", () => {
  it("uniquePairs lists the i-greater-than-j pairs", () => {
    expect(showList(uniquePairs(4))).toBe("((2 1) (3 1) (3 2) (4 1) (4 2) (4 3))");
  });

  it("prime-sum-pairs built on uniquePairs matches the module's", () => {
    expect(showList(primeSumPairsSimplified(6))).toBe(showList(primeSumPairs(6)));
  });
});
