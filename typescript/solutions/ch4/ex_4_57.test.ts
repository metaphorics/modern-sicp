// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_57, replacementAnswers } from "./ex_4_57.js";

describe("exercise 4.57: can-replace", () => {
  it("finds both of Cy's replacements but excludes Cy himself", () => {
    const [cyReplacements] = replacementAnswers();
    expect(cyReplacements).toStrictEqual([
      "(can-replace (Hacker Alyssa P) (Fect Cy D))",
      "(can-replace (Bitdiddle Ben) (Fect Cy D))",
    ]);
  });

  it("uses both salaries to select cheaper replacement candidates", () => {
    const [, lowerPaid] = replacementAnswers();
    expect(lowerPaid.length).toBeGreaterThan(0);
    expect(lowerPaid.every((answer) => answer.includes("?salary"))).toBe(false);
    expect(ex_4_57()).toContain("excludes self-replacement");
  });
});
