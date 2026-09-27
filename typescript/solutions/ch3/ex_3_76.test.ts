// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  consStream,
  type Stream,
  signChangeDetector,
  streamEnumerateInterval,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { smooth, zeroCrossings } from "./ex_3_76.js";

const streamOf = (xs: readonly number[]): Stream<number> =>
  xs.reduceRight<Stream<number>>((rest, head) => consStream(head, () => rest), null);

describe("exercise 3.76: smooth as a reusable component", () => {
  it("averages successive elements and answers one element fewer on a finite stream", () => {
    expect(streamTake(smooth(streamEnumerateInterval(1, 5)), 6)).toEqual([1.5, 2.5, 3.5, 4.5]);
  });

  it("detects the crossings of the smoothed sense data", () => {
    const sense = [1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4];
    expect(streamTake(zeroCrossings(streamOf(sense)), 12)).toEqual([
      0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0,
    ]);
  });

  it("matches the corrected 3.75 detector minus its two boundary elements", () => {
    // The exercise 3.75 fix restated over plain arrays so the
    // comparison stays inside this file: smoothed[k] pairs each
    // sample with its predecessor (seeded at 0), and the detector
    // reads consecutive smoothed points.
    const samples = [1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4];
    const smoothed = samples.map((x, i) => (x + (samples[i - 1] ?? 0)) / 2);
    const crossings = smoothed
      .slice(1)
      .map((current, i) => signChangeDetector(current, smoothed[i] ?? 0));
    expect(streamTake(zeroCrossings(streamOf(samples)), 12)).toEqual(crossings.slice(1));
  });
});
