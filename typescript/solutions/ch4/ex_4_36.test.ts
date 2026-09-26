// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_36, firstTriples } from "./ex_4_36.js";

describe("exercise 4.36: unbounded Pythagorean triples", () => {
  it("enumerates by hypotenuse, so all triples of k precede k + 1", async () => {
    expect(await Effect.runPromise(firstTriples(6))).toStrictEqual([
      "(3 4 5)",
      "(6 8 10)",
      "(5 12 13)",
      "(9 12 15)",
      "(8 15 17)",
      "(12 16 20)",
    ]);
  });

  it("reports the fairness argument", () => {
    expect(ex_4_36()).toContain("never exhausts");
  });
});
