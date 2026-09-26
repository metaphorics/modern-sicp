// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { labelTracedGcdTrace } from "./ex_5_17.js";

describe("exercise 5.17 labels in the trace", () => {
  it("names every traced line by the label in effect", () => {
    const trace = labelTracedGcdTrace();
    expect(trace).toHaveLength(26);
    expect(trace[0]).toBe("test-b: (test (op =) (reg b) (const 0))");
    expect(trace[1]).toBe("test-b: (branch (label gcd-done))");
    expect(trace[2]).toBe("test-b: (assign t (op rem) (reg a) (reg b))");
    expect(trace[3]).toBe("test-b: (assign a (reg b))");
    expect(trace[4]).toBe("test-b: (assign b (reg t))");
    expect(trace[5]).toBe("test-b: (goto (label test-b))");
    expect(trace.filter((line) => line.startsWith("gcd-done:"))).toEqual([]);
    expect(trace.every((line) => line.startsWith("test-b:"))).toBe(true);
  });
});
