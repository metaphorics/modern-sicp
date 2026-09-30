// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  integers,
  streamEnumerateInterval,
  streamRef,
  streamTake,
} from "../../packages/ch3/src/05-streams.js";

import { addStreamsN, integersThroughMapN, streamMapN } from "./ex_3_50.js";

describe("exercise 3.50: the multi-stream streamMap", () => {
  it("adds two streams element-wise like the text's addStreams", () => {
    expect(streamTake(addStreamsN(integers, integers), 6)).toEqual([2, 4, 6, 8, 10, 12]);
  });

  it("zips three streams at once and stops at the shortest", () => {
    const sums = streamMapN(
      (a, b, c) => a + b + c,
      streamEnumerateInterval(1, 100),
      streamEnumerateInterval(10, 100),
      streamEnumerateInterval(100, 103),
    );
    expect(streamTake(sums, 4)).toEqual([111, 114, 117, 120]);
  });

  it("answers the empty stream when any input stream is exhausted", () => {
    const sums = streamMapN(
      (a, b) => a + b,
      streamEnumerateInterval(1, 2),
      streamEnumerateInterval(1, 5),
    );
    expect(streamTake(sums, 5)).toEqual([2, 4]);
  });

  it("answers the empty stream when there are no streams at all", () => {
    expect(streamMapN((...xs: number[]) => xs.length)).toBeNull();
  });

  it("regenerates the integers as the ones plus the integers", () => {
    expect(streamRef(integersThroughMapN, 9)).toBe(10);
    expect(streamRef(integers, 9)).toBe(10);
  });
});
