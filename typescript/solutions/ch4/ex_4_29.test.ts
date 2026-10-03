// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { runMemoized, runRecompute } from "./ex_4_29.js";

describe("exercise 4.29: memoization speed difference", () => {
  it("memoized: 100 with count 1, then 1000 with count 2", () => {
    const result = runMemoized();
    expect(result.transcript).toEqual(["100", "1", "1000", "2"]);
  });

  it("unmemoized: 100 with count 2, then 1000 with count 5", () => {
    const result = runRecompute();
    expect(result.transcript).toEqual(["100", "2", "1000", "5"]);
  });

  it("both disciplines compute the same values", () => {
    const memoized = runMemoized().transcript;
    const recomputed = runRecompute().transcript;
    expect([memoized[0], memoized[2]]).toEqual([recomputed[0], recomputed[2]]);
  });
});
