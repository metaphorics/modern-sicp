// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { bridgeAnswers, emptyComparison, superviseeLines } from "./ex_4_78.js";

describe("exercise 4.78: the query language as a nondeterministic program", () => {
  it("bridges driver lines into search alternatives", () => {
    expect(superviseeLines()).toHaveLength(3);
    const hacker = bridgeAnswers("Hacker");
    expect(hacker).toStrictEqual([
      JSON.stringify('supervisor(["Hacker", "Alyssa", "P"], ["Bitdiddle", "Ben"])'),
    ]);
  });

  it("contrasts the driver's empty line with silent search failure", () => {
    const [driver, search] = emptyComparison();
    expect(driver).toStrictEqual(["No."]);
    expect(search).toBe(0);
  });
});
