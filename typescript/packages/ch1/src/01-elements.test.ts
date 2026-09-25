// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: section 1.1

import { describe, expect, it } from "vitest";

import {
  abs,
  absElse,
  absIf,
  aPlusAbsB,
  circumference,
  f,
  goodEnough,
  greaterOrEqual,
  greaterOrEqualNot,
  newIf,
  p,
  pi,
  radius,
  size,
  sqrt,
  sqrtIterNewIf,
  sqrtLexical,
  sqrtNested,
  square,
  squareByLog,
  squareX,
  squareY,
  sumOfSquares,
  test,
} from "./01-elements.js";

/** The demo comparisons of exercise 1.6, typed as number comparisons. */
const eq = (a: number, b: number): boolean => a === b;

describe("the elements of programming", () => {
  it("the expression session evaluates to the transcript values", () => {
    expect(486).toBe(486);
    expect(137 + 349).toBe(486);
    expect(1000 - 334).toBe(666);
    expect(5 * 99).toBe(495);
    expect(10 / 5).toBe(2);
    expect(2.7 + 10).toBe(12.7);
  });

  it("chained operators cover the book's variadic combinations", () => {
    expect(21 + 35 + 12 + 7).toBe(75);
    expect(25 * 4 * 12).toBe(1200);
  });

  it("combinations nest without limit", () => {
    expect(3 * 5 + (10 - 6)).toBe(19);
    expect(3 * (2 * 4 + (3 + 5)) + (10 - 7 + 6)).toBe(57);
  });

  it("the tree of (2 + 4 * 6) * (3 + 5 + 7) percolates to 390", () => {
    expect((2 + 4 * 6) * (3 + 5 + 7)).toBe(390);
  });

  it("names refer to the values they are bound to", () => {
    expect(size).toBe(2);
    expect(5 * size).toBe(10);
    expect(pi * (radius * radius)).toBe(314.159);
    expect(circumference).toBe(62.8318);
  });

  it("compound procedures compose like primitives", () => {
    expect(square(21)).toBe(441);
    expect(square(2 + 5)).toBe(49);
    expect(square(square(3))).toBe(81);
    expect(sumOfSquares(3, 4)).toBe(25);
    expect(f(5)).toBe(136);
  });

  it("the three abs spellings agree at the case boundaries", () => {
    expect(abs(-3)).toBe(3);
    expect(abs(0)).toBe(0);
    expect(abs(3)).toBe(3);
    expect(absElse(-3)).toBe(abs(-3));
    expect(absElse(0)).toBe(abs(0));
    expect(absElse(3)).toBe(abs(3));
    expect(absIf(-3)).toBe(abs(-3));
    expect(absIf(0)).toBe(abs(0));
    expect(absIf(3)).toBe(abs(3));
  });

  it("the range test and the two >= spellings behave alike", () => {
    const x = 7;
    expect(x > 5 && x < 10).toBe(true);
    expect(greaterOrEqual(2, 1)).toBe(true);
    expect(greaterOrEqual(1, 1)).toBe(true);
    expect(greaterOrEqual(1, 2)).toBe(false);
    expect(greaterOrEqualNot(2, 1)).toBe(greaterOrEqual(2, 1));
    expect(greaterOrEqualNot(1, 1)).toBe(greaterOrEqual(1, 1));
    expect(greaterOrEqualNot(1, 2)).toBe(greaterOrEqual(1, 2));
  });

  it("exercise 1.4 picks the operation with a conditional", () => {
    expect(aPlusAbsB(3, 5)).toBe(8);
    expect(aPlusAbsB(3, -5)).toBe(8);
    expect(aPlusAbsB(2, 0)).toBe(2);
  });

  it("exercise 1.5: the eager host answers 0 and p diverges into a RangeError", () => {
    expect(test(0, 5)).toBe(0);
    expect(() => p()).toThrow(RangeError);
  });

  it("exercise 1.6: new-if works for value arguments, then diverges on sqrt", () => {
    expect(newIf(eq(2, 3), 0, 5)).toBe(5);
    expect(newIf(eq(1, 1), 0, 5)).toBe(0);
    expect(() => sqrtIterNewIf(1.0, 2)).toThrow(RangeError);
  });

  it("the sqrt transcript produces exactly the printed values", () => {
    expect(sqrt(9)).toBe(3.00009155413138);
    expect(sqrt(100 + 37)).toBe(11.704699917758145);
    expect(sqrt(sqrt(2) + sqrt(3))).toBe(1.7739279023207892);
    expect(square(sqrt(1000))).toBe(1000.000369924366);
  });

  it("good-enough? accepts guesses whose square is within 0.001", () => {
    expect(goodEnough(3.00009155413138, 9)).toBe(true);
    expect(goodEnough(3.0, 1000)).toBe(false);
  });

  it("renaming a parameter leaves the procedure the same", () => {
    expect(squareX(7)).toBe(49);
    expect(squareY(7)).toBe(squareX(7));
  });

  it("the two squares agree at printing precision", () => {
    expect(squareByLog(5)).toBeCloseTo(square(5), 10);
    expect(squareByLog(1.2)).toBe(1.44);
  });

  it("the nested and lexical sqrt programs reproduce the flat transcript", () => {
    expect(sqrtNested(9)).toBe(sqrt(9));
    expect(sqrtLexical(9)).toBe(sqrt(9));
    expect(sqrtLexical(sqrt(2) + sqrt(3))).toBe(1.7739279023207892);
  });
});
