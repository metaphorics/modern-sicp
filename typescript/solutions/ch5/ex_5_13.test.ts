// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { derivedRegisterRuns } from "./ex_5_13.js";

describe("exercise 5.13 registers derived from the controller", () => {
  it("derives the register list and runs the machine on it", () => {
    expect(derivedRegisterRuns()).toEqual([
      "derived registers: a b t",
      "gcd(206, 40) = 2",
      "allocated registers: a b t",
    ]);
  });
});
