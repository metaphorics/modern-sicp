// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { failureCounts, naiveSolutions, prunedSolutions } from "./ex_4_40.js";

describe("exercise 4.40: pruning before the restrictions", () => {
  it("both procedures answer the same single assignment", () => {
    const answer = ["{ baker: 3, cooper: 2, fletcher: 4, miller: 5, smith: 1 }"];
    expect(naiveSolutions()).toStrictEqual(answer);
    expect(prunedSolutions()).toStrictEqual(answer);
  });

  it("measures fewer deferred backtracks when restrictions prune choices", () => {
    const [naive, pruned] = failureCounts();
    expect(naive).toBeGreaterThan(pruned);
  });
});
