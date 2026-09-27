// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { cappedResult, ex_4_47, louisFirst, swappedFirst } from "./ex_4_47.js";

describe("exercise 4.47: Louis's parse-verb-phrase", () => {
  it("answers the ordinary first parse, the capped copy runs dry after it", () => {
    expect(louisFirst()).toBe(
      "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))",
    );
    const capped = cappedResult();
    expect(capped.answers).toStrictEqual([
      "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))",
    ]);
    expect(capped.exhausted).toBe(true);
  });

  it("the interchanged order still answers the first parse", () => {
    expect(swappedFirst()).toBe(
      "(sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))",
    );
  });

  it("reports the divergence", () => {
    expect(ex_4_47()).toContain("endlessly");
  });
});
