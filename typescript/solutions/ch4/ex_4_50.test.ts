// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { renderedSamples, seed } from "./ex_4_50.js";

describe("exercise 4.50: ramb", () => {
  it("reproduces one order per seed", () => {
    const first = renderedSamples(seed, 6);
    const second = renderedSamples(seed, 6);
    expect(first).toHaveLength(6);
    expect(second).toStrictEqual(first);
  });

  it("samples well-formed sentences, not bare words", () => {
    const samples = renderedSamples(seed, 6);
    for (const line of samples) {
      expect(line.startsWith('["sentence"')).toBe(true);
      expect(line).toContain("simple-noun-phrase");
    }
  });
});
