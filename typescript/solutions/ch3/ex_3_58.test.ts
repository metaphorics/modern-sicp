// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { streamTake } from "../../packages/ch3/src/05-streams.js";

import { expand } from "./ex_3_58.js";

describe("exercise 3.58: expand's long-division digits", () => {
  it("expands 1/7 in base 10 into the repeating digits of 0.142857...", () => {
    expect(streamTake(expand(1, 7, 10), 8)).toEqual([1, 4, 2, 8, 5, 7, 1, 4]);
  });

  it("expands 3/8 in base 10 into 0.375 followed by zeros", () => {
    expect(streamTake(expand(3, 8, 10), 8)).toEqual([3, 7, 5, 0, 0, 0, 0, 0]);
  });

  it("expands 1/3 in base 2 into 0.0101..., honoring the radix argument", () => {
    expect(streamTake(expand(1, 3, 2), 8)).toEqual([0, 1, 0, 1, 0, 1, 0, 1]);
  });
});
