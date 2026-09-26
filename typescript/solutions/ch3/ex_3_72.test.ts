// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { threeSquareRepresentations } from "./ex_3_72.js";

describe("exercise 3.72: sums of two squares in three ways", () => {
  it("pins the first four numbers with three representations as sums of two squares", () => {
    expect(streamTake(threeSquareRepresentations(), 4)).toEqual([
      [325, [1, 18], [6, 17], [10, 15]],
      [425, [5, 20], [8, 19], [13, 16]],
      [650, [5, 25], [11, 23], [17, 19]],
      [725, [7, 26], [10, 25], [14, 23]],
    ]);
  });

  it("each hit's weight is the square sum of all three representations", () => {
    for (const [weight, first, second, third] of streamTake(threeSquareRepresentations(), 4)) {
      for (const [i, j] of [first, second, third]) {
        expect(i * i + j * j).toBe(weight);
        expect(i).toBeLessThanOrEqual(j);
      }
    }
  });

  it("the hits strictly increase", () => {
    let previous: number | undefined;
    for (const [weight] of streamTake(threeSquareRepresentations(), 4)) {
      if (previous !== undefined) {
        expect(weight).toBeGreaterThan(previous);
      }
      previous = weight;
    }
  });
});
