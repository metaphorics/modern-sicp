// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { compose } from "./ex_1_42.js";

const inc = (x: number): number => x + 1;
const square = (x: number): number => x * x;

describe("exercise 1.42", () => {
  it("compose(square, inc)(6) is 49", () => {
    expect(compose(square, inc)(6)).toBe(49);
  });

  it("the argument order matters", () => {
    expect(compose(inc, square)(6)).toBe(37);
  });

  it("the identity is neutral on both sides", () => {
    expect(compose(square, (x: number): number => x)(3)).toBe(9);
    expect(compose((x: number): number => x, square)(3)).toBe(9);
  });
});
