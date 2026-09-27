// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { ex_4_35, triplesBetween } from "./ex_4_35.js";

describe("exercise 4.35: an-integer-between and triples", () => {
  it("enumerates the triples between 1 and 20 in search order", async () => {
    expect(await Effect.runPromise(triplesBetween(1, 20))).toStrictEqual([
      "(3 4 5)",
      "(5 12 13)",
      "(6 8 10)",
      "(8 15 17)",
      "(9 12 15)",
      "(12 16 20)",
    ]);
  });

  it("finds nothing above the known range", async () => {
    expect(await Effect.runPromise(triplesBetween(13, 20))).toStrictEqual([]);
  });

  it("reports the order", () => {
    expect(ex_4_35()).toContain("property of the search");
  });
});
