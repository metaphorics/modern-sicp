// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { memoFib, memoFibTrace } from "./ex_3_27.js";

describe("exercise 3.27: memo-fib", () => {
  it("answers the n-th Fibonacci number, each value computed once", () => {
    expect(memoFib(0)).toBe(0);
    expect(memoFib(1)).toBe(1);
    expect(memoFib(10)).toBe(55);
    expect(memoFib(30)).toBe(832040);
  });

  it("the trace for n = 4 shows the spine, the two base computes, and the two recalls", () => {
    expect(memoFibTrace(4)).toEqual([
      { _tag: "Compute", n: 4 },
      { _tag: "Compute", n: 3 },
      { _tag: "Compute", n: 2 },
      { _tag: "Compute", n: 1 },
      { _tag: "Compute", n: 0 },
      { _tag: "Recall", n: 1 },
      { _tag: "Recall", n: 2 },
    ]);
  });

  it("each fib value 0..4 was computed exactly once", () => {
    const computes = memoFibTrace(4)
      .filter((event) => event._tag === "Compute")
      .map((event) => event.n);
    expect(computes).toEqual([4, 3, 2, 1, 0]);
  });

  it("a fresh call builds a fresh table and walks the same trace", () => {
    expect(memoFibTrace(4)).toEqual(memoFibTrace(4));
  });
});
