// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { carPair, cdrPair, consPair } from "./ex_2_04.js";

describe("exercise 2.4", () => {
  it("car(cons(x, y)) yields x and cdr(cons(x, y)) yields y", () => {
    expect(carPair(consPair(1, 2))).toBe(1);
    expect(cdrPair(consPair(1, 2))).toBe(2);
  });

  it("the law holds for arbitrary object parts, by identity not equality", () => {
    const left = { a: 1 };
    const right = { b: 2 };
    expect(carPair(consPair(left, right))).toBe(left);
    expect(cdrPair(consPair(left, right))).toBe(right);
  });

  it("the two parts may have different types, one from each side", () => {
    const z = consPair("k", 7);
    expect(carPair(z)).toBe("k");
    expect(cdrPair(z)).toBe(7);
  });
});
