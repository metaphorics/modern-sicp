// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import { newSyntaxRuns } from "./ex_5_10.js";

describe("exercise 5.10 a new syntax behind isolated syntax procedures", () => {
  it("expands the new forms to the book's instructions and runs them", () => {
    expect(newSyntaxRuns()).toEqual(["gcd(206, 40) in the new syntax = 2", "countdown(3) sum = 3"]);
  });
});
