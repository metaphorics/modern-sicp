// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { double, s } from "./ex_3_53.js";

describe("exercise 3.53: the self-referential doubling stream", () => {
  it("answers the prediction: the powers of 2", () => {
    expect(streamTake(s, 8)).toEqual([1, 2, 4, 8, 16, 32, 64, 128]);
  });

  it("keeps doubling beyond the visible prefix", () => {
    expect(streamRef(s, 12)).toBe(4096);
    expect(streamRef(s, 20)).toBe(1048576);
  });

  it("the scale-stream construction agrees with the statement's add-streams", () => {
    expect(streamTake(double, 8)).toEqual(streamTake(s, 8));
    expect(streamRef(double, 20)).toBe(streamRef(s, 20));
  });
});
