// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { consStream, type Stream, streamTake } from "../../packages/ch3/src/05-streams.js";

import { louisZeroCrossings, zeroCrossingsSmoothed } from "./ex_3_75.js";

const streamOf = (xs: readonly number[]): Stream<number> =>
  xs.reduceRight<Stream<number>>((rest, head) => consStream(head, () => rest), null);

describe("exercise 3.75: Louis's buggy smoothing detector", () => {
  it("on the book's clean sense data Louis accidentally agrees with the fix", () => {
    const sense = [1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4];
    expect(streamTake(louisZeroCrossings(streamOf(sense), 0), 13)).toEqual([
      0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0,
    ]);
    expect(streamTake(zeroCrossingsSmoothed(streamOf(sense)), 13)).toEqual([
      0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0,
    ]);
  });

  it("prefixed with oscillation, Louis reports crossings the smoothing should suppress", () => {
    const noisy = [1, -1, 1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4];
    expect(streamTake(louisZeroCrossings(streamOf(noisy), 0), 15)).toEqual([
      0, -1, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0,
    ]);
    expect(streamTake(zeroCrossingsSmoothed(streamOf(noisy)), 15)).toEqual([
      0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0,
    ]);
  });

  it("on a slow drift with one dip, Louis misses crossings the fix catches", () => {
    const drifting = [4, 3, 2, -2.1, 2, 2.5];
    expect(streamTake(louisZeroCrossings(streamOf(drifting), 0), 6)).toEqual([0, 0, 0, 0, 0, 0]);
    expect(streamTake(zeroCrossingsSmoothed(streamOf(drifting)), 6)).toEqual([0, 0, 0, -1, 0, 1]);
  });
});
