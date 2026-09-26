// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_43, solutions } from "./ex_4_43.js";

describe("exercise 4.43: the yacht puzzle", () => {
  it("told Mary Ann is a Moore: one solution, Downing", { timeout: 60000 }, async () => {
    expect(await Effect.runPromise(solutions(true))).toStrictEqual(["(lornas-father downing)"]);
  });

  it("not told: Downing and Parker both work", { timeout: 60000 }, async () => {
    expect(await Effect.runPromise(solutions(false))).toStrictEqual([
      "(lornas-father downing)",
      "(lornas-father parker)",
    ]);
  });

  it("reports both", { timeout: 120000 }, () => {
    expect(ex_4_43()).toContain("Colonel Downing");
  });
});
