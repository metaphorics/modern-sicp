// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { integers, streamRef, streamTake } from "../../packages/ch3/src/05-streams.js";

import { factorials, mulStreams } from "./ex_3_54.js";

describe("exercise 3.54: mul-streams and the factorials", () => {
  it("multiplies two streams element-wise", () => {
    expect(streamTake(mulStreams(integers, integers), 5)).toEqual([1, 4, 9, 16, 25]);
  });

  it("answers the statement's definition: element n counting from 0 is (n+1)!", () => {
    expect(streamTake(factorials, 10)).toEqual([
      1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800,
    ]);
  });

  it("stays exact deep in the self-referential definition", () => {
    expect(streamRef(factorials, 15)).toBe(20922789888000);
  });
});
