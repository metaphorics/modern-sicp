// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { outrankedAnswers, recursiveFirstOutrankedBy } from "./ex_4_64.js";

describe("exercise 4.64: the outranked-by loop", () => {
  it("answers DeWitt's question under the terminating order", () => {
    const [ben] = outrankedAnswers();
    expect(ben).toStrictEqual(['outranked-by(["Bitdiddle", "Ben"], ["Warbucks", "Oliver"])']);
  });

  it("lists the whole outranking relation without looping", () => {
    const [, all] = outrankedAnswers();
    expect(all).toHaveLength(14);
    expect(all[0]).toBe('outranked-by(["Hacker", "Alyssa", "P"], ["Bitdiddle", "Ben"])');
    expect(all).toContain('outranked-by(["Cratchet", "Robert"], ["Warbucks", "Oliver"])');
  });

  it("exhibits the recursive-first rule without running it", () => {
    expect(recursiveFirstOutrankedBy.head.tag).toBe("atom");
  });
});
