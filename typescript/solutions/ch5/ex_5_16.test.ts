// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { tracedGcdTrace, untracedRunTranscript } from "./ex_5_16.js";

const tracedInstruction = [
  "(test (op =) (reg b) (const 0))",
  "(branch (label gcd-done))",
  "(assign t (op rem) (reg a) (reg b))",
  "(assign a (reg b))",
  "(assign b (reg t))",
  "(goto (label test-b))",
];

describe("exercise 5.16 instruction tracing", () => {
  it("lists every executed instruction, ending at the taken branch", () => {
    const trace = tracedGcdTrace();
    expect(trace).toHaveLength(26);
    expect(trace.slice(0, 6)).toEqual(tracedInstruction);
    expect(trace.slice(-2)).toEqual(tracedInstruction.slice(0, 2));
    expect(trace.filter((line) => line.startsWith("(test"))).toHaveLength(5);
    expect(trace.filter((line) => line.startsWith("(branch"))).toHaveLength(5);
  });
  it("leaves no trace lines when the switch is off", () => {
    expect(untracedRunTranscript()).toEqual([]);
  });
});
