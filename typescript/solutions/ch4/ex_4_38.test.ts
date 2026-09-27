// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { bruteForceCount, ex_4_38, solutions } from "./ex_4_38.js";

describe("exercise 4.38: dwelling without Smith-Fletcher", () => {
  it("answers five solutions in search order, search and brute force agree", async () => {
    expect(await Effect.runPromise(solutions(false))).toStrictEqual([
      "((baker 1) (cooper 2) (fletcher 4) (miller 3) (smith 5))",
      "((baker 1) (cooper 2) (fletcher 4) (miller 5) (smith 3))",
      "((baker 1) (cooper 4) (fletcher 2) (miller 5) (smith 3))",
      "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))",
      "((baker 3) (cooper 4) (fletcher 2) (miller 5) (smith 1))",
    ]);
    expect(bruteForceCount(false)).toBe(5);
  });

  it("the original clause restores the unique book answer", async () => {
    expect(await Effect.runPromise(solutions(true))).toStrictEqual([
      "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))",
    ]);
    expect(bruteForceCount(true)).toBe(1);
  });

  it("reports the count", () => {
    expect(ex_4_38()).toContain("grows from one to 5");
  });
});
