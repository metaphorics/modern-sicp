// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { countLeavesRuns } from "./ex_5_21.js";

describe("exercise 5.21 count-leaves machines", () => {
  it("both machines agree with the host definition, with recursion using more stack", () => {
    expect(countLeavesRuns()).toEqual([
      "(1 2 (3 (4 5))): recursive n5, iterative n5, oracle 5; " +
        "recursive stack (total-pushes = 21 maximum-depth = 14), " +
        "iterative stack (total-pushes = 14 maximum-depth = 10)",
      "((7)): recursive n1, iterative n1, oracle 1; " +
        "recursive stack (total-pushes = 6 maximum-depth = 4), " +
        "iterative stack (total-pushes = 4 maximum-depth = 4)",
      "(): recursive n0, iterative n0, oracle 0; " +
        "recursive stack (total-pushes = 0 maximum-depth = 0), " +
        "iterative stack (total-pushes = 0 maximum-depth = 0)",
    ]);
  });
});
