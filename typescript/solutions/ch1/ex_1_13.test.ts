// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { closedFormFib, fibDefinition } from "./ex_1_13.js";

describe("exercise 1.13", () => {
  it("the closed form gives Fib(10) = 55", () => {
    expect(closedFormFib(10)).toBe(55);
  });

  it("the closed form rounds to the definition for every n up to 40", () => {
    for (let n = 0; n <= 40; n += 1) {
      expect(closedFormFib(n)).toBe(fibDefinition(n));
    }
  });
});
