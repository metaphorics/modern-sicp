// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { ex_4_64, loopExplanation, recursiveFirstOutrankedBy } from "./ex_4_64.js";

describe("exercise 4.64: recursive-first outranked-by", () => {
  it("shows recursion before the constraining supervisor clause", () => {
    expect(recursiveFirstOutrankedBy.indexOf("(outranked-by ?middle-manager ?boss)")).toBeLessThan(
      recursiveFirstOutrankedBy.indexOf("(supervisor ?staff-person ?middle-manager)"),
    );
    expect(loopExplanation()).toContain("fresh, unconstrained staff person");
  });

  it("describes why the recursive clause does not terminate", () => {
    expect(ex_4_64()).toContain("indefinitely");
  });
});
