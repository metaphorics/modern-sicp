// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { expmodExplicit, expmodExplicitCalls, expmodSquareCalls } from "./ex_1_26.js";

describe("exercise 1.26", () => {
  it("both expmods still agree on the value", () => {
    expect(expmodExplicit(2, 1024, 561)).toBe(Number(2n ** 1024n % 561n));
  });

  it("the square version makes a logarithmic number of calls", () => {
    expect(expmodSquareCalls(1024)).toBe(12);
    expect(expmodSquareCalls(2048)).toBe(13);
  });

  it("Louis's version makes a linear number of calls", () => {
    expect(expmodExplicitCalls(1024)).toBe(3071);
    expect(expmodExplicitCalls(2048)).toBe(6143);
  });
});
