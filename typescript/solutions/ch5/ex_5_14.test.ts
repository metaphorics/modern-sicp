// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { factorialMachineFactorial, factorialStackStatistics } from "./ex_5_14.js";

describe("exercise 5.14 measuring the factorial machine", () => {
  it("reports linear pushes and depth, two per recursive level", () => {
    expect(factorialStackStatistics()).toEqual([
      "n = 1: (total-pushes = 0 maximum-depth = 0)",
      "n = 2: (total-pushes = 2 maximum-depth = 2)",
      "n = 3: (total-pushes = 4 maximum-depth = 4)",
      "n = 4: (total-pushes = 6 maximum-depth = 6)",
      "n = 5: (total-pushes = 8 maximum-depth = 8)",
      "n = 6: (total-pushes = 10 maximum-depth = 10)",
      "the measured machine for n = 5 prints: (total-pushes = 8 maximum-depth = 8)",
    ]);
  });
  it("answers the host factorial", () => {
    expect(factorialMachineFactorial(5)).toBe(120);
  });
});
