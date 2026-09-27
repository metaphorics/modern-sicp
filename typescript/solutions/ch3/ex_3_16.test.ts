// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { countPairs, plainX3, sharedX2, sharedX3 } from "./ex_3_16.js";

describe("exercise 3.16: Ben's count-pairs double counts", () => {
  it("returns 3 on three pairs with no sharing", () => {
    expect(countPairs(plainX3())).toBe(3);
  });

  it("returns 4 on three pairs with one shared", () => {
    expect(countPairs(sharedX2())).toBe(4);
  });

  it("returns 7 on three pairs each reachable by two routes", () => {
    expect(countPairs(sharedX3())).toBe(7);
  });
});
