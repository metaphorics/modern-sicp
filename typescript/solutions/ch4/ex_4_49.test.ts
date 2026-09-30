// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { renderedSentences } from "./ex_4_49.js";

const first =
  '["sentence", ["simple-noun-phrase", ["article", "the"], ["noun", "student"]], ["verb", "studies"]]';
const sixth =
  '["sentence", ["simple-noun-phrase", ["article", "the"], ["noun", "professor"]], ["verb", "lectures"]]';

describe("exercise 4.49: Alyssa's generation", () => {
  it("generates the first half-dozen sentences in search order", () => {
    const six = renderedSentences(6);
    expect(six).toHaveLength(6);
    expect(six[0]).toBe(first);
    expect(six[5]).toBe(sixth);
  });

  it("cycles verbs fastest under fixed article and noun", () => {
    const six = renderedSentences(6);
    expect(six[1]).toContain('["verb", "lectures"]');
    expect(six[4]).toContain('["noun", "professor"]');
  });
});
