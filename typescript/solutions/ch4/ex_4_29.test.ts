// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";

import { answers, ex_4_29 } from "./ex_4_29.js";

describe("exercise 4.29: memoization speed difference", () => {
  it("pins both disciplines on the book's interaction", async () => {
    const observed = await Effect.runPromise(answers());
    expect(observed.memoized).toStrictEqual(["100", "1", "1000", "2"]);
    expect(observed.unmemoized).toStrictEqual(["100", "2", "1000", "5"]);
  });

  it("explains the counts as demand sites", () => {
    const report = ex_4_29();
    expect(report).toContain("100");
    expect(report).toContain("demand site");
  });
});
