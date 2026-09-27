// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { integers, pairs, streamTake } from "../../packages/ch3/src/05-streams.js";

import { allPairs, positionOfPair } from "./ex_3_67.js";

describe("exercise 3.67: all pairs of two streams", () => {
  it("begins with head, row and column interleaved", () => {
    expect(streamTake(allPairs(integers, integers), 12)).toEqual([
      [1, 1],
      [1, 2],
      [2, 1],
      [1, 3],
      [2, 2],
      [1, 4],
      [3, 1],
      [1, 5],
      [2, 3],
      [1, 6],
      [4, 1],
      [1, 7],
    ]);
  });

  it("emits both orders of every pair near the top", () => {
    expect(positionOfPair(allPairs(integers, integers), [1, 2], 5000)).toBe(1);
    expect(positionOfPair(allPairs(integers, integers), [2, 1], 5000)).toBe(2);
    expect(positionOfPair(allPairs(integers, integers), [2, 3], 5000)).toBe(8);
    expect(positionOfPair(allPairs(integers, integers), [3, 2], 5000)).toBe(12);
  });

  it("differs from the module's pairs exactly by the mirrored pairs", () => {
    const modulePairs = pairs(integers, integers);
    expect(positionOfPair(modulePairs, [1, 2], 5000)).toBe(1);
    expect(positionOfPair(modulePairs, [2, 1], 5000)).toBe(-1);
    expect(streamTake(allPairs(integers, integers), 4)).not.toEqual(streamTake(modulePairs, 4));
  });
});
