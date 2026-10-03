// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { nearAnswers } from "./ex_4_60.js";

describe("exercise 4.60: lives-near duplicates", () => {
  it("finds the one person sharing Alyssa's town", () => {
    const [alyssa] = nearAnswers();
    expect(alyssa).toStrictEqual(['lives-near(["Fect", "Cy", "D"], ["Hacker", "Alyssa", "P"])']);
  });

  it("lists every directed pair, each unordered pair twice", () => {
    const [, pairs] = nearAnswers();
    expect(pairs).toHaveLength(8);
    expect(pairs[0]).toBe('lives-near(["Bitdiddle", "Ben"], ["Reasoner", "Louis"])');
    expect(pairs).toContain('lives-near(["Hacker", "Alyssa", "P"], ["Fect", "Cy", "D"])');
    expect(pairs).toContain('lives-near(["Fect", "Cy", "D"], ["Hacker", "Alyssa", "P"])');
  });

  it("dedupes to one direction per pair by name order", () => {
    const [, , deduped] = nearAnswers();
    expect(deduped).toStrictEqual([
      'and(lives-near(["Bitdiddle", "Ben"], ["Reasoner", "Louis"]), lisp-value(["Bitdiddle", "Ben"], ["Reasoner", "Louis"]))',
      'and(lives-near(["Fect", "Cy", "D"], ["Hacker", "Alyssa", "P"]), lisp-value(["Fect", "Cy", "D"], ["Hacker", "Alyssa", "P"]))',
      'and(lives-near(["Aull", "DeWitt"], ["Bitdiddle", "Ben"]), lisp-value(["Aull", "DeWitt"], ["Bitdiddle", "Ben"]))',
      'and(lives-near(["Aull", "DeWitt"], ["Reasoner", "Louis"]), lisp-value(["Aull", "DeWitt"], ["Reasoner", "Louis"]))',
    ]);
  });
});
