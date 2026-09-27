// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import {
  ln2AcceleratedStream,
  ln2EulerStream,
  ln2Stream,
  streamLimitWithTerms,
} from "./ex_3_65.js";

describe("exercise 3.65: ln 2 approximation streams", () => {
  it("builds the alternating harmonic partial sums", () => {
    expect(streamTake(ln2Stream(), 8)).toEqual([
      1, 0.5, 0.8333333333333333, 0.5833333333333333, 0.7833333333333332, 0.6166666666666666,
      0.7595238095238095, 0.6345238095238095,
    ]);
  });

  it("euler-transforms them toward ln 2", () => {
    expect(streamTake(ln2EulerStream(ln2Stream()), 6)).toEqual([
      0.7, 0.6904761904761905, 0.6944444444444444, 0.6924242424242424, 0.6935897435897436,
      0.6928571428571428,
    ]);
  });

  it("accelerates to ln 2: element 4 is within 1e-5", () => {
    const accelerated = ln2AcceleratedStream(ln2Stream());
    expect(streamTake(accelerated, 6)).toEqual([
      1, 0.7, 0.6932773109243697, 0.6931488693329254, 0.6931471960735491, 0.6931471806635636,
    ]);
    expect(streamRef(accelerated, 4)).toBe(0.6931471960735491);
    expect(Math.abs(streamRef(accelerated, 4) - Math.LN2)).toBeLessThan(0.00001);
  });

  it("answers how rapidly each sequence converges at tolerance 0.0001", () => {
    expect(streamLimitWithTerms(ln2Stream(), 0.0001)).toEqual({
      value: 0.6930971830599583,
      terms: 10000,
    });
    expect(streamLimitWithTerms(ln2EulerStream(ln2Stream()), 0.0001)).toEqual({
      value: 0.6931879423258734,
      terms: 13,
    });
    expect(streamLimitWithTerms(ln2AcceleratedStream(ln2Stream()), 0.0001)).toEqual({
      value: 0.6931471960735491,
      terms: 5,
    });
  });
});
