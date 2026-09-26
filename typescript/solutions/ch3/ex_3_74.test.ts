// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { makeZeroCrossings, streamTake } from "../../packages/ch3/src/05-streams.js";

import { senseData, zeroCrossings } from "./ex_3_74.js";

describe("exercise 3.74: zero crossings via the generalized stream-map", () => {
  it("reproduces the book's transcript over the sense data", () => {
    expect(streamTake(senseData, 13)).toEqual([
      1, 2, 1.5, 1, 0.5, -0.1, -2, -3, -2, -0.5, 0.2, 3, 4,
    ]);
    expect(streamTake(zeroCrossings(senseData), 13)).toEqual([
      0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0,
    ]);
  });

  it("ends with the sense data: one output per input sample, none past the last", () => {
    expect(streamTake(zeroCrossings(senseData), 14)).toHaveLength(13);
  });

  it("is equivalent to Alyssa's recursive make-zero-crossings seeded at 0", () => {
    expect(streamTake(zeroCrossings(senseData), 13)).toEqual(
      streamTake(makeZeroCrossings(senseData, 0), 13),
    );
  });
});
