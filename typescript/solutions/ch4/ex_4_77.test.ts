// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { filterAnswers } from "./ex_4_77.js";

describe("exercise 4.77: delayed filtering for not", () => {
  it("keeps the non-programmer under the bound filter", () => {
    const [bound] = filterAnswers();
    expect(bound).toStrictEqual([
      'and(supervisor(["Tweakit", "Lem", "E"], ["Bitdiddle", "Ben"]), not(job(["Tweakit", "Lem", "E"], ["computer", "programmer"])))',
    ]);
  });

  it("kills every frame under the open filter", () => {
    const [, open] = filterAnswers();
    expect(open).toStrictEqual(["No."]);
  });

  it("addition 4.77a: partial grounding still over-filters", () => {
    const [, , partial] = filterAnswers();
    expect(partial).toStrictEqual(["No."]);
  });
});
