// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { pascal } from "./ex_1_12.js";

describe("exercise 1.12", () => {
  it("the edges are all 1", () => {
    for (let row = 0; row <= 8; row += 1) {
      expect(pascal(row, 0)).toBe(1);
      expect(pascal(row, row)).toBe(1);
    }
  });

  it("the printed rows match the triangle", () => {
    expect(pascal(4, 2)).toBe(6);
    expect([0, 1, 2, 3, 4].map((col) => pascal(4, col))).toStrictEqual([1, 4, 6, 4, 1]);
    expect(pascal(6, 3)).toBe(20);
    expect(pascal(10, 5)).toBe(252);
  });

  it("each inside element is the sum of the two above it", () => {
    for (let row = 2; row <= 12; row += 1) {
      for (let col = 1; col < row; col += 1) {
        expect(pascal(row, col)).toBe(pascal(row - 1, col - 1) + pascal(row - 1, col));
      }
    }
  });
});
