// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { ramanujanNumbers } from "./ex_3_71.js";

describe("exercise 3.71: Ramanujan numbers via weighted pairs", () => {
  it("pins the first six numbers with two representations as sums of two cubes", () => {
    expect(streamTake(ramanujanNumbers(), 6)).toEqual([
      [1729, [1, 12], [9, 10]],
      [4104, [2, 16], [9, 15]],
      [13832, [2, 24], [18, 20]],
      [20683, [10, 27], [19, 24]],
      [32832, [4, 32], [18, 30]],
      [39312, [2, 34], [15, 33]],
    ]);
  });

  it("each hit's weight is the cube sum of both representations", () => {
    for (const [weight, first, second] of streamTake(ramanujanNumbers(), 6)) {
      expect(first[0] ** 3 + first[1] ** 3).toBe(weight);
      expect(second[0] ** 3 + second[1] ** 3).toBe(weight);
      expect(first[0]).toBeLessThanOrEqual(first[1]);
      expect(second[0]).toBeLessThanOrEqual(second[1]);
    }
  });

  it("the hits strictly increase, so consecutive equal weights are paired up", () => {
    let previous: number | undefined;
    for (const [weight] of streamTake(ramanujanNumbers(), 6)) {
      if (previous !== undefined) {
        expect(weight).toBeGreaterThan(previous);
      }
      previous = weight;
    }
  });
});
