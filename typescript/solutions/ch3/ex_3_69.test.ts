// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { integers, streamTake } from "../../packages/ch3/src/05-streams.js";

import { pythagoreanTriples, triples } from "./ex_3_69.js";

describe("exercise 3.69: triples and the Pythagorean triples", () => {
  it("starts triples at (1, 1, 1) and covers the first row and the plane", () => {
    expect(streamTake(triples(integers, integers, integers), 8)).toEqual([
      [1, 1, 1],
      [1, 2, 2],
      [2, 2, 2],
      [1, 2, 3],
      [2, 3, 3],
      [1, 3, 3],
      [3, 3, 3],
      [1, 2, 4],
    ]);
  });

  it("pins the first five Pythagorean triples", () => {
    expect(streamTake(pythagoreanTriples, 5)).toEqual([
      [3, 4, 5],
      [6, 8, 10],
      [5, 12, 13],
      [9, 12, 15],
      [8, 15, 17],
    ]);
  });

  it("every pinned triple is ordered i <= j <= k and satisfies i^2 + j^2 = k^2", () => {
    const pinned = streamTake(pythagoreanTriples, 5);
    expect(pinned.length).toBe(5);
    for (const [i, j, k] of pinned) {
      expect(i).toBeLessThanOrEqual(j);
      expect(j).toBeLessThanOrEqual(k);
      expect(i * i + j * j).toBe(k * k);
    }
  });
});
