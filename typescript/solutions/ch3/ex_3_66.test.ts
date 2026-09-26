// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { intPairs, positionOfPair } from "./ex_3_66.js";

describe("exercise 3.66: the order of the pairs stream", () => {
  it("begins with the diagonal and first row interleaved", () => {
    expect(streamTake(intPairs, 12)).toEqual([
      [1, 1],
      [1, 2],
      [2, 2],
      [1, 3],
      [2, 3],
      [1, 4],
      [3, 3],
      [1, 5],
      [2, 4],
      [1, 6],
      [3, 4],
      [1, 7],
    ]);
  });

  it("places the diagonal pair (i, i) at position 2^i - 2", () => {
    expect(positionOfPair(intPairs, [1, 1], 5000)).toBe(0);
    expect(positionOfPair(intPairs, [2, 2], 5000)).toBe(2);
    expect(positionOfPair(intPairs, [3, 3], 5000)).toBe(6);
    expect(positionOfPair(intPairs, [4, 4], 5000)).toBe(14);
    expect(positionOfPair(intPairs, [5, 5], 5000)).toBe(30);
    expect(positionOfPair(intPairs, [6, 6], 5000)).toBe(62);
    expect(positionOfPair(intPairs, [7, 7], 5000)).toBe(126);
    expect(positionOfPair(intPairs, [8, 8], 5000)).toBe(254);
    expect(positionOfPair(intPairs, [9, 9], 5000)).toBe(510);
    expect(positionOfPair(intPairs, [10, 10], 5000)).toBe(1022);
  });

  it("spreads row 1 at positions 2j - 3, so (1, 10) is at 17 and (1, 100) at 197", () => {
    expect(positionOfPair(intPairs, [1, 10], 5000)).toBe(17);
    expect(positionOfPair(intPairs, [1, 100], 5000)).toBe(197);
    for (let j = 2; j <= 100; j += 1) {
      expect(positionOfPair(intPairs, [1, j], 5000)).toBe(2 * j - 3);
    }
  });

  it("doubles each deeper row's spacing: (3, 5) is at 18, (2, 5) at 12, (4, 5) at 22", () => {
    expect(positionOfPair(intPairs, [2, 3], 5000)).toBe(4);
    expect(positionOfPair(intPairs, [2, 4], 5000)).toBe(8);
    expect(positionOfPair(intPairs, [2, 5], 5000)).toBe(12);
    expect(positionOfPair(intPairs, [3, 4], 5000)).toBe(10);
    expect(positionOfPair(intPairs, [3, 5], 5000)).toBe(18);
    expect(positionOfPair(intPairs, [4, 5], 5000)).toBe(22);
  });

  it("never emits a pair out of order: (2, 1) does not appear", () => {
    expect(positionOfPair(intPairs, [2, 1], 5000)).toBe(-1);
  });
});
