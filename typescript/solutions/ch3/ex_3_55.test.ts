// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import {
  integers,
  ones,
  partialSums,
  piSummands,
  streamTake,
  theEmptyStream,
} from "../../packages/ch3/src/05-streams.js";

import { partialSumsEx } from "./ex_3_55.js";

describe("exercise 3.55: partial-sums", () => {
  it("sums the integers into 1 3 6 10 15 21 as the statement shows", () => {
    expect(streamTake(partialSumsEx(integers), 6)).toEqual([1, 3, 6, 10, 15, 21]);
  });

  it("adds up the ones as 1 2 3 4 ...", () => {
    expect(streamTake(partialSumsEx(ones), 6)).toEqual([1, 2, 3, 4, 5, 6]);
  });

  it("agrees element-wise with the section's own partialSums", () => {
    expect(streamTake(partialSumsEx(integers), 30)).toEqual(streamTake(partialSums(integers), 30));
    expect(streamTake(partialSumsEx(piSummands(1)), 20)).toEqual(
      streamTake(partialSums(piSummands(1)), 20),
    );
  });

  it("answers the empty stream for the empty stream", () => {
    expect(partialSumsEx(theEmptyStream)).toBeNull();
  });
});
