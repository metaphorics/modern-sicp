// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { mlist } from "../../packages/ch3/src/03-mutable-data.js";
import { makeCycle } from "./ex_3_13.js";
import { countPairs, plainX3, sharedX2, sharedX3 } from "./ex_3_16.js";
import { countPairsCorrect } from "./ex_3_17.js";

describe("exercise 3.17: counting distinct pairs with a visited set", () => {
  it("returns 3 on every three-pair structure Ben miscounted", () => {
    expect(countPairsCorrect(plainX3())).toBe(3);
    expect(countPairsCorrect(sharedX2())).toBe(3);
    expect(countPairsCorrect(sharedX3())).toBe(3);
  });

  it("where Ben sees 4 and 7, the correct count sees 3 and 3", () => {
    expect(countPairs(sharedX2())).toBe(4);
    expect(countPairsCorrect(sharedX2())).toBe(3);
    expect(countPairs(sharedX3())).toBe(7);
    expect(countPairsCorrect(sharedX3())).toBe(3);
  });

  it("agrees with Ben when nothing is shared", () => {
    expect(countPairsCorrect(plainX3())).toBe(countPairs(plainX3()));
  });

  it("returns 3 on a cyclic three-pair ring; Ben's count is never run on it", () => {
    expect(countPairsCorrect(makeCycle(mlist("a", "b", "c")))).toBe(3);
  });
});
