// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  consStream,
  type Stream,
  streamCdr,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { mergeWeighted, pairsBySum, pairsByWeightedSum, type Weight } from "./ex_3_70.js";

/** A finite stream from an array, for merge-weighted unit checks. */
const streamFromArray = <A>(items: A[]): Stream<A> =>
  items.reduceRight<Stream<A>>((rest, item) => consStream(item, () => rest), null);

describe("exercise 3.70: merge-weighted and weighted-pairs", () => {
  it("merge-weighted keeps both elements of an equal-weight tie", () => {
    const weight: Weight = ([i, j]) => i + j;
    const merged = mergeWeighted(
      streamFromArray([
        [1, 5],
        [1, 6],
      ]),
      streamFromArray([
        [2, 4],
        [2, 5],
      ]),
      weight,
    );
    expect(streamTake(merged, 4)).toEqual([
      [1, 5],
      [2, 4],
      [1, 6],
      [2, 5],
    ]);
  });

  it("pins the first ten pairs ordered by the sum i + j", () => {
    expect(streamTake(pairsBySum, 10)).toEqual([
      [1, 1],
      [1, 2],
      [1, 3],
      [2, 2],
      [1, 4],
      [2, 3],
      [1, 5],
      [2, 4],
      [3, 3],
      [1, 6],
    ]);
  });

  it("pins the first ten pairs over integers with no divisor 2, 3, or 5, by 2i + 3j + 5ij", () => {
    expect(streamTake(pairsByWeightedSum, 10)).toEqual([
      [1, 1],
      [1, 7],
      [1, 11],
      [1, 13],
      [1, 17],
      [1, 19],
      [1, 23],
      [1, 29],
      [1, 31],
      [7, 7],
    ]);
  });

  it("both streams are nondecreasing in their stated weights", () => {
    const weightOf = (pair: [number, number]): number => pair[0] + pair[1];
    const weightedSum = ([i, j]: [number, number]): number => 2 * i + 3 * j + 5 * i * j;
    for (const weights of [
      streamTake(pairsBySum, 40).map(weightOf),
      streamTake(pairsByWeightedSum, 40).map(weightedSum),
    ]) {
      let previous: number | undefined;
      for (const weight of weights) {
        if (previous !== undefined) {
          expect(weight).toBeGreaterThanOrEqual(previous);
        }
        previous = weight;
      }
    }
  });

  it("stream (b) never pairs a number divisible by 2, 3, or 5", () => {
    for (const [i, j] of streamTake(streamCdr(pairsByWeightedSum), 30)) {
      expect(i % 2).not.toBe(0);
      expect(i % 3).not.toBe(0);
      expect(i % 5).not.toBe(0);
      expect(j % 2).not.toBe(0);
      expect(j % 3).not.toBe(0);
      expect(j % 5).not.toBe(0);
    }
  });
});
