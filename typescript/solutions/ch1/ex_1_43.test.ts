// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { repeated } from "./ex_1_43.js";

const square = (x: number): number => x * x;

describe("exercise 1.43", () => {
  it("repeated(square, 2) is square composed with itself", () => {
    expect(repeated(square, 2)(5)).toBe(625);
  });

  it("repeated(square, 5)(2) is the 32nd power of 2", () => {
    expect(repeated(square, 5)(2)).toBe(4294967296);
  });

  it("repeated(inc, 100)(0) applies the increment a hundred times", () => {
    expect(repeated((x: number): number => x + 1, 100)(0)).toBe(100);
  });

  it("n = 1 returns f itself", () => {
    expect(repeated(square, 1)(7)).toBe(49);
  });

  it("the composed function applies the innermost f first", () => {
    // f squares then adds one, applied twice to 3: 101
    expect(repeated((x: number): number => x * x + 1, 2)(3)).toBe(101);
  });
});
