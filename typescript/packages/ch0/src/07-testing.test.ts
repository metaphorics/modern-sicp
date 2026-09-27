// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import fc from "fast-check";
import { describe, expect, it } from "vitest";

import { absDiff, parity } from "./07-testing.js";

describe("testing", () => {
  it.each([
    [0, "even"],
    [1, "odd"],
    [42, "even"],
  ])("parity(%i) is %s", (n, expected) => {
    expect(parity(n)).toBe(expected);
  });

  it("a property test runs the assertion over generated inputs", () => {
    fc.assert(
      fc.property(
        fc.integer({ min: -100, max: 100 }),
        fc.integer({ min: -100, max: 100 }),
        (a, b) => {
          expect(absDiff(a, b)).toBe(absDiff(b, a));
        },
      ),
    );
  });

  it("boundary cases stay explicit", () => {
    expect(absDiff(0, 0)).toBe(0);
    expect(absDiff(Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER)).toBe(0);
  });
});
