// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { runGcd, simulatedExptRuns } from "./ex_5_07.js";

describe("exercise 5.7 designed machines on the simulator", () => {
  it("runs the 5.4 expt machines against the host oracle", () => {
    expect(simulatedExptRuns()).toEqual([
      "recursive expt(2, 10) = 1024 (host 1024)",
      "recursive expt(3, 5) = 243 (host 243)",
      "iterative expt(2, 10) = 1024 (host 1024)",
      "iterative expt(3, 5) = 243 (host 243)",
    ]);
  });
  it("answers gcd(206, 40) = 2 on the book's own example machine", () => {
    expect(runGcd(206, 40)).toBe(2);
  });
});
