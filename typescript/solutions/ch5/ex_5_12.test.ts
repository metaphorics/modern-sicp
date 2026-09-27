// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { gcdMachineSummary } from "./ex_5_12.js";

describe("exercise 5.12 the assembler's summary", () => {
  it("summarizes the gcd machine's instructions, registers, and labels", () => {
    expect(gcdMachineSummary()).toBe(
      "(instructions (test (test (op =) (reg b) (const 0))) (branch (branch (label gcd-done)))" +
        " (assign (assign a (reg b)) (assign b (reg t)) (assign t (op rem) (reg a) (reg b)))" +
        " (goto (goto (label test-b))))\n" +
        "(registers a b t)\n" +
        "(entry-point registers )\n" +
        "(stack registers )\n" +
        "(sources (t ((op rem) (reg a) (reg b))) (a (reg b)) (b (reg t)))\n" +
        "(labels test-b gcd-done)",
    );
  });
});
