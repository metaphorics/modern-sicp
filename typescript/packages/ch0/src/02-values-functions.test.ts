// SPDX-License-Identifier: GPL-3.0-only
// Adapted-from-SICP: Chapter 0 section 0.2

import { describe, expect, it } from "vitest";

import {
  classify,
  factorialBig,
  halfOf,
  hypotenuseSquared,
  makeAdder,
  planck,
  realHalf,
  sign,
  size,
  square,
  sumOver,
  twice,
} from "./02-values-functions.js";

describe("values, names, and functions", () => {
  it("bindings hold literals of each kind", () => {
    expect(planck).toBe(6.626_070_15e-34);
    expect(size).toBe(2);
    expect(5 + 3 + size).toBe(10);
  });

  it("the square sessions return the prose values", () => {
    expect(square(21)).toBe(441);
    expect(square(2 + 5)).toBe(49);
    expect(square(square(3))).toBe(81);
  });

  it("a block body names its intermediate steps", () => {
    expect(hypotenuseSquared(3, 4)).toBe(25);
  });

  it("a closure builds a family of functions", () => {
    const add5 = makeAdder(5);
    const add12 = makeAdder(12);
    expect(add5(1)).toBe(6);
    expect(add12(1)).toBe(13);
  });

  it("procedures are values passed and returned", () => {
    expect(sumOver(square, 1, 3)).toBe(14);
    expect(twice(square)(2)).toBe(16);
  });

  it("conditionals produce the branch value", () => {
    expect(sign(-3)).toBe("negative");
    expect(sign(0)).toBe("zero");
    expect(classify(7)).toBe("small");
    expect(classify(42)).toBe("large");
  });

  it("integer division truncates, real division does not", () => {
    expect(halfOf(9)).toBe(4);
    expect(realHalf(9)).toBe(4.5);
  });

  it("integers stay exact up to 2^53 and bigint carries the rest", () => {
    expect(2 ** 53 + 1 === 2 ** 53).toBe(true);
    expect(factorialBig(18n)).toBe(6402373705728000n);
    expect(factorialBig(25n)).toBe(15511210043330985984000000n);
  });
});
