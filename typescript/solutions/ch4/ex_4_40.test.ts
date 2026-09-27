// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { Effect } from "effect";
import { describe, expect, it } from "vitest";
import { assignmentCounts, ex_4_40, prunedSolutions } from "./ex_4_40.js";

describe("exercise 4.40: pruning before the restrictions", () => {
  it("counts the assignment sets before and after distinctness", () => {
    expect(assignmentCounts()).toStrictEqual([3125, 120]);
  });

  it("the pruned procedure answers the same unique solution", async () => {
    expect(await Effect.runPromise(prunedSolutions())).toStrictEqual([
      "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))",
    ]);
  });

  it("reports the measured counts", () => {
    expect(ex_4_40()).toContain("1835");
    expect(ex_4_40()).toContain("the pruned order 79");
  });
});
