// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { renderedParses } from "./ex_4_48.js";

describe("exercise 4.48: adjectives in the grammar", () => {
  it("parses one sleepy cat exactly once", () => {
    expect(renderedParses('["the", "sleepy", "cat", "eats"]')).toStrictEqual([
      '["sentence", ["simple-noun-phrase", ["article", "the"], [["adjective", "sleepy"], ["noun", "cat"]]], ["verb", "eats"]]',
    ]);
  });

  it("parses the two-adjective dog exactly once", () => {
    expect(renderedParses('["the", "quick", "brown", "dog", "sleeps"]')).toStrictEqual([
      '["sentence", ["simple-noun-phrase", ["article", "the"], [["adjective", "quick"], [["adjective", "brown"], ["noun", "dog"]]]], ["verb", "sleeps"]]',
    ]);
  });
});
